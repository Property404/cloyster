use super::usize_ext::UsizeExt;
use crate::errno::Errno;
use core::{
    alloc::Layout,
    cmp, mem,
    ptr::{self, NonNull},
};

const MIN_ALIGN: usize = 32;
const HDR_SIZE: usize = MIN_ALIGN;
const PAGE_SIZE: usize = 4096;

pub(crate) type Allocator = FreeListAllocator<DefaultMemoryExtender>;

pub(crate) trait MemoryExtender {
    unsafe fn sbrk(&mut self, increment: usize) -> Result<NonNull<u8>, Errno>;
}

pub(crate) struct DefaultMemoryExtender;

impl MemoryExtender for DefaultMemoryExtender {
    unsafe fn sbrk(&mut self, increment: usize) -> Result<NonNull<u8>, Errno> {
        NonNull::new(unsafe { crate::unistd::sbrk(increment.try_into()?)? })
            .ok_or(Errno::CloysterAlloc)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
struct Node {
    free: bool,
    size: usize,
    next_node: Option<NonNull<Node>>,
    prev_node: Option<NonNull<Node>>,
}

// A free list allocator
pub(crate) struct FreeListAllocator<T> {
    head: NonNull<Node>,
    size: usize,
    allocations: usize,
    memory_extender: T,
    total_claims: usize,
}

unsafe impl<T: Send> Send for FreeListAllocator<T> {}

impl FreeListAllocator<DefaultMemoryExtender> {
    pub(crate) fn new() -> Result<Self, Errno> {
        Self::from_memory_extender(DefaultMemoryExtender)
    }
}

impl<T: MemoryExtender> FreeListAllocator<T> {
    fn from_memory_extender(mut memory_extender: T) -> Result<Self, Errno> {
        assert!(HDR_SIZE >= mem::size_of::<Node>());
        let mut head = unsafe { memory_extender.sbrk(PAGE_SIZE)?.cast() };
        unsafe {
            *head.as_mut() = Node {
                free: true,
                size: PAGE_SIZE - MIN_ALIGN,
                next_node: None,
                prev_node: None,
            };
        }
        assert!((head.as_ptr() as usize).is_aligned_to(MIN_ALIGN));
        Ok(Self {
            head,
            size: PAGE_SIZE,
            allocations: 0,
            memory_extender,
            total_claims: 0,
        })
    }

    pub(crate) fn allocations(&self) -> usize {
        self.allocations
    }

    fn claim_more(&mut self, required: usize) -> Result<NonNull<Node>, Errno> {
        let required = required.align_up(PAGE_SIZE);

        let mut node = unsafe { self.memory_extender.sbrk(required)? }.cast();
        assert_eq!(
            node.as_ptr(),
            self.head.as_ptr().wrapping_byte_add(self.size)
        );

        unsafe {
            *node.as_mut() = Node {
                free: true,
                size: required - HDR_SIZE,
                next_node: None,
                prev_node: None,
            }
        };

        self.size += required;
        self.total_claims = self.total_claims.saturating_add(1);

        Ok(node)
    }

    /// Return the size of a memory allocation
    ///
    /// # Safety
    /// Ptr must be a valid, previosly allocated region of memory
    pub(crate) unsafe fn size_of(&mut self, ptr: NonNull<u8>) -> Result<usize, Errno> {
        let ptr = ptr.as_ptr();
        let node = unsafe { ((ptr.wrapping_sub(HDR_SIZE)) as *const Node).as_ref() }
            .ok_or(Errno::CloysterAlloc)?;
        assert!(!node.free);
        Ok(node.size)
    }

    /// # Safety
    /// Ptr must be a valid, previosly allocated region of memory
    pub(crate) unsafe fn free(&mut self, ptr: NonNull<u8>) -> Result<(), Errno> {
        let ptr = ptr.as_ptr();
        let node = unsafe { ((ptr.wrapping_sub(HDR_SIZE)) as *mut Node).as_mut() }
            .ok_or(Errno::CloysterAlloc)?;
        assert!(!node.free);
        node.free = true;

        self.allocations = self
            .allocations
            .checked_sub(1)
            .expect("Freed more than allocated! This is possibly a bug in Cloyster, or you free()'d one too many times");
        Ok(())
    }

    pub(crate) fn alloc_unaligned(&mut self, size: usize) -> Result<NonNull<u8>, Errno> {
        self.alloc(Layout::from_size_align(size, MIN_ALIGN).map_err(|_| Errno::EINVAL)?)
    }

    pub(crate) fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, Errno> {
        let requested_align = cmp::max(layout.align(), MIN_ALIGN);
        let requested_size = layout.size();
        if requested_size == 0 {
            panic!("Program attempted to allocate an object of 0 bytes");
        }
        let requested_size = requested_size.align_up(MIN_ALIGN);
        assert!(requested_size.is_aligned_to(MIN_ALIGN));
        let mut prev_node: Option<NonNull<Node>> = None;
        let mut node = Some(self.head);

        loop {
            let Some(mut noderef) = node else {
                let mut newnode = self.claim_more(requested_size)?;
                unsafe {
                    if let Some(mut prev_node) = prev_node {
                        prev_node.as_mut().next_node = Some(newnode);
                    };
                    newnode.as_mut().prev_node = prev_node;
                    assert!(prev_node.is_some());
                }
                node = Some(newnode);
                continue;
            };
            let noderef = unsafe { noderef.as_mut() };
            let data_ptr = ptr::from_mut(noderef) as usize + HDR_SIZE;
            let align_diff = data_ptr.align_up(requested_align) - data_ptr;
            if noderef.free && noderef.size + align_diff >= requested_size {
                let noderef = if align_diff == 0 {
                    noderef
                } else {
                    let newnoderef = unsafe {
                        ptr::from_mut(noderef)
                            .wrapping_byte_add(align_diff)
                            .as_mut()
                            .expect("Invalid ptr")
                    };
                    assert!((ptr::from_mut(newnoderef) as usize + HDR_SIZE)
                        .is_aligned_to(requested_align));
                    *newnoderef = (*noderef).clone();
                    if let Some(mut prev) = newnoderef.prev_node {
                        let prev = unsafe { prev.as_mut() };
                        prev.size += align_diff;
                        prev.next_node = Some(NonNull::new(ptr::from_mut(newnoderef)).unwrap())
                    }
                    newnoderef
                };
                noderef.free = false;

                if noderef.size >= requested_size + HDR_SIZE + MIN_ALIGN {
                    // Now we split
                    // TODO: Check for wrapping
                    let newnode =
                        ptr::from_mut(noderef).wrapping_byte_add(HDR_SIZE + requested_size);
                    unsafe {
                        (*newnode).free = true;
                        (*newnode).size = noderef.size - requested_size - HDR_SIZE;
                        (*newnode).next_node = noderef.next_node;
                        (*newnode).prev_node = node;
                    }
                    noderef.next_node = NonNull::new(newnode);
                    noderef.size = requested_size;
                }
                self.allocations += 1;
                return NonNull::new(ptr::from_mut(noderef).wrapping_byte_add(HDR_SIZE) as *mut u8)
                    .ok_or(Errno::CloysterAlloc);
            }

            prev_node = node;
            node = noderef.next_node;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    struct MockExtender {
        _backing: Vec<u8>,
        base: usize,
        max: usize,
    }

    impl MockExtender {
        fn new(capacity: usize) -> Self {
            let mut _backing = Vec::with_capacity(capacity + MIN_ALIGN);
            let base = (_backing.as_mut_ptr() as usize).align_up(MIN_ALIGN);
            let max = base + capacity;
            Self {
                _backing,
                base,
                max,
            }
        }
    }

    impl MemoryExtender for MockExtender {
        unsafe fn sbrk(&mut self, increment: usize) -> Result<NonNull<u8>, Errno> {
            let base = self.base;
            self.base += increment;
            if self.base > self.max {
                panic!("Out of mock memory");
            }

            Ok(NonNull::new(base as *mut u8).unwrap())
        }
    }

    #[test]
    fn rand_allocations() {
        let mut rng = rand::thread_rng();
        let mut allocator =
            FreeListAllocator::from_memory_extender(MockExtender::new(100000)).unwrap();

        for _ in 1..100 {
            let mut allocs = Vec::new();

            for i in 1..100 {
                let area = allocator
                    .alloc_unaligned(rng.r#gen::<usize>() % 256 + 1)
                    .unwrap();
                allocs.push(area);
                assert_eq!(allocator.allocations, i);
            }

            for alloc in allocs {
                unsafe {
                    allocator.free(alloc).unwrap();
                }
            }
        }

        assert_eq!(allocator.allocations, 0);
    }

    #[test]
    fn multiple_allocations() {
        let mut allocs = Vec::new();
        let mut allocator =
            FreeListAllocator::from_memory_extender(MockExtender::new(100000)).unwrap();

        for i in 1..400 {
            let area = allocator.alloc_unaligned(31).unwrap();
            allocs.push(area);
            assert_eq!(allocator.allocations, i);
        }

        for alloc in allocs {
            unsafe {
                allocator.free(alloc).unwrap();
            }
        }

        assert_eq!(allocator.allocations, 0);
        assert!(allocator.total_claims < 10);
    }

    #[test]
    fn allocate_more_than_a_page() {
        let mut allocator =
            FreeListAllocator::from_memory_extender(MockExtender::new(PAGE_SIZE * 10)).unwrap();
        unsafe {
            let area = allocator.alloc_unaligned(PAGE_SIZE * 5 + 3).unwrap();
            allocator.free(area).unwrap();
        }
        assert_eq!(allocator.allocations, 0);
    }

    #[test]
    fn basic() {
        let mut allocator =
            FreeListAllocator::from_memory_extender(MockExtender::new(10000)).unwrap();
        for _ in 0..1000000 {
            let area = allocator.alloc_unaligned(800).unwrap();
            assert_eq!(allocator.allocations, 1);
            unsafe {
                {
                    let area = area.as_ptr() as *mut u32;
                    *area = 0xdeadbeef;
                    assert_eq!(*area, 0xdeadbeef);
                }
                allocator.free(area).unwrap();
            }
        }
        assert_eq!(allocator.allocations, 0);
    }

    #[test]
    fn allocate_alignment() {
        let mut allocator =
            FreeListAllocator::from_memory_extender(MockExtender::new(10000)).unwrap();
        for align in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048] {
            let ptr = allocator
                .alloc(Layout::from_size_align(45, align).unwrap())
                .unwrap();
            assert!((ptr.as_ptr() as usize).is_aligned_to(align));
            unsafe {
                allocator.free(ptr).unwrap();
            }
        }
        assert_eq!(allocator.allocations, 0);
    }
}
