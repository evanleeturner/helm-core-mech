// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002949b(
    ch_Likely: i16,
    param_2: i16,
    param_3: i16,
    param_4: i16,
) -> i16 {
    mech_trace::enter!("1002949b", ch_Likely, param_2, param_3, param_4);
    let mut result = ch_Likely;
    if ch_Likely == 0x40 {
        result = param_2;
    } else if ch_Likely == 0x25 {
        result = param_3;
    } else if ch_Likely == 0x2a {
        result = param_4;
    }
    mech_trace::rt!("1002949b", result)
}
