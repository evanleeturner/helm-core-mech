// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut s_RMG_could_not_create_file_Ok_10094380: i8;
    static mut s_RMG_was_passed_a_bad_parameter_O_100943a0: i8;
    static mut s_RMG_could_not_open_a_file_Ok_100943c4: i8;
    static mut s_RMG____out_of_memory_Ok_100943e4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005b1b0(param_1: *mut i8, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037d80(param_1: u32) {
    mech_trace::enter!("10037d80", param_1);
    let mut local_100: [i8; 256] = [0; 256];
    let buf = local_100.as_mut_ptr();
    // SAFETY: the callees and the format strings are the game's own; buf
    // points at a 256-byte local buffer that outlives both calls.
    unsafe {
        match param_1 {
            1 => {
                mech_trace::cr!(
                    "10037d80",
                    "_sprintf",
                    _sprintf(buf, &raw const s_RMG_could_not_create_file_Ok_10094380)
                );
                FUN_1005b1b0(buf, 0);
            }
            2 => {
                mech_trace::cr!(
                    "10037d80",
                    "_sprintf",
                    _sprintf(buf, &raw const s_RMG_was_passed_a_bad_parameter_O_100943a0)
                );
                FUN_1005b1b0(buf, 0);
            }
            3 => {
                mech_trace::cr!(
                    "10037d80",
                    "_sprintf",
                    _sprintf(buf, &raw const s_RMG_could_not_open_a_file_Ok_100943c4)
                );
                FUN_1005b1b0(buf, 0);
            }
            4 => {
                mech_trace::cr!(
                    "10037d80",
                    "_sprintf",
                    _sprintf(buf, &raw const s_RMG____out_of_memory_Ok_100943e4)
                );
                FUN_1005b1b0(buf, 0);
            }
            _ => {}
        }
    }
}
