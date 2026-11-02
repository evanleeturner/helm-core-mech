// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut s_mw2rmg_cfg_1008d088: [i8; 0];
    static mut DAT_1008d084: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006eee0(filename: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn FUN_1006ece0(
        buffer: *mut core::ffi::c_void,
        size: u32,
        count: u32,
        stream: *mut core::ffi::c_void,
    ) -> u32;
    fn FUN_1006ec40(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cfa0(dst_buf: *mut core::ffi::c_void) -> u32 {
    mech_trace::enter!("1000cfa0", dst_buf);
    // SAFETY: the callees and globals are the game's own; dst_buf is the
    // caller's buffer, filled by fread exactly as the original does.
    unsafe {
        let file = mech_trace::cr!(
            "1000cfa0",
            "1006eee0",
            FUN_1006eee0(
                (&raw mut s_mw2rmg_cfg_1008d088).cast::<i8>(),
                (&raw mut DAT_1008d084).cast::<i8>(),
            )
        );
        if !file.is_null() {
            mech_trace::cr!("1000cfa0", "1006ece0", FUN_1006ece0(dst_buf, 0xf3, 1, file));
            let i_var1 = mech_trace::cr!("1000cfa0", "1006ec40", FUN_1006ec40(file));
            return mech_trace::rt!("1000cfa0", ((i_var1 as u32) & 0xFFFFFF00) | 1);
        }
        mech_trace::rt!("1000cfa0", 0)
    }
}
