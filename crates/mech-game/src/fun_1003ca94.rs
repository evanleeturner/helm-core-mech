// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefac: u8;
    static mut DAT_100aefb0: u32;
    static mut DAT_100b3c4c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    #[link_name = "AIL_send_channel_voice_message"]
    fn AIL_send_channel_voice_message_20(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003ca94() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // values are read and passed in the original's order.
    unsafe {
        if DAT_100aefb0 != 0 && (DAT_100aefac as i8) >= 0 {
            let u_var1 = FUN_10003d60(DAT_100b3c4c as i32, 100) as u32;
            AIL_send_channel_voice_message_20(
                DAT_100aefb0,
                0,
                (DAT_100aefac as i32 | 0xb0) as u32,
                7,
                u_var1 & 0xff,
            );
        }
    }
}
