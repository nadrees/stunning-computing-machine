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
                (*allocation).maybe_split(layout);
                (*allocation).mark(false);
                // we dont want to return the memory address of the header itself, or the application will
                // clobber the header. Return the address of the space pointed to by the header instead.
                let header_ptr = allocation.byte_add(
                    // move past the header
                    size_of::<Allocation>()
                    // move past the offset needed to layout purposes
                    + (*allocation).offset_for_layout(layout),
                ) as *mut *mut Allocation;
                core::ptr::write(header_ptr, allocation);

                // now move forward past the pointer to the data address
                header_ptr.byte_add(size_of::<usize>()) as *mut u8
            }
            None => ptr::null::<u8>() as *mut u8,
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let header_ptr = ptr.byte_sub(size_of::<usize>()) as *const *mut Allocation;
        let header = *header_ptr;
        (*header).mark(true);
        (*header).maybe_merge();
    }
}

unsafe fn find_next_free_with_size(
    allocation: *mut Allocation,
    size: usize,
) -> Option<*mut Allocation> {
    let mut current = allocation;
    while (*current).is_free == false || (*current).size < size {
        if let Some(next) = (*current).get_next_allocation_address() {
            current = next as *mut Allocation;
        } else {
            return None;
        }
    }
    Some(current)
}

#[repr(C)]
struct Allocation {
    is_free: bool,
    size: usize,
}

impl Allocation {
    fn get_size(&self) -> AllocationSizeParts {
        AllocationSizeParts {
            header: size_of_val(self),
            header_ptr: size_of::<usize>(),
            data: self.size,
        }
    }

    /// calculates the memory address of the next Allocation header after this one
    fn get_next_allocation_address(&self) -> Option<*const Allocation> {
        let next_addr = unsafe { (self as *const Self).byte_add(self.get_size().get_total_size()) };
        if next_addr.addr() >= get_heap_end() {
            return None;
        }
        Some(next_addr)
    }

    /// Tries to split the allocation based on the requested layout. After this operation the allocation
    /// will have a little space as possible to fit the requested layout, and a new allocation will be
    /// created for the remaining space if there is sufficient left over.
    fn maybe_split(&mut self, layout: core::alloc::Layout) {
        let self_size = self.get_size();

        let size = layout.size();
        let offset = self.offset_for_layout(layout);

        // we can only split the allocation if the size + size_of<Allocation> is less than the size of
        // the current allocation block. Otherwise, there isn't enough space to both split the space and
        // store another allocation header. Finally, we reserve 1 word just before the data section to
        // store the address of the header so that we can look it up again when deallocating.
        // If allocation.size == size + size_of<Allocation> then there's no point in splitting - the header
        // would take up the remaining space.
        if self.size <= (size + offset + self_size.get_header_and_ptr_size()) {
            return;
        }

        // next address is at current address + size_of<Allocation> (to account for size of current header) + size
        let next_header_addr =
            unsafe { (self as *mut Allocation).byte_add(size + self_size.header) };
        let mut next_header = Allocation {
            is_free: true,
            size: 0,
        };

        // to get the next header's remaining size, we start with the current allocation's size, which hasn't yet
        // been modified. The difference in the current header's address and the next header's address is the space
        // we're reserving for that will be the current header's size after all modifications are done. We also need
        // to remove the next header's size. Finally, we need to hold back whatever additional offset was needed for
        // alignment.
        next_header.size = self.size
            - unsafe { next_header_addr.byte_offset_from_unsigned(self as *mut Allocation) }
            - self_size.get_header_and_ptr_size()
            - offset;
        // write out the header
        unsafe {
            core::ptr::write(next_header_addr, next_header);
        }

        // finally configure the current allocation header to remove excess size
        self.size = size;
    }

    /// Looks to see if this segment can be merged with the next segment. If so, updates this segment's header to
    /// contain the entirety of the next segment as well.
    ///
    /// NOTE: this does *not* zero out the memory. Call zalloc to zero out memory when allocating it.
    unsafe fn maybe_merge(&mut self) {
        while let Some(next_addr) = self.get_next_allocation_address() {
            if (*next_addr).is_free == false {
                return;
            }

            // update the current header's size to include the next header's size, the 1 word offset, and the size
            // of the next segment. This effectively just merges all of that memory space back into this segment's
            // available space for reallocation in the future.
            self.size += (*next_addr).get_size().get_total_size();
        }
    }

    /// Marks the allocation as free or not
    fn mark(&mut self, is_free: bool) {
        self.is_free = is_free;
    }

    fn offset_for_layout(&self, layout: core::alloc::Layout) -> usize {
        // move past the current header's space
        let data_addr = unsafe { (self as *const Self).byte_add(size_of_val(self)) };
        // the offset needed is the how far past that alignment we are at the start of the data section
        layout.align() - (data_addr as usize % layout.align())
    }
}

/// Helper struct to track the various elements that consume memory for each allocation
///
/// Each property tracks how many bytes that part of the Allocation uses in memory.
struct AllocationSizeParts {
    /// how many bytes the "header" (the actual Allocation struct) takes
    header: usize,
    /// each allocation stores a pointer to the header 1 word before the
    /// data section, so that we can find the header again when deallocating
    header_ptr: usize,
    /// how many bytes were reserved for data
    data: usize,
}

impl AllocationSizeParts {
    /// Returns the total number of bytes needed to skip past this allocation.
    /// This is the same as the address for the next allocation
    fn get_total_size(&self) -> usize {
        self.data + self.header + self.header_ptr
    }

    fn get_header_and_ptr_size(&self) -> usize {
        self.header + self.header_ptr
    }
}
