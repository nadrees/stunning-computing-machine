/// Helper struct to track the various elements that consume memory for each allocation
///
/// Each property tracks how many bytes that part of the Allocation uses in memory.
pub struct AllocationSizeParts {
    /// how many bytes the "header" (the actual Allocation struct) takes
    pub header: usize,
    /// each allocation stores a pointer to the header 1 word before the
    /// data section, so that we can find the header again when deallocating
    pub header_ptr: usize,
    /// how many bytes were reserved for data
    pub data: usize,
}

impl AllocationSizeParts {
    /// Returns the total number of bytes needed to skip past this allocation.
    /// This is the same as the address for the next allocation
    pub fn get_total_size(&self) -> usize {
        self.data + self.header + self.header_ptr
    }

    pub fn get_header_and_ptr_size(&self) -> usize {
        self.header + self.header_ptr
    }
}

#[cfg(test)]
mod tets {
    use super::*;

    #[test]
    fn test_get_total_size() {
        let asp = AllocationSizeParts {
            header: 5,
            header_ptr: 1,
            data: 4,
        };
        assert_eq!(10, asp.get_total_size());
    }

    #[test]
    fn test_get_header_and_ptr_size() {
        let asp = AllocationSizeParts {
            header: 5,
            header_ptr: 1,
            data: 4,
        };
        assert_eq!(6, asp.get_header_and_ptr_size());
    }
}
