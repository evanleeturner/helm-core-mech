// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6748: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040640(param_1: i32);
    fn FUN_10040680(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut c_void);
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050b80(
        this: *mut c_void,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a560(param_1: *mut i8, param_2: i32) {
    mech_trace::enter!("1003a560", param_1, param_2);
    // SAFETY: param_1 points at the caller's object; fields are read and
    // written at the original's offsets, and the callees and global are the
    // game's own.
    unsafe {
        let a = mech_trace::rd!(
            "1003a560",
            param_1.byte_add(0x20d).cast::<i32>(),
            read_unaligned
        );
        if a != 0 {
            FUN_10040640(a);
        }
        let pv1 = mech_trace::rd!(
            "1003a560",
            param_1.byte_add(0x209).cast::<*mut c_void>(),
            read_unaligned
        );
        if !pv1.is_null() {
            FUN_10040680(pv1 as usize as i32);
            FUN_1006ec30(pv1);
        }
        let pi2 = mech_trace::cr!(
            "1003a560",
            "10050b80",
            FUN_10050b80(
                mech_trace::gr!("1003a560", "100a6748", DAT_100a6748),
                0x2a,
                param_2.wrapping_mul(0x23).wrapping_add(0x5c),
                param_1,
                0x100c49d0,
            )
        );
        mech_trace::wr!(
            "1003a560",
            param_1.byte_add(0x209).cast::<*mut i32>(),
            write_unaligned,
            pi2
        );
    }
}
