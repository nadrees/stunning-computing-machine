/* output architectur = riscv */
OUTPUT_ARCH("riscv")

/* entry point is named boot */
ENTRY(_start)

/* 
memory map - in this case, ram starts at 0x8000_0000, which matches
the memory map provided by QEMU and most riscv boards

we're using 128 MB of ram for now

See https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c for our board's 
memory map

memory modifiers:
* w (writable)
* x (executable)
* a (allocatable)
* r (read only)
* i (initialized)
*/
MEMORY {
    uart (w) : ORIGIN = 0x10000000, LENGTH = 256
    ram (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

/* 
define program headers
* text: CPU instructions
* rodata: Constants
* data: Global, initialized variables
* bss: Global, uninitialized variables

in our case, we're loading all these headers from the file (PT_LOAD command)
see https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_node/ld_23.html
*/ 
PHDRS {
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS {
    .text : {
        /* remember the location of the start of the text section */
        PROVIDE(_text_start = .);
        /* place .text.init first, declared from any object file */
        *(.text.init) 
        /* next, place all .text and .text.* sections from any object files */ 
        *(.text .text.*)
        /* remember the end of the text section */
        PROVIDE(_text_end = .);
        /* 
        put this section in the ram portion of memory, for both virtual memory (>ram) 
        and load memory (AT>ram)
        put this section into the text program header (:text)
        */
    } >ram AT>ram :text

    /* 
    Save the address just after .text section. We'll use this later to  reference any 
    global variables addresses 
    */
    PROVIDE(_global_pointer = .);

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } >ram AT>ram :rodata

    .data : {
        PROVIDE(_data_start = .);
        *(.sdata .sdata.* .data .data.*)
        PROVIDE(_data_end = .);
    } >ram AT>ram :data

    .bss : {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.* .bss .bss.*)
        PROVIDE(_bss_end = .);
    } >ram AT>ram :bss

    /* 
    Remember the starting address of RAM, which we can use later to ensure
    we don't over-allocate memory and start overwriting program data
    */
    PROVIDE(_ram_start = ORIGIN(ram));
    PROVIDE(_ram_end = _ram_start + LENGTH(ram));

    /*
    Before remembering the stack and heap locations, align ourselves with
    the nearest word boundary
    */
    . = ALIGN(8);

    /*
    In risc, the stack grows "down", so we need to remember the start and
    stop locations for the stack. As we use it, we'll subtract the number 
    of bytes we need, and as the stack is freed we add them back.
    */
    PROVIDE(_stack_start = .);
    /* We have a stack size of 128KB */
    PROVIDE(_stack_end = _stack_start + 128K);

    /*
    Re-align us to the nearest word boundary before allocating the heap address space
    */
    . = ALIGN(8);

    PROVIDE(_heap_start = _stack_end);
    PROVIDE(_heap_end = _ram_end);

    /*
    Expose remaining needed memory pointers
    */
    PROVIDE(_uart_address = ORIGIN(uart));
}