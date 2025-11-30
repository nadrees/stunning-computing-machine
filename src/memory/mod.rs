use core::{alloc::GlobalAlloc, ptr};

use crate::{memory::allocation_size_parts::AllocationSizeParts, Globals};

mod allocation_size_parts;

pub struct Allocator<TGlobals>
where
    TGlobals: Globals + 'static,
{
    pub globals: &'static TGlobals,
}

impl<TGlobals> Allocator<TGlobals>
where
    TGlobals: Globals,
{
    /// Initializes the global memory allocator
    /// so that we know where to start allocating memory from.
    pub fn init(&self) {
        let heap_start_addr = self.globals.get_heap_start();
        let heap_end_addr = self.globals.get_heap_end();

        let heap_start = heap_start_addr as *mut Allocation<TGlobals>;
        let allocation = Allocation {
            is_free: true,
            size: heap_end_addr - heap_start_addr - size_of::<Allocation<TGlobals>>(),
            globals: self.globals,
        };
        unsafe { (*heap_start) = allocation }
    }
}

unsafe impl<TGlobals> GlobalAlloc for Allocator<TGlobals>
where
    TGlobals: Globals,
{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        assert!(layout.size() > 0, "Layout size must be greater than 0");
        let allocation = find_next_free_with_size(
            self.globals.get_heap_start() as *mut Allocation<TGlobals>,
            layout.size(),
        );
        match allocation {
            Some(allocation) => {
                (*allocation).maybe_split(layout);
                (*allocation).mark(false);
                // we dont want to return the memory address of the header itself, or the application will
                // clobber the header. Return the address of the space pointed to by the header instead.
                let header_ptr = allocation.byte_add(
                    // move past the header
                    size_of::<Allocation<TGlobals>>()
                    // move past the offset needed to layout purposes
                    + (*allocation).offset_for_layout(layout),
                ) as *mut *mut Allocation<TGlobals>;
                core::ptr::write(header_ptr, allocation);

                // now move forward past the pointer to the data address
                header_ptr.byte_add(size_of::<usize>()) as *mut u8
            }
            None => ptr::null::<u8>() as *mut u8,
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let header_ptr = ptr.byte_sub(size_of::<usize>()) as *const *mut Allocation<TGlobals>;
        let header = *header_ptr;
        (*header).mark(true);
        (*header).maybe_merge();
    }
}

unsafe fn find_next_free_with_size<TGlobals>(
    allocation: *mut Allocation<TGlobals>,
    size: usize,
) -> Option<*mut Allocation<TGlobals>>
where
    TGlobals: Globals,
{
    let mut current = allocation;
    while (*current).is_free == false || (*current).size < size {
        if let Some(next) = (*current).get_next_allocation_address() {
            current = next as *mut Allocation<TGlobals>;
        } else {
            return None;
        }
    }
    Some(current)
}

#[repr(C)]
struct Allocation<TGlobals>
where
    TGlobals: Globals + 'static,
{
    is_free: bool,
    size: usize,
    globals: &'static TGlobals,
}

impl<TGlobals> Allocation<TGlobals>
where
    TGlobals: Globals,
{
    fn get_size(&self) -> AllocationSizeParts {
        AllocationSizeParts {
            header: size_of_val(self),
            header_ptr: size_of::<usize>(),
            data: self.size,
        }
    }

    /// calculates the memory address of the next Allocation header after this one
    fn get_next_allocation_address(&self) -> Option<*const Allocation<TGlobals>> {
        let next_addr = unsafe { (self as *const Self).byte_add(self.get_size().get_total_size()) };
        if next_addr.addr() >= self.globals.get_heap_end() {
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
            unsafe { (self as *mut Allocation<TGlobals>).byte_add(size + self_size.header) };
        let mut next_header = Allocation {
            is_free: true,
            size: 0,
            globals: self.globals,
        };

        // to get the next header's remaining size, we start with the current allocation's size, which hasn't yet
        // been modified. The difference in the current header's address and the next header's address is the space
        // we're reserving for that will be the current header's size after all modifications are done. We also need
        // to remove the next header's size. Finally, we need to hold back whatever additional offset was needed for
        // alignment.
        next_header.size = self.size
            - unsafe {
                next_header_addr.byte_offset_from_unsigned(self as *mut Allocation<TGlobals>)
            }
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
