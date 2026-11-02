// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100480d9(param_1: *mut i8) -> *mut u32;
    fn FID_conflict___open(_Filename: *mut i8, _OpenFlag: i32, ...) -> i32;
    fn __write(_FileHandle: i32, _Buf: *mut core::ffi::c_void, _MaxCharCount: u32) -> i32;
    fn __close(_FileHandle: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047e21(param_1: *mut i8, buf: *mut core::ffi::c_void) -> u32 {
    // SAFETY: the callees are the game's own functions; the arguments are
    // passed through exactly as the original does.
    unsafe {
        let u_var2: u32 = 0x80;
        let i_var1: i32 = 0x8101;
        let filename = FUN_100480d9(param_1) as *mut i8;
        let i_var1 = FID_conflict___open(filename, i_var1, u_var2);
        if i_var1 == -1 {
            0xffffffff
        } else {
            __write(i_var1, buf, 0x17);
            __close(i_var1);
            0
        }
    }
}
