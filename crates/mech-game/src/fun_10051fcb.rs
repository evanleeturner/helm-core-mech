// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09e8: u32;
    static mut DAT_101153c0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051fcb(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table index is
    // kept in range by the original's modulo, and the division is done by
    // the machine's idiv as in the original.
    unsafe {
        let i_var1 = (DAT_100b09e8.wrapping_mul(4)) as i32;
        DAT_100b09e8 = DAT_100b09e8.wrapping_add(1);
        DAT_100b09e8 = DAT_100b09e8 % 0x7f;
        let val = (&raw mut DAT_101153c0)
            .cast::<u8>()
            .offset(i_var1 as isize)
            .cast::<i32>()
            .read_unaligned();
        let rem: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) param_1,
            inout("eax") val => _,
            out("edx") rem,
        );
        rem
    }
}
