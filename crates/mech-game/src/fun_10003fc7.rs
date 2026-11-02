// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003dc4(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003fc7(param_1: i32, param_2: *mut i32) -> *mut u32 {
    // SAFETY: all pointers come from the game's own structures; reads, writes
    // and calls happen in the original's order.
    unsafe {
        let pi_var1 = param_2.read_unaligned() as *mut i32;
        let pi_var2 = param_2.add(1).read_unaligned() as *mut i32;
        let mut local_24: *mut u32 = core::ptr::null_mut();
        let mut local_10: i32 = pi_var1.read_unaligned();
        if local_10 == 1 {
            local_24 = pi_var2.add(3).read_unaligned() as *mut u32;
            let mut local_14: *mut u32;
            let mut local_28: *mut u32;
            let local_20: i32;
            let local_1c: i32;
            if param_1 == 0 {
                local_14 = pi_var2.add(1).read_unaligned() as *mut u32;
                local_28 = pi_var2.add(2).read_unaligned() as *mut u32;
                local_20 = 1;
                local_1c = 2;
            } else {
                local_14 = pi_var2.add(2).read_unaligned() as *mut u32;
                local_28 = pi_var2.add(1).read_unaligned() as *mut u32;
                local_20 = 2;
                local_1c = 1;
            }
            if pi_var1.add(1).read_unaligned() == 0 {
                let mut p5 = local_24;
                let mut i = 5;
                while i != 0 {
                    p5.write_unaligned(local_14.read_unaligned());
                    local_14 = local_14.add(1);
                    p5 = p5.add(1);
                    i -= 1;
                }
                pi_var1.add(2).write_unaligned(0);
            } else {
                let v = pi_var1.add(2).read_unaligned().wrapping_add(DAT_100b3904);
                pi_var1.add(2).write_unaligned(v);
                let mut i_var3 =
                    FUN_10002570(pi_var1.add(2).read_unaligned(), pi_var2.read_unaligned());
                if i_var3 < 0x10000 {
                    if i_var3 < 0x8001 {
                        let mut p5 = local_14;
                        let mut p6 = local_24;
                        let mut i = 5;
                        while i != 0 {
                            p6.write_unaligned(p5.read_unaligned());
                            p5 = p5.add(1);
                            p6 = p6.add(1);
                            i -= 1;
                        }
                        i_var3 = FUN_10003d60(i_var3, 0x20000);
                        FUN_10003dc4(
                            local_14 as i32,
                            local_28 as i32,
                            local_24 as i32,
                            i_var3,
                            local_20,
                        );
                    } else {
                        let mut p5 = local_28;
                        let mut p6 = local_24;
                        let mut i = 5;
                        while i != 0 {
                            p6.write_unaligned(p5.read_unaligned());
                            p5 = p5.add(1);
                            p6 = p6.add(1);
                            i -= 1;
                        }
                        i_var3 = FUN_10003d60(i_var3.wrapping_sub(0x8000), 0x20000);
                        FUN_10003dc4(
                            local_14 as i32,
                            local_28 as i32,
                            local_24 as i32,
                            i_var3,
                            local_1c,
                        );
                    }
                } else {
                    let mut p5 = local_24;
                    let mut i = 5;
                    while i != 0 {
                        p5.write_unaligned(local_28.read_unaligned());
                        local_28 = local_28.add(1);
                        p5 = p5.add(1);
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
        local_24
    }
}
