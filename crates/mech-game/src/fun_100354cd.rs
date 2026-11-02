// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10115154: i32;
    static mut DAT_10160ef0: i32;
    static mut DAT_10160f20: i32;
    static mut DAT_10160f24: i32;
    static mut DAT_10160f28: i32;
    static mut DAT_10160f2c: i32;
    static mut DAT_10160f30: i32;
    static mut DAT_10160f34: i32;
    static mut DAT_10160f38: i32;
    static mut DAT_10160f3c: i32;
    static mut DAT_10160f40: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f4c: i32;
    static mut DAT_10160f54: i32;
    static mut DAT_10168380: i32;
    static mut _DAT_10168384: i32;
    static mut DAT_10168388: i32;
    static mut DAT_1016838c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10035980(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100354cd(param_1: *mut u16) -> u32 {
    mech_trace::enter!("100354cd", param_1);
    // SAFETY: param_1 points at the caller's object; the globals and the
    // callee are the game's own, accessed in the original's order.
    unsafe {
        if (mech_trace::rd!("100354cd", param_1, read) & 0x1000) != 0 {
            return mech_trace::rt!("100354cd", 1);
        }
        let i_var1 = mech_trace::rd!(
            "100354cd",
            param_1.byte_add(0x40).cast::<i32>(),
            read_unaligned
        );
        let i_var3 = mech_trace::rd!(
            "100354cd",
            param_1.byte_add(0x34).cast::<i32>(),
            read_unaligned
        )
        .wrapping_sub(mech_trace::gr!("100354cd", "_DAT_10160f48", _DAT_10160f48));
        let i_var4 = mech_trace::rd!(
            "100354cd",
            param_1.byte_add(0x38).cast::<i32>(),
            read_unaligned
        )
        .wrapping_sub(mech_trace::gr!("100354cd", "_DAT_10160f44", _DAT_10160f44));
        let i_var5 = mech_trace::rd!(
            "100354cd",
            param_1.byte_add(0x3c).cast::<i32>(),
            read_unaligned
        )
        .wrapping_sub(mech_trace::gr!("100354cd", "_DAT_10160f4c", _DAT_10160f4c));
        mech_trace::gw!(
            "100354cd",
            "10115154",
            DAT_10115154,
            mech_trace::cr!(
                "100354cd",
                "10035980",
                FUN_10035980(
                    i_var3,
                    mech_trace::gr!("100354cd", "10160f38", DAT_10160f38),
                    i_var4,
                    mech_trace::gr!("100354cd", "10160f3c", DAT_10160f3c),
                    i_var5,
                    mech_trace::gr!("100354cd", "10160f40", DAT_10160f40),
                )
            )
        );
        if mech_trace::gr!("100354cd", "10115154", DAT_10115154).wrapping_add(i_var1)
            < mech_trace::gr!("100354cd", "10160f54", DAT_10160f54)
        {
            return mech_trace::rt!("100354cd", 4);
        }
        if mech_trace::gr!("100354cd", "10160ef0", DAT_10160ef0)
            < mech_trace::gr!("100354cd", "10115154", DAT_10115154).wrapping_sub(i_var1)
        {
            return mech_trace::rt!("100354cd", 5);
        }
        let mut local_14 = mech_trace::cr!(
            "100354cd",
            "10035980",
            FUN_10035980(
                i_var3,
                mech_trace::gr!("100354cd", "10160f20", DAT_10160f20),
                i_var4,
                mech_trace::gr!("100354cd", "10160f24", DAT_10160f24),
                i_var5,
                mech_trace::gr!("100354cd", "10160f28", DAT_10160f28),
            )
        );
        if local_14 < 1 {
            local_14 = mech_trace::gr!("100354cd", "10168380", DAT_10168380).wrapping_sub(local_14);
        } else {
            local_14 =
                local_14.wrapping_sub(mech_trace::gr!("100354cd", "_DAT_10168384", _DAT_10168384));
        }
        if i_var1 < local_14 {
            return mech_trace::rt!("100354cd", 6);
        }
        local_14 = mech_trace::cr!(
            "100354cd",
            "10035980",
            FUN_10035980(
                i_var3,
                mech_trace::gr!("100354cd", "10160f2c", DAT_10160f2c),
                i_var4,
                mech_trace::gr!("100354cd", "10160f30", DAT_10160f30),
                i_var5,
                mech_trace::gr!("100354cd", "10160f34", DAT_10160f34),
            )
        );
        if local_14 < 1 {
            local_14 = mech_trace::gr!("100354cd", "1016838c", DAT_1016838c).wrapping_sub(local_14);
        } else {
            local_14 = local_14.wrapping_sub(mech_trace::gr!("100354cd", "10168388", DAT_10168388));
        }
        if i_var1 < local_14 {
            return mech_trace::rt!("100354cd", 7);
        }
        mech_trace::rt!("100354cd", 0)
    }
}
