// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a62cc: i32;
    static mut DAT_100ab6e8: u8;
    static mut s_Error___Missing_BWDBegin__100a63c8: u8;
    static mut DAT_100a63e4: u8;
    static mut s_Error___Unable_to_create_BWD_fil_100a63e8: u8;
    static mut DAT_100a62c0: u32;
    static mut DAT_100a50f4: i32;
    static mut s_File__s_not_generated_because_of_100a6410: u8;
    static mut s_Deleting__s_100a644c: u8;
    static mut s_WLD_to_BWD_conversion_was_comple_100a645c: u8;
    static mut DAT_100c6064: i32;
    static mut _DAT_100c6068: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn FUN_1006fde0(param_1: u32);
    fn __wfopen(filename: *const i8, mode: *const u8) -> *mut u8;
    fn _wprintf(format: *const u8, ...) -> i32;
    fn FUN_10011260(param_1: *mut u8) -> u32;
    fn _fclose(file: *mut u8) -> i32;
    fn FUN_1006f750(param_1: *const i8) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005b510();
    fn FUN_1000d9b0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e6f0(param_1: *mut i8) -> u32 {
    mech_trace::enter!("1005e6f0", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 is the
    // caller's file name string, passed through unchanged.
    unsafe {
        if mech_trace::gr!("1005e6f0", "100a62cc", DAT_100a62cc) == 0 {
            mech_trace::cr!(
                "1005e6f0",
                "_fwprintf",
                _fwprintf(
                    &raw mut DAT_100ab6e8,
                    &raw const s_Error___Missing_BWDBegin__100a63c8,
                )
            );
            FUN_1006fde0(0);
        } else {
            mech_trace::gw!(
                "1005e6f0",
                "100a62cc",
                DAT_100a62cc,
                mech_trace::gr!("1005e6f0", "100a62cc", DAT_100a62cc).wrapping_sub(1)
            );
        }
        let file = mech_trace::cr!(
            "1005e6f0",
            "__wfopen",
            __wfopen(param_1, &raw const DAT_100a63e4)
        );
        if file.is_null() {
            let i_var1 = mech_trace::cr!(
                "1005e6f0",
                "_wprintf",
                _wprintf(
                    &raw const s_Error___Unable_to_create_BWD_fil_100a63e8,
                    param_1,
                )
            );
            return mech_trace::rt!("1005e6f0", ((i_var1 as u32) & 0xffff0000) | 1);
        }
        let u_var2 = mech_trace::cr!("1005e6f0", "10011260", FUN_10011260(file));
        mech_trace::gu!("1005e6f0", "100a62c0", DAT_100a62c0, &=, u_var2);
        FUN_1005b510();
        FUN_1000d9b0();
        mech_trace::cr!("1005e6f0", "_fclose", _fclose(file));
        if mech_trace::gr!("1005e6f0", "100a50f4", DAT_100a50f4) != 0 {
            mech_trace::cr!(
                "1005e6f0",
                "_wprintf",
                _wprintf(
                    &raw const s_File__s_not_generated_because_of_100a6410,
                    param_1,
                )
            );
            mech_trace::cr!(
                "1005e6f0",
                "_wprintf",
                _wprintf(&raw const s_Deleting__s_100a644c, param_1)
            );
            mech_trace::cr!("1005e6f0", "1006f750", FUN_1006f750(param_1));
            mech_trace::gw!("1005e6f0", "100c6064", DAT_100c6064, 0);
            return mech_trace::rt!("1005e6f0", 0);
        }
        if mech_trace::gr!("1005e6f0", "100a62c0", DAT_100a62c0) == 0 {
            mech_trace::cr!(
                "1005e6f0",
                "_wprintf",
                _wprintf(&raw const s_WLD_to_BWD_conversion_was_comple_100a645c)
            );
            mech_trace::gw!("1005e6f0", "100c6064", DAT_100c6064, 0);
            return mech_trace::rt!("1005e6f0", 0);
        }
        if mech_trace::gr!("1005e6f0", "100c6064", DAT_100c6064) != 0 {
            mech_trace::gw!(
                "1005e6f0",
                "100c6068",
                _DAT_100c6068,
                mech_trace::gr!("1005e6f0", "100c6068", _DAT_100c6068).wrapping_add(1)
            );
        }
        mech_trace::gw!("1005e6f0", "100c6064", DAT_100c6064, 0);
        mech_trace::rt!("1005e6f0", 0)
    }
}
