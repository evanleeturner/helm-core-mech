// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_004081ac: Option<unsafe extern "stdcall" fn() -> i32>;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn LoadLibraryA(lpLibFileName: *const i8) -> *mut core::ffi::c_void;
    fn MessageBoxA(
        hWnd: *mut core::ffi::c_void,
        lpText: *const i8,
        lpCaption: *const i8,
        uType: u32,
    ) -> i32;
    fn GetProcAddress(
        hModule: *mut core::ffi::c_void,
        lpProcName: *const i8,
    ) -> Option<unsafe extern "stdcall" fn() -> i32>;
    fn FreeLibrary(hLibModule: *mut core::ffi::c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00401200(fmt: *const i8, ...);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00401550(param_1: u32, param_2: u32, param_3: u32) -> i32 {
    mech_trace::enter!("00401550", param_1, param_2, param_3);
    // SAFETY: all callees and the global are the game's own or the OS's; the
    // strings are NUL-terminated literals, and the looked-up ShellMain has the
    // signature the original casts it to.
    unsafe {
        let h_module = mech_trace::cr!(
            "00401550",
            "LoadLibraryA",
            LoadLibraryA(c"MW2SHELL.DLL".as_ptr())
        );
        if h_module.is_null() {
            mech_trace::cr!(
                "00401550",
                "MessageBoxA",
                MessageBoxA(
                    core::ptr::null_mut(),
                    c"One or more DLLs not found. Please reinstall MECHWARROR 2: Mercenaries."
                        .as_ptr(),
                    c"Mercenaries".as_ptr(),
                    0x10,
                )
            );
            return mech_trace::rt!("00401550", 1);
        }
        let p_fvar1 = mech_trace::cr!(
            "00401550",
            "GetProcAddress",
            GetProcAddress(h_module, c"ShellMain".as_ptr())
        );
        let p_fvar1 = match p_fvar1 {
            None => return mech_trace::rt!("00401550", 1),
            Some(f) => core::mem::transmute::<
                unsafe extern "stdcall" fn() -> i32,
                unsafe extern "stdcall" fn(*mut core::ffi::c_void, i32, u32, u32, u32) -> i32,
            >(f),
        };
        mech_trace::gw!(
            "00401550",
            "004081ac",
            DAT_004081ac,
            mech_trace::cr!(
                "00401550",
                "GetProcAddress",
                GetProcAddress(h_module, c"ShellWindowProc".as_ptr())
            )
        );
        let proc_addr = mech_trace::gr!("00401550", "004081ac", DAT_004081ac);
        if proc_addr.is_none() {
            return mech_trace::rt!("00401550", 1);
        }
        FUN_00401200(c"Entering Shell...\n".as_ptr());
        let i_var2 = mech_trace::cr!(
            "00401550",
            "ind",
            p_fvar1(h_module, 0, param_2, param_3, param_1)
        );
        FUN_00401200(c"Returned from Shell, retval=%d.\n".as_ptr(), i_var2);
        mech_trace::cr!("00401550", "FreeLibrary", FreeLibrary(h_module));
        mech_trace::rt!("00401550", i_var2)
    }
}
