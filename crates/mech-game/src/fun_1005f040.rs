// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10113170: u8;
    static mut DAT_100f3170: u8;
    static mut DAT_101f52e0: u8;
    static mut DAT_100af17c: i32;
    static mut DAT_100b351c: i32;
    static mut DAT_100b352c: *mut u16;
    static mut PTR_DAT_100af0ac: *mut u8;
    static mut PTR_DAT_100af100: *mut u8;
    static mut DAT_10114d74: i16;
    static mut DAT_10114d78: i16;
    static mut DAT_10114d70: *mut u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_1000c3a0(
        param_1: *mut u8,
        param_2: *mut u8,
        param_3: u16,
        param_4: u16,
        param_5: i32,
        param_6: *mut u32,
        param_7: i32,
        param_8: *mut u16,
        param_9: i32,
    );
    fn FUN_10040f45(param_1: *mut u32);
    fn FUN_10040f65(
        param_1: *mut i32,
        param_2: i32,
        param_3: *mut u32,
        param_4: *mut u32,
        param_5: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f040(
    param_1: i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) -> u32 {
    mech_trace::enter!(
        "1005f040", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    let mut param_1 = param_1;
    let mut local_8: i32 = 2;
    if param_2 < 3 {
        return mech_trace::rt!("1005f040", 0);
    }
    // SAFETY: globals and callees are the game's own; the tables and param_3
    // are read and written at the offsets the original uses.
    unsafe {
        let t113170 = (&raw mut DAT_10113170) as *mut u8;
        let t0f3170 = (&raw mut DAT_100f3170) as *mut u8;
        let buf = (&raw mut DAT_101f52e0) as *mut u8;

        if param_5 == 0 {
            param_1 = (param_1 as u32).wrapping_add(0x100) as i32;
        }
        let i_var2 = (param_1 as u32).wrapping_mul(0xe) as i32 as isize;
        if mech_trace::rd!(
            "1005f040",
            t113170.byte_offset(6 + i_var2).cast::<i16>(),
            read_unaligned
        ) == 0
            || mech_trace::rd!(
                "1005f040",
                t113170.byte_offset(8 + i_var2).cast::<i16>(),
                read_unaligned
            ) < 0
        {
            return mech_trace::rt!("1005f040", 0);
        }
        let a = mech_trace::rd!(
            "1005f040",
            t113170.byte_offset(i_var2).cast::<i16>(),
            read_unaligned
        ) as i32 as u32;
        let b = mech_trace::rd!(
            "1005f040",
            t113170.byte_offset(2 + i_var2).cast::<i16>(),
            read_unaligned
        ) as i32 as u32;
        let i_var1 = a.wrapping_mul(0x100).wrapping_add(b.wrapping_mul(8)) as i32 as isize;
        if mech_trace::rd!(
            "1005f040",
            t0f3170.byte_offset(i_var1).cast::<i16>(),
            read_unaligned
        ) < 1
        {
            return mech_trace::rt!("1005f040", 0);
        }
        let p2 = t0f3170.byte_offset(2 + i_var1).cast::<i16>();
        mech_trace::wr!(
            "1005f040",
            p2,
            write_unaligned,
            mech_trace::rd!("1005f040", p2, read_unaligned).wrapping_add(1)
        );
        let mut local_18 = mech_trace::rd!(
            "1005f040",
            t0f3170.byte_offset(4 + i_var1).cast::<*mut u16>(),
            read_unaligned
        );
        if local_18.is_null() {
            local_18 = mech_trace::cr!(
                "1005f040",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1005f040", "100af17c", DAT_100af17c),
                    mech_trace::rd!(
                        "1005f040",
                        t0f3170.byte_offset(i_var1).cast::<i16>(),
                        read_unaligned
                    ) as i32,
                    mech_trace::gr!("1005f040", "100af0ac", PTR_DAT_100af0ac) as *mut i32,
                )
            );
            if local_18.is_null() {
                return mech_trace::rt!("1005f040", 0);
            }
            mech_trace::wr!(
                "1005f040",
                t0f3170.byte_offset(4 + i_var1).cast::<*mut u16>(),
                write_unaligned,
                local_18
            );
        }
        let u_var3 = mech_trace::rd!("1005f040", local_18, read_unaligned);
        let u_var4 = mech_trace::rd!("1005f040", local_18.add(1), read_unaligned);
        local_18 = local_18.add(2);
        if param_5 != 0 {
            let m3 = (u_var3.wrapping_sub(1) as i16) as i32 as u32;
            let m4 = (u_var4.wrapping_sub(1) as i16) as i32 as u32;
            let mut i: i32 = 0;
            while i < 4 {
                let p = param_3.offset((i * 6 + 3) as isize);
                mech_trace::wr!(
                    "1005f040",
                    p,
                    write_unaligned,
                    mech_trace::rd!("1005f040", p, read_unaligned).wrapping_mul(m3)
                );
                let q = param_3.offset((i * 6 + 4) as isize);
                mech_trace::wr!(
                    "1005f040",
                    q,
                    write_unaligned,
                    mech_trace::rd!("1005f040", q, read_unaligned).wrapping_mul(m4)
                );
                i += 1;
            }
        }
        let flags = mech_trace::rd!(
            "1005f040",
            t113170.byte_offset(8 + i_var2).cast::<u16>(),
            read_unaligned
        );
        let local_20: i32 = if (flags & 4) == 0 && -1 < param_4 && param_4 < 0xf {
            1
        } else {
            0
        };
        if mech_trace::gr!("1005f040", "100b352c", DAT_100b352c).is_null() {
            mech_trace::gw!(
                "1005f040",
                "100b352c",
                DAT_100b352c,
                mech_trace::cr!(
                    "1005f040",
                    "1008668b",
                    FUN_1008668b(
                        mech_trace::gr!("1005f040", "100af17c", DAT_100af17c),
                        mech_trace::gr!("1005f040", "100b351c", DAT_100b351c),
                        mech_trace::gr!("1005f040", "100af100", PTR_DAT_100af100) as *mut i32
                    )
                )
            );
        }
        let off = (param_4 as u32).wrapping_mul(0x80) as i32 as isize;
        if param_6 == 0 {
            if local_20 != 0 {
                FUN_10040f45(
                    mech_trace::gr!("1005f040", "100b352c", DAT_100b352c).wrapping_offset(off)
                        as *mut u32,
                );
                local_8 = 3;
            }
            mech_trace::gw!(
                "1005f040",
                "10114d74",
                DAT_10114d74,
                (u_var3 as i16).wrapping_add(-1)
            );
            mech_trace::gw!(
                "1005f040",
                "10114d78",
                DAT_10114d78,
                (u_var4 as i16).wrapping_add(-1)
            );
            mech_trace::gw!("1005f040", "10114d70", DAT_10114d70, local_18);
            if param_2 != 3 {
                FUN_10040f65(
                    buf as *mut i32,
                    param_2,
                    param_3,
                    (&raw mut DAT_10114d70) as *mut u32,
                    local_8,
                );
            }
        } else {
            FUN_1000c3a0(
                buf,
                local_18 as *mut u8,
                u_var3,
                u_var4,
                param_2,
                param_3,
                local_20,
                mech_trace::gr!("1005f040", "100b352c", DAT_100b352c).wrapping_offset(off),
                param_7,
            );
        }
        mech_trace::rt!("1005f040", 1)
    }
}
