// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: *mut u8;
    static mut DAT_100c6d00: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003dd75(param_1: i32) {
    mech_trace::enter!("1003dd75", param_1);
    // SAFETY: the globals and callees are the game's own; only the address of
    // DAT_100c6d00 is taken and the pointer global is read by value.
    unsafe {
        if mech_trace::gr!("1003dd75", "100af098", DAT_100af098) == (&raw mut DAT_100c6d00)
            && param_1 != 0
        {
            mech_trace::cr!("1003dd75", "1003e369", FUN_1003e369(param_1));
            mech_trace::cr!("1003dd75", "1003e3e3", FUN_1003e3e3(param_1, 0x100c7ac8));
        }
    }
}
