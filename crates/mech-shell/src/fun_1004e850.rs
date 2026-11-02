// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100bac8d: u8;
    static mut DAT_100a6798: u32;
    static mut DAT_100d08e8: u32;
    static mut DAT_1009bd18: i8;
    static mut DAT_1009bd20: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(param_1: *mut i8) -> *mut u8;
    fn FUN_1005b1b0(param_1: *mut i8);
    fn FUN_10038710(param_1: *mut i8) -> u32;
}

unsafe extern "stdcall" {
    fn PostMessageA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004e850() -> u32 {
    mech_trace::enter!("1004e850");
    // SAFETY: globals and callees are the game's own; all pointers are
    // derived from the game's state structure and read/written where the
    // original does.
    unsafe {
        let base = mech_trace::gr!("1004e850", "10083508", DAT_10083508);
        let field_23bb = mech_trace::rd!(
            "1004e850",
            ((base.wrapping_add(0x23bb)) as *const i32),
            read_unaligned
        );
        if field_23bb != 1 {
            return mech_trace::rt!("1004e850", base & 0xffffff00);
        }

        if mech_trace::gr!("1004e850", "100bac8d", DAT_100bac8d) == 4 {
            return mech_trace::rt!("1004e850", base & 0xffffff00);
        }

        let field_1fe = mech_trace::rd!(
            "1004e850",
            ((mech_trace::gr!("1004e850", "10083508", DAT_10083508).wrapping_add(0x1fe))
                as *const i32),
            read_unaligned
        );
        if field_1fe == 4 {
            return mech_trace::rt!("1004e850", base & 0xffffff00);
        }

        let p = mech_trace::gr!("1004e850", "100a6798", DAT_100a6798);
        if mech_trace::rd!(
            "1004e850",
            ((p.wrapping_add(4)) as *const i32),
            read_unaligned
        ) != 0
        {
            return mech_trace::rt!("1004e850", p & 0xffffff00);
        }

        let mut pb = mech_trace::cr!(
            "1004e850",
            "1005bbb0",
            FUN_1005bbb0(
                mech_trace::gr!("1004e850", "10083508", DAT_10083508).wrapping_add(0x1d6)
                    as *mut i8
            )
        ) as *const u8;
        let mut pc = &raw mut DAT_1009bd18 as *const i8;
        let mut i_var3: i32 = 0;
        loop {
            let mut b = mech_trace::dr!("1004e850", pb);
            let c = mech_trace::dr!("1004e850", pc);
            if (b as i32) != (c as i32) {
                let lt = (b as i32) < (c as u8 as i32);
                i_var3 = (1 - lt as i32) - (lt as i32);
                break;
            }
            if b == 0 {
                break;
            }
            b = mech_trace::dr!("1004e850", pb.add(1));
            let c1 = mech_trace::dr!("1004e850", pc.add(1));
            if (b as i32) != (c1 as i32) {
                let lt = (b as i32) < (c1 as u8 as i32);
                i_var3 = (1 - lt as i32) - (lt as i32);
                break;
            }
            pb = pb.add(2);
            pc = pc.add(2);
            if b == 0 {
                break;
            }
        }

        if i_var3 == 0 {
            let r = mech_trace::cr!(
                "1004e850",
                "PostMessageA",
                PostMessageA(
                    mech_trace::gr!("1004e850", "100d08e8", DAT_100d08e8),
                    0xc02b,
                    0xc00a,
                    0
                )
            );
            return mech_trace::rt!("1004e850", ((r as u32) & 0xffffff00) | 1);
        }

        FUN_1005b1b0(&raw mut DAT_1009bd20);
        mech_trace::cr!(
            "1004e850",
            "PostMessageA",
            PostMessageA(
                mech_trace::gr!("1004e850", "100d08e8", DAT_100d08e8),
                0xc01c,
                0xc00a,
                0
            )
        );
        let u_var5 = mech_trace::cr!(
            "1004e850",
            "10038710",
            FUN_10038710(
                mech_trace::gr!("1004e850", "10083508", DAT_10083508).wrapping_add(0x1d6)
                    as *mut i8
            )
        );
        mech_trace::dw!(
            "1004e850",
            (mech_trace::gr!("1004e850", "10083508", DAT_10083508).wrapping_add(0x239d) as *mut u8),
            1
        );
        mech_trace::dw!(
            "1004e850",
            (mech_trace::gr!("1004e850", "10083508", DAT_10083508).wrapping_add(0x239c) as *mut u8),
            0
        );
        mech_trace::dw!(
            "1004e850",
            (mech_trace::gr!("1004e850", "10083508", DAT_10083508).wrapping_add(0x23a2) as *mut u8),
            0
        );
        mech_trace::rt!("1004e850", (u_var5 & 0xffffff00) | 1)
    }
}
