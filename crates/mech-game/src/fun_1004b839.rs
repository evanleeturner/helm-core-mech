// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1f4: u32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_100065c4(param_1: i32, param_2: *mut i32, param_3: i32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004b839(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    mech_trace::enter!(
        "1004b839", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: globals and callees are the game's own; the object pointers
    // come from the game's table and are read and written as the original does.
    unsafe {
        let mut local_14: i32 = mech_trace::gr!("1004b839", "100b38d8", DAT_100b38d8) as i32;
        loop {
            let i_var3 = local_14.wrapping_sub(1);
            if local_14 == 0 {
                break;
            }
            local_14 = i_var3;
            if mech_trace::gr!("1004b839", "100aa1f4", DAT_100aa1f4) == 0
                || i_var3 == mech_trace::gr!("1004b839", "100adf38", DAT_100adf38)
            {
                let obj = mech_trace::rd!(
                    "1004b839",
                    (&raw mut DAT_100eef40).cast::<u32>().add(i_var3 as usize),
                    read_unaligned
                ) as i32;
                let p_var1 = mech_trace::rd!(
                    "1004b839",
                    (obj.wrapping_add(0x20) as *const i32),
                    read_unaligned
                ) as *mut i32;
                let flags =
                    mech_trace::rd!("1004b839", p_var1.add(0x110).cast::<u16>(), read_unaligned);
                if flags & 0x100 == 0 {
                    let i_var2 = mech_trace::rd!("1004b839", p_var1.add(0x107), read_unaligned);
                    let x = mech_trace::rd!(
                        "1004b839",
                        (obj.wrapping_add(0x50) as *const i32),
                        read_unaligned
                    );
                    let y = mech_trace::rd!(
                        "1004b839",
                        (obj.wrapping_add(0x54) as *const i32),
                        read_unaligned
                    );
                    let z = mech_trace::rd!(
                        "1004b839",
                        (obj.wrapping_add(0x58) as *const i32),
                        read_unaligned
                    );
                    let len = mech_trace::cr!(
                        "1004b839",
                        "1000fc30",
                        FUN_1000fc30(
                            x.wrapping_sub(param_2),
                            y.wrapping_sub(param_3),
                            z.wrapping_sub(param_4),
                        )
                    );
                    if len < i_var2.wrapping_add(param_5) {
                        let mut local_24 = param_6.wrapping_mul(mech_trace::gr!(
                            "1004b839",
                            "100b3904",
                            DAT_100b3904
                        ) as i32);
                        let i_var3b = param_7.wrapping_mul(mech_trace::gr!(
                            "1004b839",
                            "100b3904",
                            DAT_100b3904
                        ) as i32);
                        if mech_trace::rd!("1004b839", p_var1.add(0xf5), read_unaligned) == 4 {
                            local_24 = local_24.wrapping_mul(2);
                        }
                        FUN_100065c4(param_1, p_var1, local_24 >> 2, 1);
                        FUN_100065c4(param_1, p_var1, local_24, 2);
                        FUN_100065c4(param_1, p_var1, local_24, 3);
                        FUN_100065c4(param_1, p_var1, local_24, 4);
                        FUN_100065c4(param_1, p_var1, local_24, 5);
                        FUN_100065c4(param_1, p_var1, local_24, 6);
                        FUN_100065c4(param_1, p_var1, local_24, 7);
                        FUN_100065c4(param_1, p_var1, local_24, 8);
                        let f2 = p_var1.add(0xf2);
                        mech_trace::wr!(
                            "1004b839",
                            f2,
                            write_unaligned,
                            mech_trace::rd!("1004b839", f2, read_unaligned).wrapping_add(i_var3b)
                        );
                    }
                }
            }
        }
    }
}
