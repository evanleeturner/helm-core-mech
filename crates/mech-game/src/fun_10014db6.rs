// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa540: u32;
    static mut DAT_100aa544: u32;
    static mut DAT_100aa548: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10015b30(param_1: i32, param_2: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10015009(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u32;
    fn FUN_100266c9(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014db6(param_1: i32, param_2: i32, param_3: *mut i32) -> u32 {
    mech_trace::enter!("10014db6", param_1, param_2, param_3);
    // SAFETY: pointers are the caller's game structures; the callees and
    // globals are the game's own, accessed in the original's order.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "10014db6",
            ((param_1.wrapping_add(0x14)) as *const i32),
            read_unaligned
        );
        let i_var2 = mech_trace::rd!(
            "10014db6",
            ((param_1.wrapping_add(0x18)) as *const i32),
            read_unaligned
        );
        let i_var3 = mech_trace::rd!(
            "10014db6",
            ((param_1.wrapping_add(0x1c)) as *const i32),
            read_unaligned
        );
        let i_var4 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var1,
                mech_trace::rd!("10014db6", param_3.add(9), read_unaligned)
            )
        );
        let i_var5 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var2,
                mech_trace::rd!("10014db6", param_3.add(10), read_unaligned)
            )
        );
        let i_var6 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var3,
                mech_trace::rd!("10014db6", param_3.add(0xb), read_unaligned)
            )
        );
        let i_var6 = i_var4.wrapping_add(i_var5).wrapping_add(i_var6);
        if i_var6 >= 0 {
            return mech_trace::rt!("10014db6", 0);
        }
        let off = mech_trace::rd!(
            "10014db6",
            ((param_1.wrapping_add(4)) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!("10014db6", ((off.wrapping_add(param_1)) as *const u8), read);
        let rec = (b as i32).wrapping_mul(0x44).wrapping_add(param_2);
        let i_var5 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var1,
                mech_trace::rd!(
                    "10014db6",
                    ((rec.wrapping_add(0xc)) as *const i32),
                    read_unaligned
                ),
            )
        );
        let i_var8 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var2,
                mech_trace::rd!(
                    "10014db6",
                    ((rec.wrapping_add(0x10)) as *const i32),
                    read_unaligned
                ),
            )
        );
        let i_var4 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var3,
                mech_trace::rd!(
                    "10014db6",
                    ((rec.wrapping_add(0x14)) as *const i32),
                    read_unaligned
                ),
            )
        );
        let i_var9 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var2,
                mech_trace::rd!("10014db6", param_3.add(1), read_unaligned)
            )
        );
        let i_var10 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(i_var1, mech_trace::rd!("10014db6", param_3, read_unaligned))
        );
        let i_var11 = mech_trace::cr!(
            "10014db6",
            "10015b30",
            FUN_10015b30(
                i_var3,
                mech_trace::rd!("10014db6", param_3.add(2), read_unaligned)
            )
        );
        let i_var4 = i_var9
            .wrapping_add(i_var10)
            .wrapping_add(i_var11)
            .wrapping_sub(i_var5.wrapping_add(i_var8).wrapping_add(i_var4));
        if i_var4 < 1 {
            return mech_trace::rt!("10014db6", 0);
        }
        if (i_var4 / i_var6).wrapping_neg() >= 0x8000 {
            return mech_trace::rt!("10014db6", 0);
        }
        let i_var4 = mech_trace::cr!(
            "10014db6",
            "10002570",
            FUN_10002570(i_var4.wrapping_neg() as u32, i_var6)
        ) as i32;
        if i_var4 < 1 || mech_trace::rd!("10014db6", param_3.add(0xc), read_unaligned) < i_var4 {
            return mech_trace::rt!("10014db6", 0);
        }
        let i_var8 = mech_trace::cr!(
            "10014db6",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10014db6", param_3.add(9), read_unaligned),
                i_var4
            )
        );
        let i_var5 = mech_trace::rd!("10014db6", param_3, read_unaligned);
        let i_var9 = mech_trace::cr!(
            "10014db6",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10014db6", param_3.add(10), read_unaligned),
                i_var4
            )
        );
        let i_var6 = mech_trace::rd!("10014db6", param_3.add(1), read_unaligned);
        let i_var10 = mech_trace::cr!(
            "10014db6",
            "10003d60",
            FUN_10003d60(
                mech_trace::rd!("10014db6", param_3.add(0xb), read_unaligned),
                i_var4
            )
        );
        let i_var4 = mech_trace::rd!("10014db6", param_3.add(2), read_unaligned);
        let x = i_var5.wrapping_add(i_var8);
        let y = i_var6.wrapping_add(i_var9);
        let z = i_var4.wrapping_add(i_var10);
        let i_var11 = mech_trace::cr!(
            "10014db6",
            "10015009",
            FUN_10015009(param_1, param_2, x, y, z)
        );
        if i_var11 == 0 {
            return mech_trace::rt!("10014db6", 0);
        }
        FUN_100266c9(param_3, x, y, z);
        mech_trace::gw!("10014db6", "100aa540", DAT_100aa540, (i_var1 >> 0xd) as u32);
        mech_trace::gw!("10014db6", "100aa544", DAT_100aa544, (i_var2 >> 0xd) as u32);
        mech_trace::gw!("10014db6", "100aa548", DAT_100aa548, (i_var3 >> 0xd) as u32);
        mech_trace::rt!("10014db6", 1)
    }
}
