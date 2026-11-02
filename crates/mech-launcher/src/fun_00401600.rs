// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_004081bc: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FindWindowA(lpClassName: *const i8, lpWindowName: *const i8) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00401600() -> bool {
    // SAFETY: FindWindowA and the string global are the game's own; the
    // string is a NUL-terminated array that is only read by address.
    unsafe {
        let h_var1 = FindWindowA(
            (&raw mut DAT_004081bc).cast::<i8>(),
            (&raw mut DAT_004081bc).cast::<i8>(),
        );
        if !h_var1.is_null() {
            return true;
        }
        let h_var1 = FindWindowA(
            (&raw mut DAT_004081bc).cast::<i8>(),
            (&raw mut DAT_004081bc).cast::<i8>(),
        );
        !h_var1.is_null()
    }
}
