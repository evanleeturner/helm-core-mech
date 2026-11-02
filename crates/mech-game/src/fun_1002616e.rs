// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aca24: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1002616e() {
    mech_trace::enter!("1002616e");
    // SAFETY: the global is the game's own memory.
    unsafe {
        mech_trace::gw!("1002616e", "100aca24", DAT_100aca24, 1);
    }
}
