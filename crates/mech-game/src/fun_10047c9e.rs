// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100480d9(param_1: *mut i8) -> *mut u32;
    #[link_name = "FID_conflict___open"]
    fn FID_conflict___open(filename: *mut i8, open_flag: i32, ...) -> i32;
    fn __write(file_handle: i32, buf: *mut core::ffi::c_void, max_char_count: u32) -> i32;
    fn __close(file_handle: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047c9e(param_1: *mut i8, buf: *mut core::ffi::c_void) -> u32 {
    mech_trace::enter!("10047c9e", param_1, buf);
    // SAFETY: the callees are the game's own C runtime and helpers; the
    // pointers are passed through untouched as the original does.
    unsafe {
        let uvar2: u32 = 0x80;
        let ivar1: i32 = 0x8101;
        let filename = mech_trace::cr!("10047c9e", "100480d9", FUN_100480d9(param_1)) as *mut i8;
        let fd = mech_trace::cr!(
            "10047c9e",
            "FID_conflict___open",
            FID_conflict___open(filename, ivar1, uvar2)
        );
        mech_trace::rt!(
            "10047c9e",
            if fd == -1 {
                0xffff_ffff
            } else {
                mech_trace::cr!("10047c9e", "__write", __write(fd, buf, 0xd6));
                mech_trace::cr!("10047c9e", "__close", __close(fd));
                0
            }
        )
    }
}
