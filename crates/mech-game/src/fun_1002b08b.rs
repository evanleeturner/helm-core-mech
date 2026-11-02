// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b08b(param_1: i32, param_2: i32, match_sign: i32) -> i32 {
    mech_trace::enter!("1002b08b", param_1, param_2, match_sign);
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    if match_sign != 0 {
        let s2: i32 = if param_2 < 0 { -1 } else { 1 };
        let s1: i32 = if param_1 < 0 { -1 } else { 1 };
        if s2 != s1 {
            param_2 = param_2.wrapping_neg();
        }
    }
    param_1 = param_1.wrapping_add(param_2);
    if param_1 < 0x401 {
        if param_1 < -0x400 {
            param_1 = -0x400;
        }
    } else {
        param_1 = 0x400;
    }
    mech_trace::rt!("1002b08b", param_1)
}
