// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f5480: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000d659();
    fn FUN_10050b20();
    fn FUN_1000d22d();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c8bd() {
    mech_trace::enter!("1000c8bd");
    // SAFETY: the callees and the global are the game's own memory and code,
    // called and accessed in the original's order.
    unsafe {
        if mech_trace::gr!("1000c8bd", "101f5480", DAT_101f5480) != 0 {
            FUN_1000d659();
            mech_trace::gw!("1000c8bd", "101f5480", DAT_101f5480, 0);
        }
        FUN_10050b20();
        FUN_1000d22d();
    }
}
