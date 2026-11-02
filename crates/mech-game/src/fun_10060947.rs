// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36f4: u32;
    static mut DAT_100b36f8: u32;
    static mut DAT_100c7e60: [u8; 1];
    static mut DAT_100af0c8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100718bd(param_1: *mut u32) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10060517();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10060947() {
    mech_trace::enter!("10060947");
    // SAFETY: the globals and callees are the game's own; the array's address
    // is passed on as the original does.
    unsafe {
        if mech_trace::gr!("10060947", "100b36f4", DAT_100b36f4) != 0 {
            mech_trace::cr!(
                "10060947",
                "100718bd",
                FUN_100718bd((&raw mut DAT_100c7e60).cast::<u32>())
            );
            mech_trace::gw!("10060947", "100b36f4", DAT_100b36f4, 0);
            mech_trace::cr!(
                "10060947",
                "1008664f",
                FUN_1008664f(
                    mech_trace::gr!("10060947", "100b36f8", DAT_100b36f8),
                    mech_trace::gr!("10060947", "100af0c8", DAT_100af0c8) as *mut i32
                )
            );
            mech_trace::gw!("10060947", "100b36f8", DAT_100b36f8, 0xffffffff);
            FUN_10060517();
        }
    }
}
