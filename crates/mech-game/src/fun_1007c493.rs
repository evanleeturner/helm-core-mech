// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100cf0dc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c3c2(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c493(param_1: i32, param_2: i32) {
    // SAFETY: DAT_100cf0dc is the game's own table of 0x14-byte entries; the
    // first field of entry param_1 is read and written where the original does,
    // and the callee is the game's own.
    unsafe {
        let p = (&raw mut DAT_100cf0dc)
            .byte_offset(param_1.wrapping_mul(0x14) as isize)
            .cast::<i32>();
        p.write_unaligned(p.read_unaligned().wrapping_sub(param_2));
        if p.read_unaligned() < 0 {
            FUN_1007c3c2(param_1);
        }
    }
}
