// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_10160bdc: u32;
    static mut DAT_10160be0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10048832(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004839c() {
    mech_trace::enter!("1004839c");
    // SAFETY: the globals and callees are the game's own; each list node
    // is read at offset 0x14 before being freed.
    unsafe {
        let mut local_c: u32 = mech_trace::gr!("1004839c", "10160be0", DAT_10160be0);
        while local_c != 0 {
            mech_trace::cr!("1004839c", "10048832", FUN_10048832(local_c as i32));
            let pv_var1: u32 = mech_trace::rd!(
                "1004839c",
                ((local_c as usize + 0x14) as *const u32),
                read_unaligned
            );
            mech_trace::cr!(
                "1004839c",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1004839c", "100adf74", DAT_100adf74),
                    1,
                    local_c as usize as *mut core::ffi::c_void
                )
            );
            local_c = pv_var1;
        }
        mech_trace::gw!("1004839c", "10160be0", DAT_10160be0, 0);
        mech_trace::gw!("1004839c", "10160bdc", DAT_10160bdc, 0);
    }
}
