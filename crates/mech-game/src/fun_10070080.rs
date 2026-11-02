// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10070080() -> u32 {
    mech_trace::enter!("10070080");
    mech_trace::rt!("10070080", 1)
}
