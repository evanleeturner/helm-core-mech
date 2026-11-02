// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003dc4(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003ea2(param_1: i32, param_2: *mut i32) -> *mut u32 {
    // SAFETY: param_2 and the structures it points to belong to the caller and
    // the game; the globals and callees are the game's own.
    unsafe {
        let pi_var1 = param_2.read_unaligned() as *mut i32;
        let pi_var2 = param_2.add(1).read_unaligned() as *mut i32;
        let mut local_1c: *mut u32 = core::ptr::null_mut();
        let mut local_10: i32 = pi_var1.read_unaligned();
        if local_10 == 1 {
            local_1c = pi_var2.add(3).read_unaligned() as *mut u32;
            let mut local_14: *mut u32;
            let mut local_20: *mut u32;
            if param_1 == 0 {
                local_14 = pi_var2.add(1).read_unaligned() as *mut u32;
                local_20 = pi_var2.add(2).read_unaligned() as *mut u32;
            } else {
                local_14 = pi_var2.add(2).read_unaligned() as *mut u32;
                local_20 = pi_var2.add(1).read_unaligned() as *mut u32;
            }
            if pi_var1.add(1).read_unaligned() == 0 {
                let mut p = local_1c;
                let mut i = 5;
                while i != 0 {
                    p.write_unaligned(local_14.read_unaligned());
                    local_14 = local_14.add(1);
                    p = p.add(1);
                    i -= 1;
                }
                pi_var1.add(2).write_unaligned(0);
            } else {
                let cur = pi_var1.add(2).read_unaligned() as u32;
                let sum = cur.wrapping_add(DAT_100b3904 as u32);
                pi_var1.add(2).write_unaligned(sum as i32);
                let i_var3 =
                    FUN_10002570(pi_var1.add(2).read_unaligned(), pi_var2.read_unaligned());
                if i_var3 < 0x10000 {
                    FUN_10003dc4(local_14 as i32, local_20 as i32, local_1c as i32, i_var3, 0);
                } else {
                    let mut p = local_1c;
                    let mut i = 5;
                    while i != 0 {
                        p.write_unaligned(local_20.read_unaligned());
                        local_20 = local_20.add(1);
                        p = p.add(1);
                        i -= 1;
                    }
                    pi_var1.add(2).write_unaligned(0);
                    local_10 = 0;
                }
            }
        }
        let first = pi_var1.read_unaligned();
        pi_var1.add(1).write_unaligned(first);
        pi_var1.write_unaligned(local_10);
        local_1c
    }
}
