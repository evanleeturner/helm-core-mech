// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64bc: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000dd98(param_1: i32) -> u32 {
    mech_trace::enter!("1000dd98", param_1);
    let mut local_8: u32 = 0;
    // SAFETY: the globals are the game's own memory; the index is bounds-checked
    // against DAT_100a7584 and the table entries are 0x7c bytes apart.
    unsafe {
        if param_1 < mech_trace::gr!("1000dd98", "100a7584", DAT_100a7584) && -1 < param_1 {
            local_8 = mech_trace::rd!(
                "1000dd98",
                (&raw mut DAT_101f64bc)
                    .cast::<u8>()
                    .byte_add((param_1 as u32).wrapping_mul(0x7c) as usize)
                    .cast::<u32>(),
                read_unaligned
            );
        }
    }
    mech_trace::rt!("1000dd98", local_8)
}
