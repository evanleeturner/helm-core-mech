// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0574: *mut c_void;
    static mut DAT_10160be8: *mut *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086951(size: u32) -> *mut c_void;
    fn FUN_100108a0(code: u32, param: *mut c_void);
    fn _strncpy(dst: *mut i8, src: *const i8, count: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100470e0(param_1: *mut u16) -> u32 {
    mech_trace::enter!("100470e0", param_1);
    let mut local_8: u32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's record and the allocation has 0xf bytes, accessed as the
    // original does.
    unsafe {
        let p_var2 = mech_trace::cr!("100470e0", "10086951", FUN_10086951(0xf));
        if p_var2.is_null() {
            FUN_100108a0(0x42, core::ptr::null_mut());
        } else {
            mech_trace::wr!("100470e0", p_var2.cast::<u32>(), write_unaligned, 0);
            mech_trace::wr!(
                "100470e0",
                p_var2.byte_add(4).cast::<u16>(),
                write_unaligned,
                mech_trace::rd!("100470e0", param_1, read_unaligned)
            );
            mech_trace::cr!(
                "100470e0",
                "_strncpy",
                _strncpy(
                    p_var2.byte_add(6).cast::<i8>(),
                    param_1.byte_add(2).cast::<i8>(),
                    8,
                )
            );
            mech_trace::wr!("100470e0", p_var2.byte_add(0xe).cast::<u8>(), write, 0);
            let mut p_var1 = p_var2;
            if !mech_trace::gr!("100470e0", "100b0574", DAT_100b0574).is_null() {
                mech_trace::wr!(
                    "100470e0",
                    mech_trace::gr!("100470e0", "10160be8", DAT_10160be8),
                    write_unaligned,
                    p_var2
                );
                p_var1 = mech_trace::gr!("100470e0", "100b0574", DAT_100b0574);
            }
            mech_trace::gw!("100470e0", "100b0574", DAT_100b0574, p_var1);
            local_8 = 1;
            mech_trace::gw!(
                "100470e0",
                "10160be8",
                DAT_10160be8,
                p_var2.cast::<*mut c_void>()
            );
        }
    }
    mech_trace::rt!("100470e0", local_8)
}
