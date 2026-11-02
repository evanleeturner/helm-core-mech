// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefac: i8;
    static mut DAT_100aefb0: u32;
    static mut DAT_100aefb8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_lock_channel_4(param_1: u32) -> u32;
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
pub unsafe extern "stdcall" fn FUN_1003c807() {
    // SAFETY: the globals and the AIL functions are the game's own; they are
    // read, written and called in the original's order.
    unsafe {
        if DAT_100aefb0 != 0 && DAT_100aefac < 0 && DAT_100aefb8 != 0 {
            let c_var1 = _AIL_lock_channel_4(DAT_100aefb0) as i8;
            if c_var1 == 0 {
                DAT_100aefac = 0;
            } else {
                DAT_100aefac = c_var1.wrapping_sub(1);
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    ((DAT_100aefac as i32) | 0xb0) as u32,
                    0x72,
                    0,
                );
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    ((DAT_100aefac as i32) | 0xc0) as u32,
                    3,
                    0,
                );
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    ((DAT_100aefac as i32) | 0xe0) as u32,
                    0,
                    0x30,
                );
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    ((DAT_100aefac as i32) | 0xb0) as u32,
                    10,
                    0x40,
                );
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    ((DAT_100aefac as i32) | 0xb0) as u32,
                    7,
                    0,
                );
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    ((DAT_100aefac as i32) | 0x90) as u32,
                    0x26,
                    0x7f,
                );
            }
        }
    }
}
