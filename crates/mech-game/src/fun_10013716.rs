// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1d0: u16;
    static mut DAT_100aa1f4: u32;
    static mut DAT_100aa2b8: u16;
    static mut DAT_100aa2ba: u8;
    static mut DAT_100adf98: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: u32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10013716() {
    mech_trace::enter!("10013716");
    let mut local: [u8; 3] = [0; 3];
    // SAFETY: the globals and the callee are the game's own; the local
    // buffer is valid for the three bytes written and passed by address.
    unsafe {
        mech_trace::wr!(
            "10013716",
            local.as_mut_ptr().cast::<u16>(),
            write_unaligned,
            mech_trace::gr!("10013716", "100aa2b8", DAT_100aa2b8)
        );
        local[2] = mech_trace::gr!("10013716", "100aa2ba", DAT_100aa2ba);
        mech_trace::cr!(
            "10013716",
            "10006e00",
            FUN_10006e00(local.as_mut_ptr() as usize as u32, 2)
        );
        if mech_trace::gr!("10013716", "100aa1f4", DAT_100aa1f4) == 1 {
            mech_trace::gu!("10013716", "100adf98", DAT_100adf98, |=, 1);
            mech_trace::gw!("10013716", "100aa1d0", DAT_100aa1d0, 1);
        }
    }
}
