use super::usize_ext::UsizeExt;
use crate::errno::Errno;
use core::{
    ffi::c_void,
    mem,
    ptr::{self, NonNull},
};

const MIN_ALIGN: usize = 32;
const HDR_SIZE: usize = MIN_ALIGN;
const PAGE_SIZE: usize = 4096;

pub(crate) trait MemoryExtender {
    unsafe fn sbrk(&mut self, increment: usize) -> Result<NonNull<c_void>, Errno>;
}

pub(crate) struct DefaultMemoryExtender;

impl MemoryExtender for DefaultMemoryExtender {
    unsafe fn sbrk(&mut self, increment: usize) -> Result<NonNull<c_void>, Errno> {
        NonNull::new(unsafe { crate::unistd::sbrk(increment.try_into()?)? })
            .ok_or(Errno::CloysterAlloc)
    }
}

#[repr(C)]
struct Node {
    free: bool,
    size: usize,
    next_node: Option<NonNull<Node>>,
    prev_node: Option<NonNull<Node>>,
}

// A free list allocator
pub(crate) struct Allocator<T> {
    head: NonNull<Node>,
    size: usize,
    allocations: usize,
    memory_extender: T,
    total_claims: usize,
}

unsafe impl<T: Send> Send for Allocator<T> {}

impl<T: MemoryExtender> Allocator<T> {
    pub(crate) fn new(mut memory_extender: T) -> Result<Self, Errno> {
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
        Ok(Self {
            head,
            size: PAGE_SIZE,
            allocations: 0,
            memory_extender,
            total_claims: 0,
        })
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

    /// # Safety
    /// Ptr must be a valid, previosly allocated region of memory
    pub(crate) unsafe fn free(&mut self, ptr: *mut c_void) -> Result<(), Errno> {
        assert!(!ptr.is_null());

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

    pub(crate) fn alloc(&mut self, requested_size: usize) -> Result<*mut c_void, Errno> {
        assert!(requested_size > 0);
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
            if noderef.free && noderef.size >= requested_size {
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
                return Ok(ptr::from_mut(noderef).wrapping_byte_add(HDR_SIZE) as *mut c_void);
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
            let mut _backing = Vec::with_capacity(capacity);
            let base = _backing.as_mut_ptr() as usize;
            let max = base + capacity;
            Self {
                _backing,
                base,
                max,
            }
        }
    }

    impl MemoryExtender for MockExtender {
        unsafe fn sbrk(&mut self, increment: usize) -> Result<NonNull<c_void>, Errno> {
            let base = self.base;
            self.base += increment;
            if self.base > self.max {
                panic!("Out of mock memory");
            }

            Ok(NonNull::new(base as *mut c_void).unwrap())
        }
    }

    #[test]
    fn rand_allocations() {
        let mut rng = rand::thread_rng();
        let mut allocator = Allocator::new(MockExtender::new(100000)).unwrap();

        for _ in 1..100 {
            let mut allocs = Vec::new();

            for i in 1..100 {
                let area = allocator.alloc(rng.gen::<usize>() % 256 + 1).unwrap() as *mut u32;
                allocs.push(area);
                assert_eq!(allocator.allocations, i);
            }

            for alloc in allocs {
                unsafe {
                    allocator.free(alloc as *mut c_void).unwrap();
                }
            }
        }

        assert_eq!(allocator.allocations, 0);
    }

    #[test]
    fn multiple_allocations() {
        let mut allocs = Vec::new();
        let mut allocator = Allocator::new(MockExtender::new(100000)).unwrap();

        for i in 1..400 {
            let area = allocator.alloc(31).unwrap() as *mut u32;
            allocs.push(area);
            assert_eq!(allocator.allocations, i);
        }

        for alloc in allocs {
            unsafe {
                allocator.free(alloc as *mut c_void).unwrap();
            }
        }

        assert_eq!(allocator.allocations, 0);
        assert!(allocator.total_claims < 10);
    }

    #[test]
    fn allocate_more_than_a_page() {
        let mut allocator = Allocator::new(MockExtender::new(PAGE_SIZE * 10)).unwrap();
        unsafe {
            let area = allocator.alloc(PAGE_SIZE * 5 + 3).unwrap();
            allocator.free(area).unwrap();
        }
        assert_eq!(allocator.allocations, 0);
    }

    #[test]
    fn basic() {
        let mut allocator = Allocator::new(MockExtender::new(10000)).unwrap();
        for _ in 0..1000000 {
            let area = allocator.alloc(800).unwrap() as *mut u32;
            assert_eq!(allocator.allocations, 1);
            unsafe {
                *area = 0xdeadbeef;
                assert_eq!(*area, 0xdeadbeef);
                allocator.free(area as *mut c_void).unwrap();
            }
        }
        assert_eq!(allocator.allocations, 0);
    }
}
