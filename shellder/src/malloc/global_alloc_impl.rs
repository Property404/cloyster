use crate::errno::Errno;
use alloc::{collections::BTreeMap, vec::Vec};
use core::ptr::NonNull;

// A free list allocator
pub(crate) struct Allocator {
    map: BTreeMap<NonNull<u8>, Vec<u8>>,
}

unsafe impl Send for Allocator {}

impl Allocator {
    pub(crate) fn new() -> Result<Self, Errno> {
        Ok(Self {
            map: BTreeMap::new(),
        })
    }

    pub(crate) fn allocations(&self) -> usize {
        self.map.len()
    }

    /// Return the size of a memory allocation
    ///
    /// # Safety
    /// Ptr must be a valid, previosly allocated region of memory
    pub(crate) unsafe fn size_of(&mut self, ptr: NonNull<u8>) -> Result<usize, Errno> {
        self.map
            .get(&ptr)
            .map(|v| v.len())
            .ok_or(Errno::CloysterAlloc)
    }

    /// # Safety
    /// Ptr must be a valid, previosly allocated region of memory
    pub(crate) unsafe fn free(&mut self, ptr: NonNull<u8>) -> Result<(), Errno> {
        self.map.remove(&ptr).ok_or(Errno::CloysterAlloc)?;
        Ok(())
    }

    pub(crate) fn alloc(&mut self, requested_size: usize) -> Result<NonNull<u8>, Errno> {
        let vec = Vec::try_with_capacity(requested_size).map_err(|_| Errno::CloysterAlloc)?;
        let ptr = NonNull::new(vec.as_ptr() as *mut u8)
            .expect("This should never ever happen: null vector");
        self.map.insert(ptr, vec);
        Ok(ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const PAGE_SIZE: usize = 0x1000;

    #[test]
    fn rand_allocations() {
        let mut rng = rand::thread_rng();
        let mut allocator = Allocator::new().unwrap();

        for _ in 1..100 {
            let mut allocs = Vec::new();

            for i in 1..100 {
                let area = allocator.alloc(rng.r#gen::<usize>() % 256 + 1).unwrap();
                allocs.push(area);
                assert_eq!(allocator.allocations(), i);
            }

            for alloc in allocs {
                unsafe {
                    allocator.free(alloc).unwrap();
                }
            }
        }

        assert_eq!(allocator.allocations(), 0);
    }

    #[test]
    fn multiple_allocations() {
        let mut allocs = Vec::new();
        let mut allocator = Allocator::new().unwrap();

        for i in 1..400 {
            let area = allocator.alloc(31).unwrap();
            allocs.push(area);
            assert_eq!(allocator.allocations(), i);
        }

        for alloc in allocs {
            unsafe {
                allocator.free(alloc).unwrap();
            }
        }

        assert_eq!(allocator.allocations(), 0);
    }

    #[test]
    fn allocate_more_than_a_page() {
        let mut allocator = Allocator::new().unwrap();
        unsafe {
            let area = allocator.alloc(PAGE_SIZE * 5 + 3).unwrap();
            allocator.free(area).unwrap();
        }
        assert_eq!(allocator.allocations(), 0);
    }

    #[test]
    fn basic() {
        let mut allocator = Allocator::new().unwrap();
        for _ in 0..1000000 {
            let area = allocator.alloc(800).unwrap();
            assert_eq!(allocator.allocations(), 1);
            unsafe {
                {
                    let area = area.as_ptr() as *mut u32;
                    *area = 0xdeadbeef;
                    assert_eq!(*area, 0xdeadbeef);
                }
                allocator.free(area).unwrap();
            }
        }
        assert_eq!(allocator.allocations(), 0);
    }
}
