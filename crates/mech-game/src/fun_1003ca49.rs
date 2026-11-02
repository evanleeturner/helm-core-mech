// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefac: u8;
    static mut DAT_100aefb0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_send_channel_voice_message_20(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003ca49() {
    mech_trace::enter!("1003ca49");
    // SAFETY: the globals and the callee are the game's own; they are read
    // and called in the original's order.
    unsafe {
        if mech_trace::gr!("1003ca49", "100aefb0", DAT_100aefb0) != 0
            && (mech_trace::gr!("1003ca49", "100aefac", DAT_100aefac) as i8) >= 0
        {
            mech_trace::cr!(
                "1003ca49",
                "_AIL_send_channel_voice_message_20",
                _AIL_send_channel_voice_message_20(
                    mech_trace::gr!("1003ca49", "100aefb0", DAT_100aefb0),
                    0,
                    (mech_trace::gr!("1003ca49", "100aefac", DAT_100aefac) as u32) | 0xb0,
                    7,
                    0
                )
            );
        }
    }
}
