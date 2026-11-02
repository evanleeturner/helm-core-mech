// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100cded4: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c9fd(param_1: i32) -> i32 {
    mech_trace::enter!("1007c9fd", param_1);
    let mut local_8: i32 = -1;
    let mut local_c: i32 = 0;
    while local_c < 0x80 {
        // SAFETY: DAT_100cded4 is the game's own table of 0x80 entries of
        // 0x24 bytes; the first field of each entry is read.
        let v = unsafe {
            mech_trace::rd!(
                "1007c9fd",
                (&raw mut DAT_100cded4)
                    .cast::<u8>()
                    .byte_add(local_c.wrapping_mul(0x24) as u32 as usize)
                    .cast::<i32>(),
                read_unaligned
            )
        };
        if v == param_1 {
            local_8 = local_c;
            break;
        }
        local_c = local_c.wrapping_add(1);
    }
    mech_trace::rt!("1007c9fd", local_8)
}
