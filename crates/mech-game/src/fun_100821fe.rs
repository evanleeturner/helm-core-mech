// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9250: u32;
    static mut DAT_100c9254: u32;
    static mut DAT_100c9258: i32;
    static mut DAT_100c925c: u16;
    static mut DAT_100c925e: u16;
    static mut DAT_100c9260: u32;
    static mut DAT_100c9264: u32;
    static mut DAT_100c9268: u32;
    static mut DAT_100c926c: u32;
    static mut DAT_100c9270: u32;
    static mut DAT_100c9274: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100821fe(param_1: u32, param_2: i32) {
    // SAFETY: the globals are the game's own memory, written in the original order.
    unsafe {
        DAT_100c9250 = 0x28;
        DAT_100c9254 = param_1;
        DAT_100c9258 = (0u32.wrapping_sub(param_2 as u32)) as i32;
        DAT_100c925c = 1;
        DAT_100c925e = 8;
        DAT_100c9260 = 0;
        DAT_100c9264 = 0;
        DAT_100c9270 = 0;
        DAT_100c9268 = 0;
        DAT_100c926c = 0;
        DAT_100c9274 = 0;
    }
}
