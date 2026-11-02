// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c5c30: i32;
    static mut DAT_100c5c34: i32;
    static mut DAT_100c5c38: i32;
    static mut DAT_100c5c3c: i16;
    static mut DAT_100c5c3e: i16;
    static mut DAT_100c5c40: i32;
    static mut DAT_100c5c44: i32;
    static mut DAT_100c5c48: i32;
    static mut DAT_100c5c4c: i32;
    static mut DAT_100c5c50: i32;
    static mut DAT_100c5c54: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060010(width: i32, height: i32) {
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        DAT_100c5c30 = 0x28;
        DAT_100c5c3c = 1;
        DAT_100c5c3e = 8;
        DAT_100c5c34 = width;
        DAT_100c5c38 = (0u32.wrapping_sub(height as u32)) as i32;
        DAT_100c5c40 = 0;
        DAT_100c5c44 = 0;
        DAT_100c5c50 = 0;
        DAT_100c5c48 = 0;
        DAT_100c5c4c = 0;
        DAT_100c5c54 = 0;
    }
}
