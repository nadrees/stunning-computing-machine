use core::{alloc::GlobalAlloc, ptr};

use crate::linker::{get_heap_end, get_heap_start};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

/// Initializes the global memory allocator
/// so that we know where to start allocating memory from.
pub fn init() {
    let heap_start_addr = get_heap_start();
    let heap_end_addr = get_heap_end();

    let heap_start = heap_start_addr as *mut Allocation;
    let allocation = Allocation {
        is_free: true,
        size: heap_end_addr - heap_start_addr - size_of::<Allocation>(),
        next: None,
    };
    unsafe { (*heap_start) = allocation }
}

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        assert!(layout.size() > 0, "Layout size must be greater than 0");
        let allocation =
            find_next_free_with_size(get_heap_start() as *mut Allocation, layout.size());
        match allocation {
            Some(allocation) => {
                let allocation = maybe_split_allocation(allocation, layout.size());
                // we dont want to return the memory address of the header itself, or the application will
                // clobber the header. Return the address of the space pointed to by the header instead.
                allocation.byte_add(size_of::<Allocation>()) as *mut u8
            }
            None => ptr::null::<u8>() as *mut u8,
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}

unsafe fn find_next_free_with_size(
    allocation: *mut Allocation,
    size: usize,
) -> Option<*mut Allocation> {
    let mut current = allocation;
    while (*current).is_free == false || (*current).size < size {
        if let Some(next) = (*current).next {
            current = next;
        } else {
            return None;
        }
    }
    Some(current)
}

unsafe fn maybe_split_allocation(allocation: *mut Allocation, size: usize) -> *mut Allocation {
    // we can only split the allocation if the size + size_of<Allocation> is less than the size of
    // the current allocation block. Otherwise, there isn't enough space to both split the space and
    // store another allocation header.
    // If allocation.size == size + size_of<Allocation> then there's no point in splitting - the header
    // would take up the remaining space.
    if (*allocation).size <= (size + size_of::<Allocation>()) {
        return allocation;
    }

    // next address is at current address + size_of<Allocation> (to account for size of current header) + size
    let next_header_addr = allocation.byte_add(size + size_of::<Allocation>());
    let mut next_header = Allocation {
        is_free: true,
        size: 0,
        next: None,
    };

    // to get the next header's remaining size, we start with the current allocation's size, which hasn't yet
    // been modified. The difference in the current header's address and the next header's address is the space
    // we're reserving for that will be the current header's size after all modifications are done. We also need
    // to remove the next header's size.
    next_header.size = (*allocation).size
        - next_header_addr.byte_offset_from_unsigned(allocation)
        - size_of::<Allocation>();
    // if the current allocation has a next, move it to the next_header, or we'll lose this reference when we
    // update the current allocation header.
    next_header.next = (*allocation).next;
    // write out the header
    (*next_header_addr) = next_header;

    // finally configure the current allocation header to remove excess size, and set next appropriately
    (*allocation).next = Some(next_header_addr);
    (*allocation).size = size;

    // allocation is now fully updated, return it
    allocation
}

#[repr(C)]
struct Allocation {
    is_free: bool,
    size: usize,
    next: Option<*mut Allocation>,
}
