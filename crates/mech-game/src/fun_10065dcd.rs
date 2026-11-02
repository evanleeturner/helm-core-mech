// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100b3920: u32;
    static mut DAT_100b3924: u32;
    static mut DAT_100b3928: u32;
    static mut DAT_100b3930: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a72(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065dcd() {
    mech_trace::enter!("10065dcd");
    // SAFETY: the callee and the globals are the game's own memory; reads and
    // writes happen in the original's order.
    unsafe {
        mech_trace::cr!(
            "10065dcd",
            "10036a72",
            FUN_10036a72(mech_trace::gr!("10065dcd", "100b3924", DAT_100b3924))
        );
        mech_trace::cr!(
            "10065dcd",
            "10036a72",
            FUN_10036a72(mech_trace::gr!("10065dcd", "100b3928", DAT_100b3928))
        );
        mech_trace::cr!(
            "10065dcd",
            "10036a72",
            FUN_10036a72(mech_trace::gr!("10065dcd", "100b3920", DAT_100b3920))
        );
        mech_trace::gw!("10065dcd", "100b3930", DAT_100b3930, 0);
        mech_trace::gw!("10065dcd", "100b38fc", DAT_100b38fc, 0);
    }
}
