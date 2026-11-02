// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100f3170: u8;
    static mut DAT_10113170: u8;
    static mut DAT_10113172: u8;
    static mut DAT_10113178: u8;
    static mut DAT_1011317a: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f81d(param_1: i16, mut param_2: u16) {
    if 0x1f < param_2 {
        param_2 = 0;
    }
    let i_var1: i32 = (param_1 as i32).wrapping_mul(0xe);
    // SAFETY: the globals are the game's own tables; offsets are applied
    // to their addresses exactly as the original does.
    unsafe {
        let a = (&raw mut DAT_10113178)
            .byte_offset(i_var1 as isize)
            .cast::<i16>()
            .read_unaligned();
        if -1 < a {
            let idx = (&raw mut DAT_10113170)
                .byte_offset(i_var1 as isize)
                .cast::<i16>()
                .read_unaligned();
            let off = (param_2 as u32)
                .wrapping_mul(8)
                .wrapping_add((idx as i32).wrapping_mul(0x100) as u32);
            let v = (&raw mut DAT_100f3170)
                .byte_offset(off as i32 as isize)
                .cast::<i16>()
                .read_unaligned();
            if 0 < v {
                (&raw mut DAT_10113172)
                    .byte_offset(i_var1 as isize)
                    .cast::<u16>()
                    .write_unaligned(param_2);
                (&raw mut DAT_1011317a)
                    .byte_offset(i_var1 as isize)
                    .cast::<u32>()
                    .write_unaligned(0);
            }
        }
    }
}
