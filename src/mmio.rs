//! MMIO (Memory Mapped IO) allos reading and writing to peripheral devices
//! as thought they are addresses in memory. This simplifies the OS's job
//! of communicating because everything can be treated as RAM.

use num_traits::Num;

/// Performs a write to the address provided
/// The offset uses pointer arithmetic to calculate the final
/// address to write to, meaning that the final address =
/// address + sizeof(usize) * offset
#[inline]
pub fn write<T>(address: usize, offset: usize, value: T)
where
    T: Num + Sized,
{
    let address = address as *mut T;
    unsafe { address.add(offset).write_volatile(value) }
}

/// Performs a read from the address provided
/// The offset uses pointer arithmetic to calculate the final
/// address to write to, meaning that the final address =
/// address + sizeof(usize) * offset
#[inline]
pub fn read<T>(address: usize, offset: usize) -> T
where
    T: Num + Sized,
{
    let address = address as *mut T;
    unsafe { address.add(offset).read_volatile() }
}
