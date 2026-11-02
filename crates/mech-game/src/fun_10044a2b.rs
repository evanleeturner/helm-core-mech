// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044a2b(param_1: i32) -> i32 {
    // SAFETY: the global is the game's own; param_1 is an address of the
    // caller's structure, read and written at the original's offsets.
    unsafe {
        let divisor = DAT_100b3904;
        if divisor == 0 {
            return 0;
        }
        let a = (param_1.wrapping_add(0x1ee) as *const i32).read_unaligned();
        let b = (param_1.wrapping_add(0xc0) as *const i32).read_unaligned();
        let num = a.wrapping_sub(b);
        let q: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d}",
            d = in(reg) divisor,
            inout("eax") num => q,
            out("edx") _,
            options(nomem, nostack)
        );
        let v = (param_1.wrapping_add(0xc0) as *const i32).read_unaligned();
        (param_1.wrapping_add(0x1ee) as *mut i32).write_unaligned(v);
        q
    }
}
