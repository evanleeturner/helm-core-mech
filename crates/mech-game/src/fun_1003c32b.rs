// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb4: u32;
    static mut DAT_10160e90: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_sequence_status_4(param_1: u32) -> u32;
    fn _AIL_stop_sequence_4(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c32b() {
    mech_trace::enter!("1003c32b");
    // SAFETY: the globals and the AIL callees are the game's own; the table
    // at DAT_10160e90 holds eight 4-byte entries, read where the original does.
    unsafe {
        if mech_trace::gr!("1003c32b", "100aefb4", DAT_100aefb4) == 0 {
            let mut local_8: i32 = 0;
            while local_8 < 8 {
                let p = (&raw mut DAT_10160e90)
                    .byte_add(local_8.wrapping_mul(4) as usize)
                    .cast::<i32>();
                if mech_trace::rd!("1003c32b", p, read_unaligned) != -1 {
                    let u_var1 = mech_trace::rd!("1003c32b", p, read_unaligned) as u32;
                    let i_var2 = mech_trace::cr!(
                        "1003c32b",
                        "_AIL_sequence_status_4",
                        _AIL_sequence_status_4(u_var1)
                    ) as i32;
                    if i_var2 == 4 {
                        mech_trace::cr!(
                            "1003c32b",
                            "_AIL_stop_sequence_4",
                            _AIL_stop_sequence_4(u_var1)
                        );
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
        }
    }
}
