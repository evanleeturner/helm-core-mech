// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef030: u8;
    static mut DAT_100ef034: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000dde1(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32) -> u8;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10006bbb(param_1: u32) -> i32;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004a6cc(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("1004a6cc", param_1, param_2, param_3, param_4);
    let p = param_1 as *mut u8;
    let mut local_34: i32;
    let mut local_8: i32;
    let mut local_c: i32;
    // SAFETY: param_1 points at the game's object; the globals, tables and
    // callees are the game's own memory and code, accessed as the original does.
    unsafe {
        if mech_trace::rd!("1004a6cc", p.byte_add(0x34).cast::<i32>(), read_unaligned) < 0 {
            return;
        }
        if mech_trace::rd!("1004a6cc", p.byte_add(0x38).cast::<i32>(), read_unaligned) == 0x200 {
            let idx = mech_trace::rd!("1004a6cc", p.byte_add(0x34).cast::<i32>(), read_unaligned);
            let local_10 = mech_trace::rd!(
                "1004a6cc",
                (&raw const DAT_100eef40).cast::<u32>().offset(idx as isize),
                read_unaligned
            ) as i32;
            let q = local_10 as *mut u8;
            let f = mech_trace::rd!("1004a6cc", q.byte_add(0x14).cast::<u16>(), read_unaligned);
            if (f & 2) != 0 || (f & 4) != 0 {
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x20).cast::<i32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x1c).cast::<i32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x18).cast::<i32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x38).cast::<i32>(),
                    write_unaligned,
                    0
                );
                return;
            }
            local_34 = mech_trace::rd!("1004a6cc", q.byte_add(0x50).cast::<i32>(), read_unaligned);
            local_8 = mech_trace::rd!("1004a6cc", q.byte_add(0x54).cast::<i32>(), read_unaligned);
            local_c = mech_trace::rd!("1004a6cc", q.byte_add(0x58).cast::<i32>(), read_unaligned);
        } else {
            if mech_trace::rd!("1004a6cc", p.byte_add(0x38).cast::<i32>(), read_unaligned) != 0x400
            {
                return;
            }
            let off = mech_trace::rd!("1004a6cc", p.byte_add(0x34).cast::<i32>(), read_unaligned)
                .wrapping_mul(0x40) as isize;
            let l20 = (&raw mut DAT_100ef030).offset(off).cast::<u16>();
            if (mech_trace::rd!("1004a6cc", l20, read_unaligned) & 4) != 0 {
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x20).cast::<i32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x1c).cast::<i32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x18).cast::<i32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "1004a6cc",
                    p.byte_add(0x38).cast::<i32>(),
                    write_unaligned,
                    0
                );
                return;
            }
            let arg = mech_trace::rd!(
                "1004a6cc",
                (&raw mut DAT_100ef034).offset(off).cast::<i32>(),
                read_unaligned
            );
            local_34 = 0;
            local_8 = 0;
            local_c = 0;
            mech_trace::cr!(
                "1004a6cc",
                "1000dde1",
                FUN_1000dde1(
                    arg,
                    &raw mut local_34 as *mut u32,
                    &raw mut local_8 as *mut u32,
                    &raw mut local_c as *mut u32,
                )
            );
        }
        let mut local_18: u32 = local_34.wrapping_sub(param_2) as u32;
        let mut local_14: u32 = local_8.wrapping_sub(param_3) as u32;
        let mut local_1c: u32 = local_c.wrapping_sub(param_4) as u32;
        let local_24 = mech_trace::cr!(
            "1004a6cc",
            "1000fc30",
            FUN_1000fc30(local_18 as i32, local_14 as i32, local_1c as i32)
        );
        if local_24 < 0x65 {
            let v = mech_trace::rd!("1004a6cc", p.byte_add(0x3c).cast::<u32>(), read_unaligned);
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x3c).cast::<u32>(),
                write_unaligned,
                v | 0x8000
            );
        }
        local_18 = mech_trace::cr!("1004a6cc", "10002570", FUN_10002570(local_18, local_24));
        local_14 = mech_trace::cr!("1004a6cc", "10002570", FUN_10002570(local_14, local_24));
        local_1c = mech_trace::cr!("1004a6cc", "10002570", FUN_10002570(local_1c, local_24));
        let mut local_28: u32 =
            mech_trace::rd!("1004a6cc", p.byte_add(0xc).cast::<u32>(), read_unaligned);
        let mut local_2c: u32 =
            mech_trace::rd!("1004a6cc", p.byte_add(0x10).cast::<u32>(), read_unaligned);
        let mut local_30: u32 =
            mech_trace::rd!("1004a6cc", p.byte_add(0x14).cast::<u32>(), read_unaligned);
        mech_trace::cr!(
            "1004a6cc",
            "1000fef4",
            FUN_1000fef4(&raw mut local_28, &raw mut local_2c, &raw mut local_30)
        );
        if (mech_trace::rd!("1004a6cc", p.byte_add(4), read) & 0x10) == 0 {
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x18).cast::<u32>(),
                write_unaligned,
                local_18.wrapping_sub(local_28)
            );
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x1c).cast::<u32>(),
                write_unaligned,
                local_14.wrapping_sub(local_2c)
            );
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x20).cast::<u32>(),
                write_unaligned,
                local_1c.wrapping_sub(local_30)
            );
        } else {
            let s = mech_trace::cr!(
                "1004a6cc",
                "10002570",
                FUN_10002570(mech_trace::gr!("1004a6cc", "100b3904", DAT_100b3904), 0x2d4)
            ) as i32;
            let c18 = mech_trace::rd!("1004a6cc", p.byte_add(0x18).cast::<i32>(), read_unaligned);
            let a = mech_trace::cr!(
                "1004a6cc",
                "10003d60",
                FUN_10003d60(
                    local_18.wrapping_sub(local_28).wrapping_sub(c18 as u32) as i32,
                    s,
                )
            );
            let c1c = mech_trace::rd!("1004a6cc", p.byte_add(0x1c).cast::<i32>(), read_unaligned);
            let b = mech_trace::cr!(
                "1004a6cc",
                "10003d60",
                FUN_10003d60(
                    local_14.wrapping_sub(local_2c).wrapping_sub(c1c as u32) as i32,
                    s,
                )
            );
            let c20 = mech_trace::rd!("1004a6cc", p.byte_add(0x20).cast::<i32>(), read_unaligned);
            let c = mech_trace::cr!(
                "1004a6cc",
                "10003d60",
                FUN_10003d60(
                    local_1c.wrapping_sub(local_30).wrapping_sub(c20 as u32) as i32,
                    s,
                )
            );
            let v18 = mech_trace::rd!("1004a6cc", p.byte_add(0x18).cast::<i32>(), read_unaligned);
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x18).cast::<i32>(),
                write_unaligned,
                v18.wrapping_add(a)
            );
            let v1c = mech_trace::rd!("1004a6cc", p.byte_add(0x1c).cast::<i32>(), read_unaligned);
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x1c).cast::<i32>(),
                write_unaligned,
                v1c.wrapping_add(b)
            );
            let v20 = mech_trace::rd!("1004a6cc", p.byte_add(0x20).cast::<i32>(), read_unaligned);
            mech_trace::wr!(
                "1004a6cc",
                p.byte_add(0x20).cast::<i32>(),
                write_unaligned,
                v20.wrapping_add(c)
            );
        }
        local_8 = mech_trace::cr!("1004a6cc", "10006d3e", FUN_10006d3e());
        local_34 = mech_trace::cr!(
            "1004a6cc",
            "10006bbb",
            FUN_10006bbb(local_2c.wrapping_shl(0xd))
        );
        local_34 = local_34.wrapping_neg();
        mech_trace::cr!(
            "1004a6cc",
            "1002f668",
            FUN_1002f668(
                mech_trace::rd!("1004a6cc", p.byte_add(0x30).cast::<i32>(), read_unaligned),
                local_34,
                local_8,
                0,
                0,
            )
        );
    }
}
