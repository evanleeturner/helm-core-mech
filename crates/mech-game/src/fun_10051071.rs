// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000387d() -> u32;
    fn FUN_100035dd(param_1: u32, param_2: u32, param_3: u32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10051071() {
    // SAFETY: both callees are the game's own functions; arguments are passed
    // in the original order with the original constants.
    unsafe {
        let u_var1 = FUN_1000387d();
        FUN_100035dd(0xd7, 100, 0x40, u_var1);
    }
}
