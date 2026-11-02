// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___open(_filename: *mut i8, _open_flag: i32, ...) -> i32;
    fn __filelength(_file_handle: i32) -> i32;
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
    fn __close(_file_handle: i32) -> i32;
    fn __read(_file_handle: i32, _dst_buf: *mut core::ffi::c_void, _max_char_count: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, p: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047b36(
    param_1: *mut i8,
    param_2: *mut u32,
    param_3: *mut i32,
    param_4: *mut i32,
) -> i32 {
    // SAFETY: callees and globals are the game's own; the pointer parameters
    // are the caller's valid out-slots, read and written where the original does.
    unsafe {
        let mut file_handle = FID_conflict___open(param_1, 0x8000);
        if file_handle == -1 {
            file_handle = -1;
        } else {
            let s_var1 = __filelength(file_handle) as u32;
            param_2.write_unaligned(s_var1);
            if param_4.is_null() {
                let pv = HeapAlloc(DAT_100adf74, 1, param_2.read_unaligned());
                param_3.write_unaligned(pv as i32);
            } else {
                let i_var3 =
                    FUN_100814bb(param_2.read_unaligned() as i32, param_4.read_unaligned());
                param_3.write_unaligned(i_var3);
            }
            if param_3.read_unaligned() == 0 {
                __close(file_handle);
                file_handle = -1;
            } else {
                let s = __read(
                    file_handle,
                    param_3.read_unaligned() as *mut core::ffi::c_void,
                    param_2.read_unaligned(),
                ) as u32;
                if s != param_2.read_unaligned() {
                    if param_4.is_null() {
                        HeapFree(
                            DAT_100adf74,
                            1,
                            param_3.read_unaligned() as *mut core::ffi::c_void,
                        );
                    }
                    __close(file_handle);
                    file_handle = -1;
                }
            }
        }
        file_handle
    }
}
