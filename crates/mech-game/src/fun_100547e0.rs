// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100544ee(param_1: *mut i32, param_2: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100547e0(param_1: *mut i32, param_2: *mut i32) {
    if param_1 < param_2 {
        let mut local_c: *mut i32;
        let mut local_10: *mut i32 = param_2.wrapping_add(8);
        let mut pi_var3: *mut i32 = param_1;
        let mut pi_var2: *mut i32;
        // SAFETY: the pointers address the caller's array of 32-byte records;
        // the callee and reads/writes follow the original's order.
        unsafe {
            loop {
                loop {
                    local_c = pi_var3;
                    pi_var3 = local_c.wrapping_add(8);
                    let b = FUN_100544ee(param_1, pi_var3);
                    pi_var2 = local_10;
                    if b == 0 {
                        break;
                    }
                }
                loop {
                    local_10 = pi_var2;
                    pi_var2 = local_10.wrapping_sub(8);
                    let b = FUN_100544ee(pi_var2, param_1);
                    if b == 0 {
                        break;
                    }
                }
                if pi_var2 <= pi_var3 {
                    break;
                }
                let t = pi_var3.read_unaligned();
                pi_var3.write_unaligned(pi_var2.read_unaligned());
                pi_var2.write_unaligned(t);
                let a = local_c.wrapping_add(9);
                let b = local_10.wrapping_sub(7);
                let t = a.read_unaligned();
                a.write_unaligned(b.read_unaligned());
                b.write_unaligned(t);
                local_10 = pi_var2;
            }
            let t = param_1.read_unaligned();
            param_1.write_unaligned(pi_var2.read_unaligned());
            pi_var2.write_unaligned(t);
            let a = param_1.wrapping_add(1);
            let b = local_10.wrapping_sub(7);
            let t = a.read_unaligned();
            a.write_unaligned(b.read_unaligned());
            b.write_unaligned(t);
            if param_1 < local_10.wrapping_sub(0x10) {
                FUN_100547e0(param_1, local_10.wrapping_sub(0x10));
            }
            if local_10 < param_2 {
                FUN_100547e0(local_10, param_2);
            }
        }
    }
}
