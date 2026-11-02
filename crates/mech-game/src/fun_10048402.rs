// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_101686f0: u32;
    static mut DAT_101f52d4: u32;
    static mut DAT_101f52f4: u32;
    static mut PTR_DAT_100af0b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1005b547(param_1: i32, param_2: i32) -> u32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_100663e5(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_10065e40(param_1: u32, param_2: i32, param_3: i32) -> i32;
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10048402(param_1: *mut i32) {
    mech_trace::enter!("10048402", param_1);
    // SAFETY: param_1 points at the game's object; the globals and callees
    // are the game's own; pointers read from the object are used as the
    // original does.
    unsafe {
        let local_1c = mech_trace::rd!("10048402", param_1, read_unaligned) as *mut i32;
        if local_1c.is_null() {
            return;
        }
        let piVar1 = mech_trace::rd!("10048402", param_1.add(6), read_unaligned) as *mut i32;
        if piVar1.is_null() {
            return;
        }
        if mech_trace::rd!("10048402", local_1c, read_unaligned) == 0
            || mech_trace::rd!("10048402", piVar1, read_unaligned) == 0
        {
            return;
        }
        if mech_trace::rd!("10048402", param_1.add(4), read_unaligned) != -1 {
            let local_10 = mech_trace::cr!(
                "10048402",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("10048402", "100af17c", DAT_100af17c) as i32,
                    (mech_trace::rd!("10048402", param_1.add(4), read_unaligned) as u32)
                        .wrapping_add(mech_trace::gr!("10048402", "101686f0", DAT_101686f0)),
                    mech_trace::gr!("10048402", "100af0b4", PTR_DAT_100af0b4) as usize as *mut i32,
                )
            );
            if !local_10.is_null() {
                let local_18 = mech_trace::cr!(
                    "10048402",
                    "1005b547",
                    FUN_1005b547(local_10 as usize as i32, 0)
                );
                mech_trace::cr!(
                    "10048402",
                    "1008664f",
                    FUN_1008664f(
                        (mech_trace::rd!("10048402", param_1.add(4), read_unaligned) as u32)
                            .wrapping_add(mech_trace::gr!("10048402", "101686f0", DAT_101686f0)),
                        mech_trace::gr!("10048402", "100af0b4", PTR_DAT_100af0b4) as usize
                            as *mut i32,
                    )
                );
                let mut local_c: u32 = ((local_18 as i32) >> 0x10) as u32;
                let mut local_8: u32 = local_18 & 0xffff;
                local_c = mech_trace::cr!(
                    "10048402",
                    "10002570",
                    FUN_10002570(
                        local_c,
                        (mech_trace::gr!("10048402", "101f52f4", DAT_101f52f4) as i32)
                            .wrapping_add(1)
                    )
                );
                local_8 = mech_trace::cr!(
                    "10048402",
                    "10002570",
                    FUN_10002570(
                        local_8,
                        (mech_trace::gr!("10048402", "101f52d4", DAT_101f52d4) as i32)
                            .wrapping_add(1)
                    )
                );
                let p1 = mech_trace::rd!("10048402", piVar1, read_unaligned);
                let p3 = mech_trace::rd!("10048402", piVar1.add(3), read_unaligned);
                let p2 = mech_trace::rd!("10048402", piVar1.add(2), read_unaligned);
                let p4 = mech_trace::rd!("10048402", piVar1.add(4), read_unaligned);
                local_c = mech_trace::cr!(
                    "10048402",
                    "10002570",
                    FUN_10002570(local_c, p3.wrapping_sub(p1).wrapping_add(1))
                );
                local_8 = mech_trace::cr!(
                    "10048402",
                    "10002570",
                    FUN_10002570(local_8, p4.wrapping_sub(p2).wrapping_add(1))
                );
                mech_trace::cr!(
                    "10048402",
                    "100663e5",
                    FUN_100663e5(
                        piVar1 as usize as i32,
                        piVar1 as usize as i32,
                        local_c as i32,
                        local_8 as i32,
                    )
                );
            }
        }
        let mut local_24: i32 = 0;
        let local_20 = mech_trace::cr!(
            "10048402",
            "10002570",
            FUN_10002570(
                1,
                mech_trace::rd!("10048402", param_1.add(0xf), read_unaligned).wrapping_add(2)
            )
        );
        let iVar3 = mech_trace::cr!("10048402", "10002570", FUN_10002570(local_20, 0x20000)) as i32;
        mech_trace::wr!("10048402", param_1.add(0x13), write_unaligned, iVar3);
        mech_trace::wr!(
            "10048402",
            param_1.add(0x19),
            write_unaligned,
            local_20.wrapping_mul(2) as i32
        );
        mech_trace::wr!(
            "10048402",
            param_1.add(0x17),
            write_unaligned,
            mech_trace::rd!("10048402", param_1.add(0x19), read_unaligned)
        );
        mech_trace::wr!(
            "10048402",
            param_1.add(0x15),
            write_unaligned,
            mech_trace::rd!("10048402", param_1.add(0x17), read_unaligned)
        );
        mech_trace::cr!(
            "10048402",
            "10065e40",
            FUN_10065e40(
                mech_trace::rd!("10048402", local_1c, read_unaligned) as u32,
                local_1c as usize as i32,
                local_1c as usize as i32,
            )
        );
        mech_trace::cr!(
            "10048402",
            "10065e40",
            FUN_10065e40(
                mech_trace::rd!("10048402", piVar1, read_unaligned) as u32,
                piVar1 as usize as i32,
                piVar1 as usize as i32,
            )
        );
        let lc = local_1c as usize as i32;
        mech_trace::cr!(
            "10048402",
            "100660e1",
            FUN_100660e1(lc, &raw mut local_24, &raw mut local_24)
        );
        mech_trace::cr!(
            "10048402",
            "100660e1",
            FUN_100660e1(lc, param_1.add(0x12), param_1.add(0x12))
        );
        mech_trace::cr!(
            "10048402",
            "100660e1",
            FUN_100660e1(lc, param_1.add(0x14), param_1.add(0x14))
        );
        mech_trace::cr!(
            "10048402",
            "100660e1",
            FUN_100660e1(lc, param_1.add(0x16), param_1.add(0x16))
        );
        mech_trace::cr!(
            "10048402",
            "100660e1",
            FUN_100660e1(lc, param_1.add(0x18), param_1.add(0x18))
        );
        if (mech_trace::rd!("10048402", param_1.add(1).cast::<u8>(), read) & 0x10) != 0 {
            let iVar3 = mech_trace::rd!("10048402", piVar1.add(1), read_unaligned);
            let iVar2 = mech_trace::rd!("10048402", local_1c.add(1), read_unaligned);
            let d = iVar3.wrapping_sub(iVar2);
            mech_trace::wr!(
                "10048402",
                piVar1.add(1),
                write_unaligned,
                mech_trace::rd!("10048402", piVar1.add(1), read_unaligned).wrapping_sub(d)
            );
            mech_trace::wr!(
                "10048402",
                piVar1.add(3),
                write_unaligned,
                mech_trace::rd!("10048402", piVar1.add(3), read_unaligned).wrapping_sub(d)
            );
        }
        mech_trace::wr!("10048402", param_1.add(0x10), write_unaligned, local_24);
        mech_trace::wr!(
            "10048402",
            param_1.add(0x11),
            write_unaligned,
            local_20 as i32
        );
    }
}
