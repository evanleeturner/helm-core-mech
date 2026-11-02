// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefac: u8;
    static mut DAT_100aefb0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    #[link_name = "AIL_send_channel_voice_message"]
    fn _AIL_send_channel_voice_message_20(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
    ) -> u32;
    #[link_name = "AIL_release_channel"]
    fn _AIL_release_channel_8(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c9e2() {
    // SAFETY: the globals and the AIL callees are the game's own; they are
    // read, called and written in the original's order.
    unsafe {
        if DAT_100aefb0 != 0 && (DAT_100aefac as i8) >= 0 {
            _AIL_send_channel_voice_message_20(
                DAT_100aefb0,
                0,
                (DAT_100aefac as u32) | 0x80,
                0x26,
                0x7f,
            );
            _AIL_release_channel_8(DAT_100aefb0, (DAT_100aefac as u32).wrapping_add(1));
            DAT_100aefac = 0xff;
        }
    }
}
