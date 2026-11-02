// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut u8;
    static mut DAT_100ab6e8: u8;
    static mut s_Out_of_memory_in_allocate_string_100a5984: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn _fflush(file: *mut u8) -> i32;
    fn FUN_1006fde0(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(heap: *mut u8, flags: u32, size: u32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005bb20(param_1: *mut i8) -> *mut i8 {
    mech_trace::enter!("1005bb20", param_1);
    // SAFETY: param_1 is a NUL-terminated string supplied by the caller; the
    // heap handle, file stream and callees are the game's own.
    unsafe {
        let mut len: u32 = 0;
        while mech_trace::dr!("1005bb20", param_1.byte_add(len as usize)) != 0 {
            len = len.wrapping_add(1);
        }
        if len == 0 {
            return mech_trace::rt!("1005bb20", core::ptr::null_mut());
        }
        let total = len.wrapping_add(1);
        let dst = mech_trace::cr!(
            "1005bb20",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1005bb20", "10098224", DAT_10098224),
                1,
                total
            )
        ) as *mut i8;
        if dst.is_null() {
            mech_trace::cr!(
                "1005bb20",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(
                    &raw mut DAT_100ab6e8,
                    &raw mut s_Out_of_memory_in_allocate_string_100a5984 as *const u8,
                )
            );
            mech_trace::cr!("1005bb20", "_fflush", _fflush(&raw mut DAT_100ab6e8));
            FUN_1006fde0(1);
        }
        let mut src = param_1 as *const i8;
        let mut out = dst;
        let mut n = total >> 2;
        while n != 0 {
            let v = mech_trace::rd!("1005bb20", src.cast::<u32>(), read_unaligned);
            mech_trace::wr!("1005bb20", out.cast::<u32>(), write_unaligned, v);
            src = src.byte_add(4);
            out = out.byte_add(4);
            n -= 1;
        }
        let mut r = total & 3;
        while r != 0 {
            mech_trace::dw!("1005bb20", out, mech_trace::dr!("1005bb20", src));
            src = src.byte_add(1);
            out = out.byte_add(1);
            r -= 1;
        }
        mech_trace::rt!("1005bb20", dst)
    }
}
