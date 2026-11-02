// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_101f52d8: u32;
    static mut DAT_101f5300: *mut u8;
    static mut DAT_100af0cc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___open(filename: *const i8, open_flag: i32, ...) -> i32;
    fn FUN_1008668b(param_1: i32, buf: u32, param_3: *mut i32) -> *mut u16;
    fn __close(file_handle: i32) -> i32;
    fn __write(file_handle: i32, buf: *const u8, max_char_count: u32) -> i32;
    fn FUN_1008664f(param_1: u32, buf: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047a54(param_1: *mut i8, buf: *mut u8) -> u32 {
    // SAFETY: the callees and globals are the game's own; the pointers are
    // the caller's and are passed through unchanged.
    unsafe {
        let file_handle = FID_conflict___open(param_1, 0x8101, 0x80);
        if file_handle == -1 {
            return 0;
        }
        let b = FUN_1008668b(DAT_100af17c as i32, 0xd, DAT_100af0cc as *mut i32);
        if b.is_null() {
            __close(file_handle);
            return 0;
        }
        __write(file_handle, b as *const u8, 0x20);
        __write(file_handle, buf, 0x300);
        __write(file_handle, DAT_101f5300, DAT_101f52d8);
        __close(file_handle);
        FUN_1008664f(0xd, DAT_100af0cc as *mut i32);
        1
    }
}
