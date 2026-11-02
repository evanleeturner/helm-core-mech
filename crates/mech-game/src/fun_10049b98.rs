// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_101ec774: u8;
    static mut DAT_101ec778: u8;
    static mut DAT_101ec780: u8;
    static mut DAT_101ec78c: u8;
    static mut DAT_101ec790: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10049c45(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10049b98() {
    mech_trace::enter!("10049b98");
    let mut local_8: i32 = 0;
    while local_8 < 0xff {
        let i_var1: i32 = local_8.wrapping_mul(0x60);
        // SAFETY: the globals are the game's own tables of 0x60-byte entries;
        // reads and writes go where the original does, in the same order.
        unsafe {
            let p78c = (&raw mut DAT_101ec78c)
                .byte_offset(i_var1 as isize)
                .cast::<i32>();
            let p780 = (&raw mut DAT_101ec780)
                .byte_offset(i_var1 as isize)
                .cast::<i32>();
            if mech_trace::rd!("10049b98", p78c, read_unaligned) != 0
                && mech_trace::rd!("10049b98", p780, read_unaligned) != 0
            {
                let p774 = (&raw mut DAT_101ec774)
                    .byte_offset(i_var1 as isize)
                    .cast::<i32>();
                let p778 = (&raw mut DAT_101ec778)
                    .byte_offset(i_var1 as isize)
                    .cast::<i32>();
                let p790 = (&raw mut DAT_101ec790)
                    .byte_offset(i_var1 as isize)
                    .cast::<i32>();
                mech_trace::wr!(
                    "10049b98",
                    p774,
                    write_unaligned,
                    mech_trace::rd!("10049b98", p774, read_unaligned)
                        .wrapping_add(mech_trace::gr!("10049b98", "100b3904", DAT_100b3904) as i32)
                );
                mech_trace::wr!(
                    "10049b98",
                    p778,
                    write_unaligned,
                    mech_trace::rd!("10049b98", p778, read_unaligned)
                        .wrapping_sub(mech_trace::gr!("10049b98", "100b3904", DAT_100b3904) as i32)
                );
                if mech_trace::rd!("10049b98", p790, read_unaligned) == 0 {
                    mech_trace::cr!("10049b98", "10049c45", FUN_10049c45(local_8));
                }
            }
        }
        local_8 = local_8.wrapping_add(1);
    }
}
