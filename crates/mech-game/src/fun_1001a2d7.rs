// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf70: u32;
    static mut DAT_100ab074: u32;
    static mut DAT_100ab07c: u32;
    static mut DAT_100ab090: u32;
    static mut DAT_100ab094: u32;
    static mut DAT_100ab09c: u32;
    static mut DAT_100ab0a0: u32;
    static mut DAT_100ab318: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b38d8: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100ba9b0: u32;
    static mut DAT_100ba9b4: u32;
    static mut DAT_100ba9b8: u32;
    static mut DAT_100ba9bc: u32;
    static mut DAT_100ba9c0: u32;
    static mut DAT_100ba9c4: u32;
    static mut DAT_100ba9d4: u8;
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a635(param_1: i32);
    fn FUN_1001a5e1(param_1: i32);
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_1001aa9c(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1001b179(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001a620() -> u32;
    fn FUN_1001a9f9();
    fn FUN_1001ae8f();
    fn FUN_1001b0bb();
    fn FUN_10039454();
    fn FUN_1001b3e9();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001a2d7() {
    mech_trace::enter!("1001a2d7");
    // SAFETY: the globals and callees are the game's own; the table read
    // is indexed exactly as the original does.
    unsafe {
        let local_8: i32;
        mech_trace::gw!(
            "1001a2d7",
            "100ab074",
            DAT_100ab074,
            mech_trace::gr!("1001a2d7", "100ba9c4", DAT_100ba9c4)
        );
        FUN_1001a635(0);
        mech_trace::gw!("1001a2d7", "100ab090", DAT_100ab090, 0);
        mech_trace::gw!("1001a2d7", "100ab094", DAT_100ab094, 0);
        if mech_trace::gr!("1001a2d7", "100b38d8", DAT_100b38d8) == 0 {
            local_8 = 2;
            FUN_1001a5e1(2);
            mech_trace::gw!("1001a2d7", "100ab0a0", DAT_100ab0a0, 0);
        } else {
            let mut l = mech_trace::cr!("1001a2d7", "1001a620", FUN_1001a620()) as i32;
            if l == 0 {
                let p = (&raw mut DAT_100eef40).cast::<u32>();
                mech_trace::gw!(
                    "1001a2d7",
                    "100ab0a0",
                    DAT_100ab0a0,
                    mech_trace::rd!(
                        "1001a2d7",
                        p.offset(
                            mech_trace::gr!("1001a2d7", "100adf38", DAT_100adf38) as i32 as isize
                        ),
                        read_unaligned
                    )
                );
            }
            if mech_trace::gr!("1001a2d7", "100ab0a0", DAT_100ab0a0) == 0 {
                l = 2;
            }
            local_8 = l;
        }
        mech_trace::gw!("1001a2d7", "100aaf70", DAT_100aaf70, 0);
        match local_8 {
            0 | 6 => {
                FUN_1001ae8f();
            }
            1 => {
                let i_var2 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9bc", DAT_100ba9bc) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                let i_var3 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9c0", DAT_100ba9c0) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                let i_var1 = mech_trace::gr!("1001a2d7", "100ba9b0", DAT_100ba9b0) as i32;
                let mut local_20 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9b4", DAT_100ba9b4) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                if mech_trace::gr!("1001a2d7", "100ab318", DAT_100ab318) != 0
                    && mech_trace::gr!("1001a2d7", "100ab09c", DAT_100ab09c) == 0
                {
                    local_20 = (mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32)
                        .wrapping_shl(0xe);
                }
                FUN_1001aa9c(i_var2 >> 0x10, i_var3 >> 0x10, i_var1, local_20);
            }
            3 => {
                FUN_1001a9f9();
            }
            4 => {
                FUN_1001b0bb();
            }
            _ => {
                let i_var1 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9c0", DAT_100ba9c0) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                let i_var2 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9bc", DAT_100ba9bc) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                let i_var3 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9b8", DAT_100ba9b8) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                let i_var4 = mech_trace::cr!(
                    "1001a2d7",
                    "10011e60",
                    FUN_10011e60(
                        mech_trace::gr!("1001a2d7", "100ba9b4", DAT_100ba9b4) as i32,
                        mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32,
                        0xb5
                    )
                ) as i32;
                let b0 = mech_trace::gr!("1001a2d7", "100ba9b0", DAT_100ba9b0) as i32;
                let local_1c: i32;
                if b0 < 1 {
                    if b0 < 0 {
                        local_1c = (mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32)
                            .wrapping_mul(-0x2d0000)
                            / 0xb5;
                    } else {
                        local_1c = 0;
                    }
                } else {
                    local_1c = (mech_trace::gr!("1001a2d7", "100b3904", DAT_100b3904) as i32)
                        .wrapping_mul(0x2d0000)
                        / 0xb5;
                }
                mech_trace::gw!("1001a2d7", "100ba9d4", DAT_100ba9d4, 1);
                FUN_1001b179(
                    i_var1 >> 0xf,
                    i_var2 >> 0xf,
                    i_var3 >> 0x10,
                    i_var4,
                    local_1c,
                );
            }
        }
        mech_trace::gw!("1001a2d7", "100ab07c", DAT_100ab07c, local_8 as u32);
        FUN_10039454();
        FUN_1001b3e9();
    }
}
