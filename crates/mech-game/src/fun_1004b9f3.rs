// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38dc: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100ef030: u8;
    static mut DAT_100ef034: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000dd98(param_1: i32) -> u32;
    fn FUN_1001cac9(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u32;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1004bd53(
        param_1: u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004b9f3(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    // SAFETY: globals, arrays and callees are the game's own memory; the
    // entry table is walked exactly as the original does.
    unsafe {
        let mut local_10: i32 = DAT_100b38dc as i32;
        while local_10 != 0 {
            let i_var2 = local_10.wrapping_sub(1);
            local_10 = i_var2;
            let off = i_var2.wrapping_mul(0x40) as isize;
            let local_c = (&raw mut DAT_100ef030).offset(off).cast::<u16>();
            if (local_c.read_unaligned() & 4) == 0 {
                let p34 = (&raw mut DAT_100ef034).offset(off).cast::<i32>();
                let local_20 = FUN_1000dd98(p34.read_unaligned()) as i32;
                if local_20 != 0 {
                    let mut local_2c: u32 = 0;
                    let mut local_30: u32 = 0;
                    let mut local_34: u32 = 0;
                    let local_28 = FUN_1001cac9(
                        local_20,
                        &raw mut local_2c,
                        &raw mut local_30,
                        &raw mut local_34,
                    ) as i32;
                    let local_14 = (local_2c as i32).wrapping_sub(param_2);
                    let local_18 = (local_30 as i32).wrapping_sub(param_3);
                    let local_1c = (local_34 as i32).wrapping_sub(param_4);
                    let local_24 = param_5.wrapping_add(local_28);
                    let local_8 = FUN_1000fc30(local_14, local_18, local_1c);
                    if local_8 < local_24 {
                        let i_var1 = FUN_10003d60(param_6, DAT_100b3904 as i32);
                        FUN_1004bd53(
                            param_1,
                            local_20,
                            i_var1,
                            local_2c as i32,
                            local_30 as i32,
                            local_34 as i32,
                        );
                    }
                }
            }
        }
    }
}
