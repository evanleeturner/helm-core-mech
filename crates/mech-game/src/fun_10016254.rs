// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100adf38: i32;
    static mut DAT_100aa568: i32;
    static mut DAT_100aa56c: i32;
    static mut DAT_10114e28: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10067d51(param_1: *mut i32) -> i32;
    fn FUN_10002570(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1001608d(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016254(param_1: i32) {
    // SAFETY: all pointers and globals are the game's own memory; reads and
    // calls happen in the original's order.
    unsafe {
        let pi_var1 =
            ((param_1 as u32).wrapping_add(0xa4) as *const i32).read_unaligned() as *mut i32;
        let mut local_14: i32 = 0xe;
        let mut i_var3 = ((param_1 as u32).wrapping_add(0x8c) as *const i32).read_unaligned();
        let idx = DAT_100adf38;
        let entry = (&raw mut DAT_100eef40)
            .cast::<i32>()
            .offset(idx as isize)
            .read_unaligned();
        let inner = ((entry as u32).wrapping_add(0x20) as *const i32).read_unaligned();
        let v = ((inner as u32).wrapping_add(0x3cc) as *const i32).read_unaligned();
        DAT_100aa56c = FUN_10011e60(v, i_var3, 100);
        let i_var2 = FUN_10067d51(&raw mut DAT_100aa568);

        let w = (pi_var1.add(3).read_unaligned() as u32)
            .wrapping_sub(pi_var1.add(1).read_unaligned() as u32)
            .wrapping_add(1) as i32
            >> 1;
        let h = (pi_var1.add(4).read_unaligned() as u32)
            .wrapping_sub(pi_var1.add(2).read_unaligned() as u32)
            .wrapping_add(1) as i32
            >> 1;
        let w2 = (pi_var1.add(3).read_unaligned() as u32)
            .wrapping_sub(pi_var1.add(1).read_unaligned() as u32)
            .wrapping_add(1) as i32
            >> 1;
        FUN_10056489(pi_var1, w, 0, w2, h, 0, 2);

        let local_18: i32;
        if (i_var2 >> 0x10) < i_var3 {
            i_var3 = FUN_10002570(
                ((i_var2 >> 0x10) as u32).wrapping_shl(0x10) as i32,
                (i_var3 as u32).wrapping_shl(0x10) as i32,
            );
            let m = FUN_10003d60(i_var3, 0x1680000);
            local_18 = m >> 0x10;
        } else {
            local_18 = 0x163;
        }
        if local_18 != 0 {
            let mut local_2c: i32 = 0;
            while local_2c < local_18 {
                if local_2c < 0x79 || 0x10d < local_2c {
                    if 0x10d < local_2c {
                        local_14 = 10;
                    }
                } else {
                    local_14 = 3;
                }
                let a = ((local_2c.wrapping_add(0x10e) % 0x168) as u32).wrapping_shl(0x10) as i32;
                let b = (((DAT_10114e28 as u32).wrapping_add(0xa4)) as *const i32).read_unaligned();
                FUN_1001608d(pi_var1, a, local_14, b);
                local_2c = local_2c.wrapping_add(0xf);
            }
            if local_18 < 0x79 || 0x10d < local_18 {
                if 0x10d < local_18 {
                    local_14 = 10;
                }
            } else {
                local_14 = 3;
            }
            let a = ((local_18.wrapping_add(0x10e) % 0x168) as u32).wrapping_shl(0x10) as i32;
            let b = (((DAT_10114e28 as u32).wrapping_add(0xa4)) as *const i32).read_unaligned();
            FUN_1001608d(pi_var1, a, local_14, b);
        }
    }
}
