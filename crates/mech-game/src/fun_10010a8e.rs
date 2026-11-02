// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004726c();
    fn FUN_1003289f();
    fn FUN_100861a3();
    fn FUN_10012575();
    fn FUN_10088e1a();
    fn FUN_1005dce9();
    fn FUN_1000ed10();
    fn FUN_10065cdb();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10010a8e() {
    mech_trace::enter!("10010a8e");
    // SAFETY: the callees are the game's own functions, called in the original order.
    unsafe {
        FUN_1004726c();
        FUN_1003289f();
        FUN_100861a3();
        FUN_10012575();
        FUN_10088e1a();
        FUN_1005dce9();
        FUN_1000ed10();
        FUN_10065cdb();
    }
}
