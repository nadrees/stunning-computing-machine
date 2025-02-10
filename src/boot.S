# boot.S
# bootloader

# disable compress riscv instructions
.option norvc

.section .text.init
.global _start
_start:
    # first, read the hart (process) id from the mhartid (machine-mode hart id) register
    csrr    t0, mhartid
    # send the process to sleep if it's not process id zero, the only one guaranteed to 
    # exist by the riscv spec
    bnez    t0, wait_for_boot_interrupt
    # we dont need MMU (Memory Managemet Unit) so disable it by setting the supervisor
    # address translation and protections (satp) register to 0
    csrw    satp, zero 

init_global_pointer:
.option push
.option norelax
    la      gp, _global_pointer
.option pop

init_bss:
    la      a0, _bss_start
    la      a1, _bss_end
init_bss_loop:
    bgeu    a0, a1, init_stack_pointer
    sw      zero, (a0)
    addi    a0, a0, 8
    j       init_bss_loop

init_stack_pointer:
    la      sp, _stack_end

init_mstatus:
    # move us into machine mode (bits 11 & 12 both set to 1), and then
    # renable MPIE (machine priveledged interupt enable) and MIE (machine 
    # interup enable) bits
    li      a0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw    mstatus, a0

init_mepc:
    # setup the address to return to once we drop out of machine mode
    la      a0, rs_main # rs_main defined in rust
    # write the address to the machine exception program counter, which
    # tracks where to return to when a trap handler is completed
    csrw    mepc, a0

init_trap_vector:
    la      a0, trap_vector
    # write address of our trap function to the mtvec (machine trap vector)
    # register to handle all interrupts
    csrw    mtvec, a0

enable_interrupts:
    # enable machine level interrups bits
    li      a0, (1 << 11) | (1 << 7) || (1 << 3)
    # store bits in mie (machine interrupt enable) register
    csrw    mie, a0

begin_execution:
    la      ra, end # set up where to go when execution ends
    # "return" to main code, starting program execution
    mret

wait_for_boot_interrupt:
    # this sends any processes to sleep waiting for an interrupt signal
    wfi
    j wait_for_boot_interrupt

trap_vector:
    mret

end: