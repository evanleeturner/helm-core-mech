// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_004081b0: Option<unsafe extern "stdcall" fn() -> i32>;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00401200(fmt: *const i8, ...);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn LoadLibraryA(lp_lib_file_name: *const i8) -> *mut core::ffi::c_void;
    fn GetProcAddress(
        h_module: *mut core::ffi::c_void,
        lp_proc_name: *const i8,
    ) -> Option<unsafe extern "stdcall" fn() -> i32>;
    fn FreeLibrary(h_lib_module: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004014b0(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> i32 {
    // SAFETY: the Win32 and game functions and the global are the game's own;
    // the strings are NUL-terminated literals, and the resolved SimMain has
    // the stdcall signature the original casts it to.
    unsafe {
        let h_module = LoadLibraryA(c"MERCSW.DLL".as_ptr());
        if h_module.is_null() {
            return 1;
        }
        let p_fvar1 = GetProcAddress(h_module, c"SimMain".as_ptr());
        let sim_main = match p_fvar1 {
            None => return 1,
            Some(f) => f,
        };
        DAT_004081b0 = GetProcAddress(h_module, c"SimWindowProc".as_ptr());
        let window_proc = DAT_004081b0;
        if window_proc.is_none() {
            return 1;
        }
        FUN_00401200(c"Entering Sim...\n".as_ptr());
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, i32, u32, u32, u32, u32) -> i32 =
            core::mem::transmute(sim_main);
        let i_var2 = f(h_module, 0, param_2, param_3, param_4, param_1);
        FUN_00401200(c"Returned from Sim, retval=%d.\n".as_ptr(), i_var2);
        FreeLibrary(h_module);
        i_var2
    }
}
