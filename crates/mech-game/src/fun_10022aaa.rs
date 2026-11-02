// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10022aaa() -> u32 {
    mech_trace::enter!("10022aaa");
    mech_trace::rt!("10022aaa", 0xc81)
}
