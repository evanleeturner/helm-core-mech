// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36dc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006065d(param_1: u32, param_2: u32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060864(param_1: i32, param_2: u32, param_3: i32) {
    // SAFETY: the global and the callee are the game's own; arguments are
    // passed through in the original order.
    unsafe {
        FUN_1006065d(DAT_100b36dc.wrapping_add(param_1 as u32), param_2, param_3);
    }
}
