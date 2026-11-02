// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5058: u8;
    static mut DAT_100a5080: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007f49(param_1: i32, param_2: i32) -> u8;
    fn FUN_10007440(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10008156(param_1: i32, param_2: i32) {
    if param_2 > -1 && param_1 != -1 && param_1 < 0xb {
        // SAFETY: the globals and callees are the game's own; only addresses
        // of the globals are formed, and the callees receive them as the
        // original does.
        unsafe {
            let local_8: *mut i32 = if param_2 == -1 {
                core::ptr::null_mut()
            } else {
                (&raw mut DAT_100a5058)
                    .wrapping_byte_offset(param_2.wrapping_mul(0xc) as isize)
                    .cast::<i32>()
            };
            match param_1 {
                1 => {
                    FUN_10007f49(param_2, 2);
                }
                2 => {
                    FUN_10007f49(param_2, 4);
                }
                3 => {
                    FUN_10007f49(param_2, 3);
                }
                4 => {
                    FUN_10007f49(param_2, 5);
                }
                5 => {
                    FUN_10007f49(param_2, 6);
                }
                6 => {
                    FUN_10007f49(param_2, 7);
                }
                9 => {
                    FUN_10007f49(param_2, 1);
                }
                10 => {}
                _ => {
                    FUN_10007440(
                        local_8,
                        (&raw mut DAT_100a5080)
                            .wrapping_byte_offset(param_1.wrapping_mul(0xc) as isize)
                            .cast::<i32>(),
                        -1,
                    );
                }
            }
        }
    }
}
