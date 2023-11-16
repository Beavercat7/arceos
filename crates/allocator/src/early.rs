//! Buddy memory allocation.
//!
//! TODO: more efficient


use core::alloc::Layout;
use core::ptr::NonNull;
use bitmap_allocator::BitAlloc;
const MIN_HEAP_SIZE: usize = 0x8000;

use buddy_system_allocator::Heap;
// use slab_allocator::Heap;
use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator,PageAllocator};

type BitAllocUsed = bitmap_allocator::BitAlloc1M;
/// A byte-granularity memory allocator based on the [buddy_system_allocator].
///
/// [buddy_system_allocator]: https://docs.rs/buddy_system_allocator/latest/buddy_system_allocator/
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    inner1: Heap<32>, 
    inner2: BitAllocUsed,
    base: usize,
    total_pages: usize,
    used_pages: usize,
    total_bytes: usize,
    used_bytes: usize,
    byte_counts: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Creates a new empty `BitmapPageAllocator`.
    pub const fn new() -> Self {
        Self {
            inner1: Heap::<32>::new(), 
            inner2: BitAllocUsed::DEFAULT, 
            base: MIN_HEAP_SIZE,
            total_pages: 0,
            used_pages: 0,
            total_bytes: 0,
            used_bytes: 0,
            byte_counts: 0,
        }
    } 
}

// impl<const PAGE_SIZE: usize> BaseAllocator for BitmapPageAllocator<PAGE_SIZE> {
//     fn init(&mut self, start: usize, size: usize) {
       
//     }

//     fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
//         Err(AllocError::NoMemory) // unsupported
//     }
// }
impl<const PAGE_SIZE: usize> BaseAllocator for  EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        unsafe { self.inner1.init(start, size) };  
        assert!(PAGE_SIZE.is_power_of_two());
        let end = super::align_down(start + size, PAGE_SIZE);
        let start = super::align_up(start, PAGE_SIZE);
        self.base = start;
        self.total_pages = (end - start) / PAGE_SIZE;
        self.inner2.insert(0..self.total_pages);
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe { self.inner1.add_to_heap(start, start + size) };
        Ok(())
    }
}

impl <const PAGE_SIZE: usize>ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> { 
        self.byte_counts+=1;
        self.inner1.alloc(layout).map_err(|_| AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.byte_counts-=1;
        self.inner1.dealloc(pos, layout)
    }

    fn total_bytes(&self) -> usize {
        self.inner1.stats_total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.inner1.stats_alloc_actual()
    }

    fn available_bytes(&self) -> usize {
        self.inner1.stats_total_bytes() - self.inner1.stats_alloc_actual()
    }
}


impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
         let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let align_log2 = align_pow2.trailing_zeros() as usize;

        match num_pages.cmp(&1) {
            core::cmp::Ordering::Equal => self.inner2.alloc().map(|idx| idx * PAGE_SIZE + self.base),
            core::cmp::Ordering::Greater => self
                .inner2
                .alloc_contiguous(num_pages, align_pow2)
                .map(|idx| idx * PAGE_SIZE + self.base),
            _ => return Err(AllocError::InvalidParam),
        }
        .ok_or(AllocError::NoMemory)
        .inspect(|_| self.used_pages += num_pages)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        // TODO: not decrease `used_pages` if deallocation failed
        self.used_pages -= num_pages;
        self.inner2.dealloc((pos - self.base) / PAGE_SIZE)
    }

    fn total_pages(&self) -> usize {
        self.total_pages
    }

    fn used_pages(&self) -> usize {
        self.used_pages
    }

    fn available_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
}
