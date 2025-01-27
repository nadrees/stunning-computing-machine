use core::arch::asm;

pub struct SBIReturn {
    pub error: usize,
    pub value: usize,
}

fn sbi_call(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    fid: usize, // function id
    eid: usize, // extension id
) -> SBIReturn {
    let error: usize;
    let value: usize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid
        );
    }
    SBIReturn { error, value }
}

pub fn putchar(ch: char) {
    sbi_call(ch as usize, 0, 0, 0, 0, 0, 0, 1);
}
