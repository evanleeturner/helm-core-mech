// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100cf0d0: u8;
    static mut DAT_100cf0d4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f28d(param_1: i32) -> u32;
    fn FUN_1001cac9(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u32;
    fn FUN_1003c190(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1007c493(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007ca5c(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    mech_trace::enter!("1007ca5c", param_1, param_2, param_3, param_4, param_5);
    let mut local_20: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0x80;
    // SAFETY: the globals and callees are the game's own; the table at
    // 0x100cf0d0 has 0x80 entries of 0x14 bytes, read where the original does.
    unsafe {
        while local_10 != 0 {
            let i_var2 = local_10.wrapping_sub(1);
            let i_var1 = i_var2.wrapping_mul(0x14) as isize;
            local_10 = i_var2;
            let a = mech_trace::rd!(
                "1007ca5c",
                (&raw mut DAT_100cf0d0).byte_offset(i_var1).cast::<i32>(),
                read_unaligned
            );
            if a != 0 {
                let b = mech_trace::rd!(
                    "1007ca5c",
                    (&raw mut DAT_100cf0d4).byte_offset(i_var1).cast::<i32>(),
                    read_unaligned
                );
                if b != 0 {
                    let i_var2 = mech_trace::cr!("1007ca5c", "1002f28d", FUN_1002f28d(b)) as i32;
                    let local_c = mech_trace::cr!(
                        "1007ca5c",
                        "1001cac9",
                        FUN_1001cac9(
                            i_var2,
                            (&raw mut local_14).cast::<u32>(),
                            (&raw mut local_18).cast::<u32>(),
                            (&raw mut local_20).cast::<u32>(),
                        )
                    ) as i32;
                    let local_1c = local_14.wrapping_sub(param_1);
                    let local_8 = local_c.wrapping_add(param_4);
                    let r = mech_trace::cr!(
                        "1007ca5c",
                        "1003c190",
                        FUN_1003c190(
                            local_1c,
                            local_18.wrapping_sub(param_2),
                            local_20.wrapping_sub(param_3),
                            local_8,
                        )
                    );
                    if r != 0 {
                        let m = mech_trace::cr!(
                            "1007ca5c",
                            "10003d60",
                            FUN_10003d60(
                                param_5,
                                mech_trace::gr!("1007ca5c", "100b3904", DAT_100b3904) as i32
                            )
                        );
                        mech_trace::cr!("1007ca5c", "1007c493", FUN_1007c493(local_10, m));
                    }
                }
            }
        }
    }
}
