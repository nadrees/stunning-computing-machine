// these are provided by the linker script
unsafe extern "C" {
    unsafe static _uart_address: usize;
    unsafe static _heap_start: usize;
    unsafe static _heap_end: usize;
}

macro_rules! make_get_linker_variable {
    ($name:ident) => {
        pastey::paste! {
            pub fn [<get $name>]() -> usize {
                &raw const $name as usize
            }
        }
    };
}

make_get_linker_variable!(_uart_address);
make_get_linker_variable!(_heap_start);
make_get_linker_variable!(_heap_end);
