// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6748: *mut c_void;
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

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a4d0(param_1: *mut i8, param_2: u32, param_3: u32, param_4: i32) {
    mech_trace::enter!("1003a4d0", param_1, param_2, param_3, param_4);
    let _ = (param_2, param_3);
    // SAFETY: param_1 is the caller's NUL-terminated object with fields at
    // 0x209 and 0x20d; the callees and global are the game's own.
    unsafe {
        let mut i_var4: i32 = -1;
        let mut pc_var5: *const i8 = param_1;
        loop {
            if i_var4 == 0 {
                break;
            }
            i_var4 = i_var4.wrapping_sub(1);
            let c_var1 = mech_trace::rd!("1003a4d0", pc_var5, read);
            pc_var5 = pc_var5.add(1);
            if c_var1 == 0 {
                break;
            }
        }
        if i_var4 != -2 {
            let mut pv_var2 = mech_trace::rd!(
                "1003a4d0",
                param_1.byte_add(0x20d).cast::<*mut c_void>(),
                read_unaligned
            );
            if !pv_var2.is_null() {
                mech_trace::cr!("1003a4d0", "10040680", FUN_10040680(pv_var2 as i32));
                mech_trace::cr!("1003a4d0", "1006ec30", FUN_1006ec30(pv_var2));
            }
            mech_trace::wr!("1003a4d0", param_1.byte_add(0x20d), write, 0);
            mech_trace::wr!("1003a4d0", param_1.byte_add(0x20e), write, 0);
            mech_trace::wr!("1003a4d0", param_1.byte_add(0x20f), write, 0);
            mech_trace::wr!("1003a4d0", param_1.byte_add(0x210), write, 0);
            pv_var2 = mech_trace::rd!(
                "1003a4d0",
                param_1.byte_add(0x209).cast::<*mut c_void>(),
                read_unaligned
            );
            if !pv_var2.is_null() {
                mech_trace::cr!("1003a4d0", "10040680", FUN_10040680(pv_var2 as i32));
                mech_trace::cr!("1003a4d0", "1006ec30", FUN_1006ec30(pv_var2));
            }
            let pi_var3 = mech_trace::cr!(
                "1003a4d0",
                "10050b80",
                FUN_10050b80(
                    mech_trace::gr!("1003a4d0", "100a6748", DAT_100a6748),
                    0x2a,
                    param_4.wrapping_mul(0x23).wrapping_add(0x5c),
                    param_1,
                    0x100c48d0,
                )
            );
            mech_trace::wr!(
                "1003a4d0",
                param_1.byte_add(0x209).cast::<*mut i32>(),
                write_unaligned,
                pi_var3
            );
        }
    }
}
