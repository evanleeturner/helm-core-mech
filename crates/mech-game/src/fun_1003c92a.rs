// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100aefac: u8;
    static mut DAT_100aefb0: u32;
    static mut DAT_100b3c4c: u32;
    static mut DAT_100eef40: [u32; 1];
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

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c92a(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; the table entry
    // and the structure it points to are valid game memory read as the
    // original does.
    unsafe {
        if DAT_100aefb0 != 0 && (DAT_100aefac as i8) >= 0 {
            let entry = (&raw const DAT_100eef40)
                .cast::<u32>()
                .add(DAT_100adf38 as usize)
                .read_unaligned();
            let inner = ((entry.wrapping_add(0x20)) as *const u32).read_unaligned();
            let val = ((inner.wrapping_add(0x3d4)) as *const i32).read_unaligned();
            if 1 < val {
                let u_var1 = (param_1.wrapping_mul(4).wrapping_add(0x2000)) as u32;
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    (DAT_100aefac as u32) | 0xe0,
                    u_var1 & 0x7f,
                    (u_var1 & 0x3f00) >> 8,
                );
                let u_var1 = FUN_10003d60(DAT_100b3c4c as i32, 100) as u32;
                _AIL_send_channel_voice_message_20(
                    DAT_100aefb0,
                    0,
                    (DAT_100aefac as u32) | 0xb0,
                    7,
                    u_var1 & 0xff,
                );
            }
        }
    }
}
