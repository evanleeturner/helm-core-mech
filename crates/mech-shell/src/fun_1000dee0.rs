// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut core::ffi::c_void;
    static mut DAT_1008d55c: *mut core::ffi::c_void;
    static mut DAT_1008d560: *mut core::ffi::c_void;
    static mut DAT_100a725c: *mut *mut core::ffi::c_void;
    static mut DAT_100a7560: *mut core::ffi::c_void;
    static mut DAT_100d1540: i32;
    static mut DAT_100d1544: *mut core::ffi::c_void;
    static mut DAT_1008d158: [u8; 0];
    static mut DAT_100c5c30: [u8; 0];
    static mut s_GDI_CreateDIBSection_failed___d_1008d9e8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SetDIBColorTable(
        hdc: *mut core::ffi::c_void,
        i_start: u32,
        c_entries: u32,
        prgbq: *const core::ffi::c_void,
    ) -> u32;
    fn CreateDIBSection(
        hdc: *mut core::ffi::c_void,
        pbmi: *const core::ffi::c_void,
        usage: u32,
        pp_bits: *mut *mut core::ffi::c_void,
        h_section: *mut core::ffi::c_void,
        offset: u32,
    ) -> *mut core::ffi::c_void;
    fn SelectObject(
        hdc: *mut core::ffi::c_void,
        h: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn GetLastError() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(fmt: *const i8, ...);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000dee0() -> i32 {
    // SAFETY: globals and callees are the game's own memory and functions;
    // pointers are passed through exactly as the original does.
    unsafe {
        if !DAT_1008d560.is_null() {
            DAT_100d1540 =
                SetDIBColorTable(DAT_1008d55c, 0, 0x100, (&raw const DAT_1008d158).cast()) as i32;
            return 1;
        }
        DAT_1008d560 = CreateDIBSection(
            DAT_1008d150,
            (&raw const DAT_100c5c30).cast(),
            1,
            DAT_100a725c,
            core::ptr::null_mut(),
            0,
        );
        if !DAT_1008d560.is_null() && !(*DAT_100a725c).is_null() {
            DAT_100a7560 = *DAT_100a725c;
            DAT_100d1544 = SelectObject(DAT_1008d55c, DAT_1008d560);
            return 1;
        }
        FUN_100640a0(
            (&raw const s_GDI_CreateDIBSection_failed___d_1008d9e8).cast(),
            GetLastError(),
        );
        0
    }
}
