// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f984: *mut i8;
    static mut DAT_1008fc94: u8;
    static mut DAT_100a6730: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1006ec30(param_1: *mut core::ffi::c_void);
    fn FUN_1006f200(param_1: u32) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005ef70(param_1: i32);
    fn FUN_1005f810(param_1: *mut core::ffi::c_void);
    fn FUN_1005f890(param_1: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005ee60(
        this: *mut core::ffi::c_void,
        param_1: *mut i8,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: i32,
    ) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ace0(param_1: *mut i8) {
    mech_trace::enter!("1001ace0", param_1);
    let mut local_34: [i8; 32] = [0; 32];
    // SAFETY: the callees and globals are the game's own; param_1 is a
    // NUL-terminated string from the caller, and local_34 is the local buffer.
    unsafe {
        let mut len: u32 = 0;
        while mech_trace::dr!("1001ace0", param_1.byte_add(len as usize)) != 0 {
            len = len.wrapping_add(1);
        }
        if len == 0 {
            if !mech_trace::gr!("1001ace0", "1008f984", DAT_1008f984).is_null() {
                FUN_1005f890(mech_trace::gr!("1001ace0", "1008f984", DAT_1008f984).cast());
            }
        } else {
            mech_trace::cr!(
                "1001ace0",
                "_sprintf",
                _sprintf(
                    local_34.as_mut_ptr(),
                    (&raw const DAT_1008fc94).cast::<i8>(),
                    param_1,
                )
            );
            let old = mech_trace::gr!("1001ace0", "1008f984", DAT_1008f984);
            if !old.is_null() {
                FUN_1005ef70(old as i32);
                FUN_1006ec30(old.cast());
            }
            let obj = mech_trace::cr!("1001ace0", "1006f200", FUN_1006f200(0x144));
            let mut result: *mut i8 = core::ptr::null_mut();
            if !obj.is_null() {
                result = mech_trace::cr!(
                    "1001ace0",
                    "1005ee60",
                    FUN_1005ee60(
                        obj,
                        local_34.as_mut_ptr(),
                        0x1b9,
                        0x1a,
                        0x68,
                        0,
                        mech_trace::gr!("1001ace0", "100a6730", DAT_100a6730),
                    )
                );
            }
            mech_trace::gw!("1001ace0", "1008f984", DAT_1008f984, result);
            FUN_1005f810(result.cast());
        }
    }
}
