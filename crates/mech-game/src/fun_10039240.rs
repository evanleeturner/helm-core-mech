// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae744: u32;
    static mut DAT_100ae748: u32;
    static mut DAT_100ae74c: u32;
    static mut DAT_100ae750: u32;
    static mut DAT_100ae754: u32;
    static mut DAT_100ae758: u32;
    static mut DAT_100c6ce0: u32;
    static mut DAT_100c6ce4: u32;
    static mut DAT_100c6ce8: u32;
    static mut DAT_100c6cf0: u32;
    static mut DAT_100c6cf4: u32;
    static mut DAT_100c6cf8: u32;
    static mut DAT_100c6cfc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002f7f7(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10039240(param_1: i32) {
    if param_1 == 0 {
        return;
    }
    // SAFETY: param_1 and the structures it links to are the game's own
    // memory, read where the original reads; globals and callees are the
    // game's own.
    unsafe {
        let p6c = ((param_1 as u32).wrapping_add(0x6c) as *const i32).read_unaligned();
        if p6c == 0 {
            return;
        }
        let i_var1 = ((p6c as u32).wrapping_add(0x1c) as *const i32).read_unaligned();
        if i_var1 == 0 {
            return;
        }
        let mut local_18 = (i_var1 as u32).wrapping_add(0x18) as *const i32;
        let mut local_14: i32 = 0x7fffffff;
        let mut local_c: i32 = 0x7fffffff;
        let mut local_28: i32 = -0x7fffffff;
        let mut local_24: i32 = -0x7fffffff;
        let mut local_20: i32 =
            ((i_var1 as u32).wrapping_add(4) as *const i16).read_unaligned() as i32;
        while local_20 != 0 {
            let a = local_18.read_unaligned();
            let b = local_18.add(2).read_unaligned();
            if local_24 < a {
                local_24 = a;
            }
            if local_28 < b {
                local_28 = b;
            }
            if a < local_c {
                local_c = a;
            }
            if b < local_14 {
                local_14 = b;
            }
            local_18 = local_18.wrapping_add(0x11);
            local_20 = local_20.wrapping_sub(1);
        }
        let mut local_1c: i32 = local_24.wrapping_sub(local_c);
        if local_24.wrapping_sub(local_c) < local_28.wrapping_sub(local_14) {
            local_1c = local_28.wrapping_sub(local_14);
        }
        if 0 < local_1c {
            let b4 = ((param_1 as u32).wrapping_add(4) as *const i32).read_unaligned();
            let b_var2 = b4 != 0;
            if b_var2 {
                local_1c = local_1c.wrapping_add(((local_1c >> 0x1f) as u32 & 0x3fff) as i32);
            } else {
                let q = local_1c / 3;
                local_1c = q.wrapping_add(((q >> 0x1f) as u32 & 0x3fff) as i32);
            }
            DAT_100ae744 = (((local_1c >> 0xe).wrapping_add(1)) as u32).wrapping_mul(0x4000);
            DAT_100ae754 = b_var2 as u32;
            DAT_100c6cfc = ((DAT_100ae744 >> 3).wrapping_add(DAT_100ae744)) >> 1;
            DAT_100ae758 = param_1 as u32;
            DAT_100ae748 = 1;
            DAT_100ae750 = 1;
            DAT_100ae74c = 0;
            DAT_100c6cf8 = 0;
            DAT_100c6cf4 = 0;
            DAT_100c6cf0 = 0;
            DAT_100c6ce8 = 0;
            DAT_100c6ce4 = 0;
            DAT_100c6ce0 = 0;
            FUN_1002f727(param_1);
            FUN_1002f7f7(param_1);
        }
    }
}
