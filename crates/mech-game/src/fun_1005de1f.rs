// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab238: u32;
    static mut DAT_100ab23c: u32;
    static mut DAT_100b3c9c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1001d916(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005de1f(_param_1: u32, param_2: i32) {
    // SAFETY: the callees and globals are the game's own; DAT_100b3c9c holds
    // a pointer to a structure whose field at offset 0x34 is written.
    unsafe {
        DAT_100ab238 = FUN_10003d60(param_2, 0xf) as u32;
        DAT_100ab23c = DAT_100ab238;
        ((DAT_100b3c9c as usize + 0x34) as *mut u32).write_unaligned(DAT_100ab238);
        FUN_1001d916(DAT_100ab238);
    }
}
