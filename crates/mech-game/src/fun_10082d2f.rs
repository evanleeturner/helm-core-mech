// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100b9f68: u32;
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_1002f28d(param_1: i32) -> u32;
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10082d2f(param_1: i32, param_2: i32, param_3: u32) -> u32 {
    mech_trace::enter!("10082d2f", param_1, param_2, param_3);
    let mut local_28: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_20: u32 = 0;
    let mut local_18: i32 = 0;

    if param_2 < 0 {
        return mech_trace::rt!("10082d2f", 0xffffffff);
    }
    // SAFETY: globals are the game's own memory.
    if param_2 >= unsafe { mech_trace::gr!("10082d2f", "100b38d8", DAT_100b38d8) } {
        return mech_trace::rt!("10082d2f", 0xfffffffe);
    }
    if (param_3 & 0x15) != 0 {
        return mech_trace::rt!("10082d2f", 0xfffffff7);
    }
    // SAFETY: the game's tables and objects are read where the original reads
    // them; pointers come from the game's own object table.
    unsafe {
        let tbl = (&raw mut DAT_100eef40).cast::<u32>();
        let local_1c =
            mech_trace::rd!("10082d2f", tbl.offset(param_1 as isize), read_unaligned) as usize;
        let p = mech_trace::rd!("10082d2f", tbl.offset(param_2 as isize), read_unaligned) as usize
            as *const i32;
        let flags = mech_trace::rd!("10082d2f", p.add(5).cast::<u16>(), read_unaligned) as u32;
        let rd = |q: usize, off: usize| -> i32 {
            mech_trace::rd!("10082d2f", ((q + off) as *const i32), read_unaligned)
        };

        if mech_trace::gr!("10082d2f", "100b9f68", DAT_100b9f68) == 0
            && (flags & 0x10) != 0
            && param_1 == mech_trace::gr!("10082d2f", "100adf38", DAT_100adf38)
        {
            return mech_trace::rt!("10082d2f", 0xfffffffd);
        }
        if (flags & 6) != 0 {
            return mech_trace::rt!("10082d2f", 0xfffffffd);
        }
        if mech_trace::rd!("10082d2f", p.add(1), read_unaligned) == param_1 {
            return mech_trace::rt!("10082d2f", 0xfffffffd);
        }
        if (param_3 & 0x10000) != 0 && (flags & 0x40) == 0 {
            return mech_trace::rt!("10082d2f", 0xfffffffc);
        }
        if (param_3 & 0x20000) != 0
            && mech_trace::cr!("10082d2f", "1004d5e3", FUN_1004d5e3(param_2)) != 0
        {
            return mech_trace::rt!("10082d2f", 0xfffffff7);
        }
        if (param_3 & 0x40000) != 0
            && mech_trace::cr!("10082d2f", "1004d5e3", FUN_1004d5e3(param_2)) != 1
        {
            return mech_trace::rt!("10082d2f", 0xfffffff7);
        }
        if mech_trace::gr!("10082d2f", "100b9f68", DAT_100b9f68) == 0
            && (flags & 0x1400) == 0
            && param_1 == mech_trace::gr!("10082d2f", "100adf38", DAT_100adf38)
        {
            return mech_trace::rt!("10082d2f", 0xfffffffb);
        }
        if (flags & 0x800) != 0 && param_1 == mech_trace::gr!("10082d2f", "100adf38", DAT_100adf38)
        {
            return mech_trace::rt!("10082d2f", 0xfffffffb);
        }
        let v10 = mech_trace::rd!("10082d2f", p.add(0x10), read_unaligned);
        if v10 == 0 {
            return mech_trace::rt!("10082d2f", 0xfffffff8);
        }
        if mech_trace::cr!("10082d2f", "1002f28d", FUN_1002f28d(v10)) == 0 {
            return mech_trace::rt!("10082d2f", 0xfffffff8);
        }
        let i_var2 = mech_trace::rd!("10082d2f", p.add(0x14), read_unaligned);
        let mut local_8 = mech_trace::rd!("10082d2f", p.add(0x15), read_unaligned);
        let local_10 = mech_trace::rd!("10082d2f", p.add(0x16), read_unaligned);
        if rd(local_1c, 0x1bc) == 1 && mech_trace::rd!("10082d2f", p, read_unaligned) == 1 {
            let q = mech_trace::rd!("10082d2f", p.add(8), read_unaligned) as usize;
            local_8 = local_8.wrapping_sub(rd(q, 0x400));
        }
        let local_c = local_10.wrapping_sub(rd(local_1c, 0x58)) as u32;
        mech_trace::cr!(
            "10082d2f",
            "10083a59",
            FUN_10083a59(
                i_var2.wrapping_sub(rd(local_1c, 0x50)) as u32,
                local_8.wrapping_sub(rd(local_1c, 0x54)) as u32,
                local_c,
                &raw mut local_24,
                (&raw mut local_28).cast::<u32>(),
                &raw mut local_20,
                &raw mut local_18,
            )
        );
        if (flags & 0x1000) == 0
            && local_28 > 0x2ab98
            && param_1 == mech_trace::gr!("10082d2f", "100adf38", DAT_100adf38)
        {
            return mech_trace::rt!("10082d2f", 0xfffffff9);
        }
        let w = |off: usize, v: u32| {
            mech_trace::wr!(
                "10082d2f",
                ((local_1c + off) as *mut u32),
                write_unaligned,
                v
            )
        };
        w(200, i_var2 as u32);
        w(0xcc, local_8 as u32);
        w(0xd0, local_10 as u32);
        w(0xd4, local_24 as u32);
        w(0xc4, local_28 as u32);
        w(0xc0, local_20);
        w(0xd8, local_18 as u32);
        mech_trace::rt!("10082d2f", 1)
    }
}
