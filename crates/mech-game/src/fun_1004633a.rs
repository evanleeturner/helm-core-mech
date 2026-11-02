// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004633a() -> u32 {
    mech_trace::enter!("1004633a");
    let mut local_8: u32 = 0;
    // SAFETY: the global and the MCI call are the game's own; local_8 lives
    // on this frame for the duration of the call.
    unsafe {
        mech_trace::cr!(
            "1004633a",
            "mciSendCommandA",
            mciSendCommandA(
                mech_trace::gr!("1004633a", "100b04b0", DAT_100b04b0),
                0x804,
                0,
                (&raw mut local_8) as usize as u32
            )
        );
        mech_trace::gw!("1004633a", "100b04b0", DAT_100b04b0, 0xffffffff);
    }
    mech_trace::rt!("1004633a", 0)
}
