// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c96bc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d245(param_1: u32) {
    mech_trace::enter!("1007d245", param_1);
    // SAFETY: the global is the game's own memory; written once as the original does.
    unsafe {
        mech_trace::gw!("1007d245", "100c96bc", DAT_100c96bc, param_1);
    }
}
