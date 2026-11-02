// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: [i8; 0];
    static mut DAT_100eef40: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(a: i32) -> i32;
    fn FUN_10034b8e(a: *mut i32, b: *mut i32, c: *mut i32) -> i32;
    fn FUN_1003ba89(a: *mut i32, b: *mut i32, c: *mut i32) -> i32;
    fn FUN_1002a132(a: i32) -> i32;
    fn FUN_10044bba(a: i32, b: i32) -> i32;
    fn FUN_10044761(a: i32, b: i16) -> u32;
    fn FUN_10002570(a: u32, b: i32) -> u32;
    fn FUN_100444ea(a: i32, b: *mut i32, c: i32, d: i16, e: i32, f: i32);
    fn FUN_10014768(a: *mut i32, b: *mut i32, c: u32) -> i32;
}

unsafe fn rd<T>(addr: i32) -> T {
    // SAFETY: caller supplies a valid game address.
    unsafe { mech_trace::rd!("10044cc4", (addr as usize as *const T), read_unaligned) }
}

unsafe fn wr<T>(addr: i32, v: T) {
    // SAFETY: caller supplies a valid game address.
    unsafe { mech_trace::wr!("10044cc4", (addr as usize as *mut T), write_unaligned, v) }
}

unsafe fn tbl(i: u32) -> i32 {
    // SAFETY: the game's own table, indexed as the original does.
    unsafe {
        mech_trace::rd!(
            "10044cc4",
            (&raw mut DAT_100eef40).cast::<i32>().add(i as usize),
            read
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044cc4(param_1: i32, param_2: *mut i32) {
    mech_trace::enter!("10044cc4", param_1, param_2);
    // SAFETY: all pointers and addresses are the game's own memory,
    // accessed exactly where the original accesses them.
    unsafe {
        let mut local_1c: u32 = 0;
        let mut local_18: i32 = 0;
        let mut local_78: [i32; 14] = [0; 14];
        let mut local_3c: i32 = 0;
        let mut local_30: i32 = 0;
        let mut local_38: i32 = 0;
        let mut local_24: i32;
        let mut local_28: i32;
        let mut local_34: i32;

        let base = mech_trace::rd!("10044cc4", param_2, read);
        let mut flags: u32 = rd::<u32>(base.wrapping_add(0x1f2));
        if ((flags >> 2) & 1) == 0 {
            return;
        }
        if mech_trace::cr!("10044cc4", "10051f43", FUN_10051f43(3)) == 0 {
            return;
        }
        let kind = rd::<i32>(param_1.wrapping_add(4));
        let t = mech_trace::rd!(
            "10044cc4",
            (&raw mut DAT_100a7bc8)
                .cast::<u8>()
                .byte_add(kind.wrapping_mul(0x68) as usize)
                .cast::<i32>(),
            read_unaligned
        );
        if !(t != 0 || kind == 0x15) {
            return;
        }
        let base = mech_trace::rd!("10044cc4", param_2, read);
        if rd::<i32>(base.wrapping_add(0x10)) == 2 {
            local_1c = rd::<u32>(base.wrapping_add(0xdc));
        } else if (rd::<u16>(base.wrapping_add(0x17a)) & 0x200) == 0 {
            local_1c = rd::<u32>(base.wrapping_add(0xdc));
        } else {
            local_1c = rd::<u16>(base.wrapping_add(0x17a)) as u32;
        }
        let local_14 = local_1c & 0xf00;
        let local_c = local_1c & 0xff;
        if !(local_c != 0 && local_14 == 0x200) {
            return;
        }
        if rd::<i32>(tbl(local_c).wrapping_add(0x1c8)) == 4 {
            return;
        }
        if rd::<i32>(mech_trace::rd!("10044cc4", param_2, read).wrapping_add(0x10)) == 2 {
            local_18 = tbl(local_c);
        }
        if local_18 == 0
            && mech_trace::cr!(
                "10044cc4",
                "10034b8e",
                FUN_10034b8e(param_2, &raw mut local_30, &raw mut local_38)
            ) != 0
        {
            local_24 = rd::<i32>(tbl(local_c).wrapping_add(0x50));
            local_28 = rd::<i32>(tbl(local_c).wrapping_add(0x54));
            local_34 = rd::<i32>(tbl(local_c).wrapping_add(0x58));
            if mech_trace::cr!(
                "10044cc4",
                "1003ba89",
                FUN_1003ba89(&raw mut local_24, &raw mut local_28, &raw mut local_34)
            ) != 0
            {
                let local_2c: i32 = 0x30000;
                let local_8: i32 = 0xf;
                let b = mech_trace::rd!("10044cc4", param_2, read);
                let sum = (rd::<i32>(b.wrapping_add(0xd8)) as u32).wrapping_add(mech_trace::rd!(
                    "10044cc4",
                    param_2.add(7),
                    read
                )
                    as u32);
                let local_10 = (sum as i32) % 0x1680000;
                let i3 = mech_trace::cr!(
                    "10044cc4",
                    "1002a132",
                    FUN_1002a132(mech_trace::rd!("10044cc4", param_2, read))
                );
                let local_20 = (i3 >> 0x10) % 0x168;
                if local_10 < local_2c
                    && (0u32.wrapping_sub(local_2c as u32) as i32) < local_10
                    && local_20 < local_8
                    && (0u32.wrapping_sub(local_8 as u32) as i32) < local_20
                {
                    local_18 = tbl(local_c);
                }
            }
        }
        if local_18 != 0
            && mech_trace::cr!("10044cc4", "10044bba", FUN_10044bba(local_18, 0x41)) != 0
        {
            if mech_trace::cr!("10044cc4", "10051f43", FUN_10051f43(3)) != 0 {
                let local_40 = mech_trace::cr!(
                    "10044cc4",
                    "10044761",
                    FUN_10044761(
                        local_18,
                        rd::<i32>(mech_trace::rd!("10044cc4", param_2, read).wrapping_add(4))
                            as i16
                    )
                );
                let u4 = mech_trace::cr!("10044cc4", "10002570", FUN_10002570(local_40, 0x5a0000));
                let mut local_7c: i16 =
                    if mech_trace::cr!("10044cc4", "10051f43", FUN_10051f43(2)) == 0 {
                        1
                    } else {
                        -1
                    };
                let i3 = ((u4 >> 0x10) as i16 as i32).wrapping_add(1);
                let u1 = (i3 >> 0x1f) as u16;
                let x = ((i3 as u16 ^ u1).wrapping_sub(u1)) & 3;
                let s2 = (x ^ u1).wrapping_sub(u1).wrapping_mul(4) as i16;
                let mut local_80: i16 = 0;
                while local_80 < 2 {
                    FUN_100444ea(
                        local_18,
                        local_78.as_mut_ptr(),
                        local_7c as i32,
                        s2,
                        5000,
                        0,
                    );
                    let r = mech_trace::cr!(
                        "10044cc4",
                        "10014768",
                        FUN_10014768(
                            local_78.as_mut_ptr(),
                            &raw mut local_3c,
                            rd::<u32>(local_18.wrapping_add(4)),
                        )
                    );
                    if r == 0 {
                        wr::<i32>(local_18.wrapping_add(0x1c8), 0xb);
                        wr::<i32>(local_18.wrapping_add(0x1d4), s2 as i32);
                        break;
                    }
                    local_7c = 0u32.wrapping_sub(local_7c as i32 as u32) as i16;
                    local_80 = local_80.wrapping_add(1);
                }
            }
            flags = rd::<u32>(local_18.wrapping_add(0x1f2));
            if rd::<i32>(local_18.wrapping_add(0x1c8)) == 0 && ((flags >> 1) & 1) != 0 {
                wr::<i32>(local_18.wrapping_add(0x1c8), 4);
            }
        }
    }
}
