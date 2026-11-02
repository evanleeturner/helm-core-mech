// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb4: u32;
    static mut DAT_10160e90: [u8; 32];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003c6bf() -> u32;
    fn _AIL_sequence_status_4(param_1: u32) -> u32;
    fn _AIL_set_sequence_volume_12(param_1: u32, param_2: u32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c756(param_1: i16) {
    mech_trace::enter!("1003c756", param_1);
    // SAFETY: the globals and callees are the game's own; the table holds
    // eight 4-byte sequence handles read at the original's offsets.
    unsafe {
        if mech_trace::gr!("1003c756", "100aefb4", DAT_100aefb4) == 0 {
            let u_var1 = mech_trace::cr!("1003c756", "1003c6bf", FUN_1003c6bf());
            if (u_var1 as i16) != 0 {
                let mut local_8: i32 = 0;
                while local_8 < 8 {
                    let p = (&raw mut DAT_10160e90)
                        .cast::<u8>()
                        .byte_add(local_8.wrapping_mul(4) as usize)
                        .cast::<i32>();
                    if mech_trace::rd!("1003c756", p, read_unaligned) != -1 {
                        let i_var2 = mech_trace::cr!(
                            "1003c756",
                            "_AIL_sequence_status_4",
                            _AIL_sequence_status_4(
                                mech_trace::rd!("1003c756", p, read_unaligned) as u32
                            )
                        );
                        if i_var2 as i32 == 4 {
                            mech_trace::cr!(
                                "1003c756",
                                "_AIL_set_sequence_volume_12",
                                _AIL_set_sequence_volume_12(
                                    mech_trace::rd!("1003c756", p, read_unaligned) as u32,
                                    (param_1 as i32) as u32,
                                    1000,
                                )
                            );
                        }
                    }
                    local_8 = local_8.wrapping_add(1);
                }
            }
        }
    }
}
