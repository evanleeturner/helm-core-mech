// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028707(_param_1: u32, param_2: u16) -> u16 {
    mech_trace::enter!("10028707", _param_1, param_2);
    mech_trace::rt!("10028707", param_2)
}
