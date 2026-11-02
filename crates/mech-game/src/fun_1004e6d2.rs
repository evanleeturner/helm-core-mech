// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1012327a: u8;
    static mut DAT_1012327c: u8;
    static mut DAT_10123280: u8;
    static mut DAT_10123281: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004e5e3(param_1: i32, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004e6d2(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: the globals are the game's own tables, indexed exactly as the
    // original does; the callee is the game's own function.
    unsafe {
        let i_var2 = param_1
            .wrapping_mul(0x3c0a)
            .wrapping_add(param_2.wrapping_mul(0x13f));
        let c_var1 = (&raw mut DAT_10123280).byte_offset(i_var2 as isize).read() as i8;
        let mut local_10: u32 = 1;
        let t = (&raw mut DAT_1012327a).byte_offset(i_var2 as isize).read();
        if t == 0x05 || t == 0x06 {
            return 0;
        }
        let v = (&raw mut DAT_1012327c)
            .byte_offset(i_var2 as isize)
            .cast::<i32>()
            .read_unaligned();
        if v == 0x10 {
            return 1;
        }
        let mut local_14: i32 = 0;
        while local_14 < 8
            && (&raw mut DAT_10123281)
                .byte_offset(i_var2.wrapping_add(local_14.wrapping_mul(0xc)) as isize)
                .cast::<i32>()
                .read_unaligned()
                != 0
        {
            let u_var4 = FUN_1004e5e3(param_1, param_2, local_14);
            if c_var1 == 0 {
                if u_var4 != 0 {
                    return 1;
                }
            } else {
                local_10 &= u_var4;
                if local_10 == 0 {
                    return 0;
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
        if c_var1 == 0 { 0 } else { 1 }
    }
}
