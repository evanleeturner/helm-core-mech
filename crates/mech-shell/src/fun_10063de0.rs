// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063de0(param_1: *mut i32) {
    mech_trace::enter!("10063de0", param_1);
    let mut b_var4 = false;
    if !param_1.is_null() {
        // SAFETY: param_1 points at an array of 0x38-byte records terminated by
        // a record whose first field is -1; reads and writes follow the original.
        unsafe {
            let mut i_var2 = mech_trace::rd!("10063de0", param_1, read_unaligned);
            let mut pi_var6 = param_1;
            let mut i_var5 = i_var2;
            while i_var5 != -1 {
                if mech_trace::rd!("10063de0", pi_var6.add(0xb), read_unaligned) == 2 {
                    b_var4 = true;
                }
                pi_var6 = pi_var6.add(0xe);
                i_var5 = mech_trace::rd!("10063de0", pi_var6, read_unaligned);
            }
            pi_var6 = param_1;
            if b_var4 {
                while i_var2 != -1 {
                    let old = mech_trace::rd!("10063de0", pi_var6.add(0xb), read_unaligned);
                    mech_trace::wr!("10063de0", pi_var6.add(0xb), write_unaligned, 1);
                    if old != 2 {
                        mech_trace::wr!("10063de0", pi_var6.add(0xb), write_unaligned, 0);
                    }
                    i_var2 = mech_trace::rd!("10063de0", pi_var6.add(0xe), read_unaligned);
                    pi_var6 = pi_var6.add(0xe);
                }
            }
            pi_var6 = param_1;
            if mech_trace::rd!("10063de0", param_1, read_unaligned) != -1 {
                loop {
                    let pv_var3 = mech_trace::rd!("10063de0", pi_var6.add(6), read_unaligned)
                        as usize as *mut c_void;
                    if !pv_var3.is_null() {
                        mech_trace::cr!(
                            "10063de0",
                            "10040680",
                            FUN_10040680(pv_var3 as usize as i32)
                        );
                        mech_trace::cr!("10063de0", "1006ec30", FUN_1006ec30(pv_var3));
                        mech_trace::wr!("10063de0", pi_var6.add(6), write_unaligned, 0);
                    }
                    let next = pi_var6.add(0xe);
                    pi_var6 = next;
                    if mech_trace::rd!("10063de0", next, read_unaligned) == -1 {
                        break;
                    }
                }
                let mut p = param_1;
                let mut i_var5 = mech_trace::rd!("10063de0", p, read_unaligned);
                while i_var5 != -1 {
                    let f = mech_trace::rd!(
                        "10063de0",
                        p.add(7)
                            .cast::<Option<unsafe extern "C" fn(*mut i32) -> i32>>(),
                        read_unaligned
                    );
                    if let Some(func) = f {
                        let r = mech_trace::cr!("10063de0", "ind", func(p));
                        mech_trace::wr!("10063de0", p.add(6), write_unaligned, r);
                    }
                    p = p.add(0xe);
                    i_var5 = mech_trace::rd!("10063de0", p, read_unaligned);
                }
            }
        }
    }
}
