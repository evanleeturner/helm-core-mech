// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5aa0: *mut i32;
    static mut DAT_100a5aa4: i32;
    static mut DAT_100a5af4: [i8; 0];
    static mut DAT_100a5b9c: [i8; 0];
    static mut DAT_100a6748: *mut c_void;
    static mut DAT_100a5b3c: [i8; 0];
    static mut DAT_100a5b80: [i8; 0];
    static mut DAT_100a5af8: [i8; 0];
    static mut DAT_100a5b10: [i8; 0];
    static mut DAT_100a5b68: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut c_void) -> u8;
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

unsafe fn copy_str(src: *const i8, dst: *mut i8) {
    // SAFETY: src is a NUL-terminated string; dst is the caller's buffer.
    unsafe {
        let mut n: usize = 0;
        while mech_trace::dr!("1005bc00", src.add(n)) != 0 {
            n += 1;
        }
        n += 1;
        let mut i = 0;
        while i < n {
            mech_trace::dw!(
                "1005bc00",
                dst.add(i),
                mech_trace::dr!("1005bc00", src.add(i))
            );
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005bc00(param_1: i32) {
    mech_trace::enter!("1005bc00", param_1);
    let mut local_100: [i8; 256] = [0; 256];
    // SAFETY: globals and callees are the game's own; strings are NUL-terminated
    // and copied into the local buffer as the original does.
    unsafe {
        let pi_var2 = mech_trace::gr!("1005bc00", "100a5aa0", DAT_100a5aa0);
        if mech_trace::gr!("1005bc00", "100a5aa4", DAT_100a5aa4) != param_1 {
            mech_trace::gw!("1005bc00", "100a5aa4", DAT_100a5aa4, param_1);
            if !mech_trace::gr!("1005bc00", "100a5aa0", DAT_100a5aa0).is_null() {
                mech_trace::cr!(
                    "1005bc00",
                    "10040680",
                    FUN_10040680(mech_trace::gr!("1005bc00", "100a5aa0", DAT_100a5aa0) as i32)
                );
                mech_trace::cr!("1005bc00", "1006ec30", FUN_1006ec30(pi_var2 as *mut c_void));
                mech_trace::gw!("1005bc00", "100a5aa0", DAT_100a5aa0, core::ptr::null_mut());
            }
            copy_str((&raw mut DAT_100a5af4).cast::<i8>(), local_100.as_mut_ptr());
            let src: *const i8 = match param_1 {
                0 | 5 => (&raw mut DAT_100a5b3c).cast::<i8>(),
                1 | 6 => (&raw mut DAT_100a5b10).cast::<i8>(),
                2 => (&raw mut DAT_100a5b68).cast::<i8>(),
                3 | 7 => (&raw mut DAT_100a5af8).cast::<i8>(),
                4 => (&raw mut DAT_100a5b80).cast::<i8>(),
                _ => (&raw mut DAT_100a5b9c).cast::<i8>(),
            };
            copy_str(src, local_100.as_mut_ptr());
            mech_trace::gw!(
                "1005bc00",
                "100a5aa0",
                DAT_100a5aa0,
                mech_trace::cr!(
                    "1005bc00",
                    "10050b80",
                    FUN_10050b80(
                        mech_trace::gr!("1005bc00", "100a6748", DAT_100a6748),
                        0x1d,
                        0x1ba,
                        local_100.as_mut_ptr(),
                        0x100c48d0,
                    )
                )
            );
        }
    }
}
