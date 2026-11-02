// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09e0: u32;
    static mut DAT_101153c0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051f43(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the table index is read
    // as the original does, and the division uses the machine's idiv.
    unsafe {
        let i_var1 = (DAT_100b09e0.wrapping_mul(4)) as i32;
        DAT_100b09e0 = DAT_100b09e0.wrapping_add(1);
        if DAT_100b09e0 == 0x7f {
            DAT_100b09e0 = 0;
        }
        let v = (&raw mut DAT_101153c0)
            .cast::<u8>()
            .byte_offset(i_var1 as isize)
            .cast::<i32>()
            .read_unaligned();
        let rem: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) param_1,
            inout("eax") v => _,
            out("edx") rem,
        );
        rem
    }
}
