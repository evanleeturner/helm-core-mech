// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4104: u32;
    static mut DAT_100aafa4: u32;
    static mut DAT_100adcd0: u32;
    static mut DAT_100b09f8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10002415() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055c40(param_1: u32) -> u32 {
    mech_trace::enter!("10055c40", param_1);
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        mech_trace::rt!(
            "10055c40",
            match param_1 {
                0x2a => mech_trace::gr!("10055c40", "100adcd0", DAT_100adcd0),
                0x53 => mech_trace::gr!("10055c40", "100a4104", DAT_100a4104),
                0x57 => mech_trace::gr!("10055c40", "100b09f8", DAT_100b09f8),
                0xbf => mech_trace::cr!("10055c40", "10002415", FUN_10002415()),
                0xc0 => (mech_trace::gr!("10055c40", "100aafa4", DAT_100aafa4) == 1) as u32,
                _ => 0,
            }
        )
    }
}
