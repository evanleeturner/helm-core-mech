// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adc20: u32;
    static mut DAT_100adc24: u32;
    static mut DAT_100c6b8c: u32;
    static mut DAT_100c6b90: u32;
    static mut DAT_10114e20: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1003517a(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32) -> u8;
    fn FUN_10016cdd(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10034a3b(_param_1: u32, _param_2: u32, param_3: i32, param_4: i32) {
    // SAFETY: the globals and callees are the game's own; DAT_10114e20 holds
    // a pointer to a structure whose field at +0xa4 is a pointer, read as the
    // original does.
    unsafe {
        let mut i_var3 = DAT_100adc20 as i32;
        let pi_var1 = ((DAT_10114e20.wrapping_add(0xa4)) as *const *mut i32).read_unaligned();
        let i_var2 = FUN_10003d60(DAT_100c6b8c as i32, param_3.wrapping_add(0x168) % 0x168);
        i_var3 = i_var3.wrapping_sub(i_var2);
        let mut local_8: i32;
        if i_var3 < 0 {
            local_8 = FUN_10003d60(DAT_100c6b8c as i32, 0x168);
        } else {
            local_8 = FUN_10003d60(DAT_100c6b8c as i32, 0x168);
            local_8 = local_8.wrapping_neg();
        }
        local_8 = i_var3.wrapping_add(local_8);
        FUN_1003517a(i_var3, DAT_100adc24 as i32, 0x19, pi_var1);
        FUN_1003517a(local_8, DAT_100adc24 as i32, 0x19, pi_var1);
        if param_4 != 0 {
            if param_4 < 0 {
                FUN_10016cdd(
                    pi_var1,
                    (DAT_100adc20 as i32).wrapping_add(param_4),
                    (DAT_100adc24 as i32).wrapping_sub(DAT_100c6b90 as i32),
                    param_4.wrapping_neg(),
                    DAT_100c6b90 as i32,
                    0xf,
                );
            } else {
                FUN_10016cdd(
                    pi_var1,
                    DAT_100adc20 as i32,
                    (DAT_100adc24 as i32).wrapping_sub(DAT_100c6b90 as i32),
                    param_4,
                    DAT_100c6b90 as i32,
                    0xf,
                );
            }
        }
        FUN_1003517a(DAT_100adc20 as i32, DAT_100adc24 as i32, 0x13, pi_var1);
    }
}
