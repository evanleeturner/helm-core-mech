// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100ad0b0: i32;
    static mut DAT_10123240: [i8; 0];
    static mut DAT_10123130: [i32; 0];
    static mut DAT_101600e8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d47f(param_1: i32) -> u32;
    fn FUN_10027fed(param_1: i32) -> i32;
    fn FUN_1002b6fe(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1002b48b(param_1: i32) -> u32;
    fn FUN_1002b4cf(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002a3e0(param_1: i32, param_2: i16, param_3: u16);
    fn FUN_1002c354(param_1: i32);
}

#[allow(non_snake_case)]
unsafe fn handle(
    local_c: *mut i32,
    local_20: i32,
    u_var2: u16,
    local_18: i32,
    u_var1: u16,
    local_8: u32,
) {
    // SAFETY: local_c points at a game object returned by the game's own
    // routine; the fields are read and written where the original does.
    unsafe {
        if mech_trace::rd!("1002b120", local_c.byte_add(16), read_unaligned) == 2 {
            FUN_1002b4cf(
                local_c,
                2,
                (local_20 as i16 as i32).wrapping_add(1),
                u_var2 as i32,
            );
            if (u_var2 as i16 as i32) != mech_trace::gr!("1002b120", "100adf38", DAT_100adf38)
                || mech_trace::gr!("1002b120", "100ad0b0", DAT_100ad0b0) == 0
            {
                if mech_trace::rd!("1002b120", local_c.byte_add(4), read_unaligned)
                    == (u_var2 as i16 as i32)
                    && local_18 == 5
                {
                    FUN_1002a3e0(local_c as i32, 0, 0);
                } else {
                    FUN_1002a3e0(local_c as i32, local_18 as i16, u_var1);
                }
                let p = local_c.byte_add(0x17e);
                let mut t = mech_trace::rd!("1002b120", p, read_unaligned) as u32;
                t &= 0xfffffffc;
                mech_trace::wr!("1002b120", p, write_unaligned, t as i32);
                let mut t = mech_trace::rd!("1002b120", p, read_unaligned) as u32;
                t |= local_8;
                mech_trace::wr!("1002b120", p, write_unaligned, t as i32);
                FUN_1002c354(local_c as i32);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b120(param_1: i32) {
    mech_trace::enter!("1002b120", param_1);
    // SAFETY: the callees and globals are the game's own memory; pointers
    // derived from them are read and written where the original does.
    unsafe {
        let mut u_var1: u16 = 0;
        let u_var3 = mech_trace::cr!("1002b120", "1004d47f", FUN_1004d47f(param_1));
        let mut u_var2 = u_var3 as u16;
        if u_var2 == 0xffff {
            let i = mech_trace::cr!("1002b120", "10027fed", FUN_10027fed(param_1));
            u_var2 = i as u16;
            if u_var2 == 0xffff {
                return;
            }
        }
        let mut local_44: [*mut i32; 5] = [core::ptr::null_mut(); 5];
        let i_var4 = mech_trace::cr!(
            "1002b120",
            "1002b6fe",
            FUN_1002b6fe(param_1, local_44.as_mut_ptr() as i32, 1)
        );
        if i_var4 == 0 {
            return;
        }
        let local_10: *mut i8 = (&raw mut DAT_10123240)
            .cast::<i8>()
            .wrapping_byte_add((param_1 as u32).wrapping_mul(0x3c0a) as usize);
        let local_24 = mech_trace::rd!(
            "1002b120",
            (&raw mut DAT_10123130)
                .cast::<i32>()
                .offset(param_1 as isize),
            read_unaligned
        );
        let off = (local_24 as u32).wrapping_mul(0x13f) as usize;
        let local_14: u16 = if local_24 == -1 {
            0x200
        } else {
            mech_trace::rd!(
                "1002b120",
                local_10
                    .wrapping_byte_add(off)
                    .wrapping_byte_add(0x3c)
                    .cast::<i32>(),
                read_unaligned
            ) as u16
        };
        let local_18: i32;
        if local_14 == 0x400 {
            local_18 = 10;
        } else if local_14 == 0x800 || local_14 == 0x10 {
            local_18 = 0xb;
        } else {
            u_var1 = u_var2 | 0x200;
            local_18 = 5;
        }
        let mut local_8: u32 = 0x40;
        let t = mech_trace::rd!(
            "1002b120",
            local_10
                .wrapping_byte_add(off)
                .wrapping_byte_add(0xb7)
                .cast::<i32>(),
            read_unaligned
        );
        let u_var5 = mech_trace::cr!("1002b120", "1002b48b", FUN_1002b48b(t));
        local_8 |= u_var5;
        let mut local_20: i32 = 0;
        while local_20 < 0x10
            && ((1i32.wrapping_shl((local_20 as u8 & 0x1f) as u32)) & (local_14 as i32)) == 0
        {
            local_20 += 1;
        }
        let mut local_1c: i32 = 0;
        while local_1c < i_var4 {
            let local_c = local_44[local_1c as usize];
            handle(local_c, local_20, u_var2, local_18, u_var1, local_8);
            local_1c += 1;
        }
        let flag = mech_trace::rd!(
            "1002b120",
            (&raw mut DAT_101600e8)
                .cast::<i8>()
                .wrapping_byte_add((param_1 as u32).wrapping_mul(0x3c) as usize)
                .cast::<i32>(),
            read_unaligned
        );
        if flag != 0 {
            let local_c = local_44[4];
            handle(local_c, local_20, u_var2, local_18, u_var1, local_8);
        }
        let i_var4 = mech_trace::cr!("1002b120", "1004d47f", FUN_1004d47f(param_1)) as i32;
        if i_var4 == mech_trace::gr!("1002b120", "100adf38", DAT_100adf38) && local_14 != 0x10 {
            mech_trace::gw!("1002b120", "100ad0b0", DAT_100ad0b0, 1);
        }
    }
}
