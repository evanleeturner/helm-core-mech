// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36d4: u32;
    static mut DAT_100b36d8: u32;
    static mut DAT_100b36e0: u32;
    static mut DAT_100b36f0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10060563(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10060517() {
    // SAFETY: the globals and the callee are the game's own memory and code,
    // read, written and called in the original's order.
    unsafe {
        if DAT_100b36d8 != 0 && DAT_100b36f0 < 1 {
            DAT_100b36d8 = 0;
            FUN_10060563(DAT_100b36e0 as i32);
            DAT_100b36d4 = DAT_100b36e0;
        }
    }
}
