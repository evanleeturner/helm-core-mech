// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa208: *mut i32;
    static mut DAT_100aa210: i32;
    static mut DAT_100aa20c: i32;
    static mut DAT_101eb518: i32;
    static mut DAT_100aa1f0: i32;
    static mut DAT_100aa1f4: i32;
    static mut DAT_100b3908: i32;
    static mut DAT_100adf98: i32;
    static mut DAT_100aa4a8: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8);
    fn FUN_10013a57();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100140da() {
    // SAFETY: the globals and callees are the game's own; DAT_100aa208 is
    // non-null when dereferenced, and the call target is computed as in the original.
    unsafe {
        if !DAT_100aa208.is_null() {
            DAT_100aa210 = DAT_100aa20c;
            DAT_101eb518 = 0;
            let vt = DAT_100aa208.cast::<*mut i32>().read_unaligned();
            let target = vt.wrapping_add(0x34) as usize;
            let f: unsafe extern "stdcall" fn(*mut i32, i32, u32, i32, i32) =
                core::mem::transmute(target);
            f(DAT_100aa208, 0, FUN_10013a57 as usize as u32, 0, 0x20);
        }
        FUN_10002168(&raw mut DAT_100aa4a8);
        if DAT_101eb518 == 1 {
            DAT_100aa1f0 = 2;
            DAT_100aa1f4 = 0;
            DAT_100b3908 = 0;
            DAT_100adf98 |= 1;
        } else if DAT_100aa210 == DAT_100aa20c {
            DAT_100aa1f4 = 1;
            DAT_100b3908 = 0;
        } else {
            DAT_100aa1f4 = 2;
            DAT_100b3908 = 2;
        }
    }
}
