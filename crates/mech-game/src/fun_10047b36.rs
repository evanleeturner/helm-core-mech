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
    mech_trace::enter!("10047b36", param_1, param_2, param_3, param_4);
    // SAFETY: callees and globals are the game's own; the pointer parameters
    // are the caller's valid out-slots, read and written where the original does.
    unsafe {
        let mut file_handle = mech_trace::cr!(
            "10047b36",
            "FID_conflict___open",
            FID_conflict___open(param_1, 0x8000)
        );
        if file_handle == -1 {
            file_handle = -1;
        } else {
            let s_var1 =
                mech_trace::cr!("10047b36", "__filelength", __filelength(file_handle)) as u32;
            mech_trace::wr!("10047b36", param_2, write_unaligned, s_var1);
            if param_4.is_null() {
                let pv = mech_trace::cr!(
                    "10047b36",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("10047b36", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("10047b36", param_2, read_unaligned)
                    )
                );
                mech_trace::wr!("10047b36", param_3, write_unaligned, pv as i32);
            } else {
                let i_var3 = mech_trace::cr!(
                    "10047b36",
                    "100814bb",
                    FUN_100814bb(
                        mech_trace::rd!("10047b36", param_2, read_unaligned) as i32,
                        mech_trace::rd!("10047b36", param_4, read_unaligned)
                    )
                );
                mech_trace::wr!("10047b36", param_3, write_unaligned, i_var3);
            }
            if mech_trace::rd!("10047b36", param_3, read_unaligned) == 0 {
                mech_trace::cr!("10047b36", "__close", __close(file_handle));
                file_handle = -1;
            } else {
                let s = mech_trace::cr!(
                    "10047b36",
                    "__read",
                    __read(
                        file_handle,
                        mech_trace::rd!("10047b36", param_3, read_unaligned)
                            as *mut core::ffi::c_void,
                        mech_trace::rd!("10047b36", param_2, read_unaligned),
                    )
                ) as u32;
                if s != mech_trace::rd!("10047b36", param_2, read_unaligned) {
                    if param_4.is_null() {
                        mech_trace::cr!(
                            "10047b36",
                            "HeapFree",
                            HeapFree(
                                mech_trace::gr!("10047b36", "100adf74", DAT_100adf74),
                                1,
                                mech_trace::rd!("10047b36", param_3, read_unaligned)
                                    as *mut core::ffi::c_void,
                            )
                        );
                    }
                    mech_trace::cr!("10047b36", "__close", __close(file_handle));
                    file_handle = -1;
                }
            }
        }
        mech_trace::rt!("10047b36", file_handle)
    }
}
