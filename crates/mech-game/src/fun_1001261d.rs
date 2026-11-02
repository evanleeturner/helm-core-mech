// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1dc: *mut u16;
    static mut DAT_100aa1e0: u32;
    static mut DAT_100aa254: u16;
    static mut DAT_100adf38: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef030: [u8; 0];
    static mut DAT_100ef032: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: u32, param_2: u32) -> u8;
}

unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the caller supplies an address of a live game object.
    unsafe { mech_trace::rd!("1001261d", (a as usize as *const T), read_unaligned) }
}

unsafe fn wr<T>(a: u32, v: T) {
    // SAFETY: the caller supplies an address of a live game object.
    unsafe { mech_trace::wr!("1001261d", (a as usize as *mut T), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001261d() {
    mech_trace::enter!("1001261d");
    // SAFETY: all globals and the objects they point to are the game's own
    // memory; reads and writes happen in the original's order.
    unsafe {
        let b = || mech_trace::gr!("1001261d", "100aa1dc", DAT_100aa1dc) as u32;

        wr::<u16>(b(), mech_trace::gr!("1001261d", "100aa254", DAT_100aa254));
        let i_var2: u32 = mech_trace::rd!(
            "1001261d",
            (&raw mut DAT_100eef40 as *const u32).add(mech_trace::gr!(
                "1001261d",
                "100adf38",
                DAT_100adf38
            ) as usize),
            read
        );
        let pi_var3: u32 = rd::<u32>(i_var2.wrapping_add(0x20));
        wr::<u16>(b().wrapping_add(7), 0);
        wr::<u8>(b().wrapping_add(2), rd::<u8>(i_var2.wrapping_add(4)));
        wr::<u16>(
            b().wrapping_add(3),
            rd::<u32>(i_var2.wrapping_add(0xdc)) as u16,
        );
        let v: u32 = rd::<u32>(i_var2.wrapping_add(0xdc));
        if ((v >> 8) as u8 & 0xf) == 4 {
            let u_var4: u32 = rd::<u32>(i_var2.wrapping_add(0xdc)) & 0xff;
            let a30 =
                (&raw mut DAT_100ef030 as *mut u8 as u32).wrapping_add(u_var4.wrapping_mul(0x40));
            if (rd::<u16>(a30) & 0x20) != 0 {
                let sh = (rd::<u8>(i_var2.wrapping_add(8)) & 0x1f) as u32;
                let a32 = (&raw mut DAT_100ef032 as *mut u8 as u32)
                    .wrapping_add(u_var4.wrapping_mul(0x40));
                let s = rd::<i16>(a32) as i32;
                if ((1u32.wrapping_shl(sh)) as i32 & s) != 0 {
                    let o = rd::<u16>(b().wrapping_add(7));
                    wr::<u16>(b().wrapping_add(7), o | 0x400);
                }
            }
        }
        wr::<u16>(
            b().wrapping_add(5),
            rd::<u32>(i_var2.wrapping_add(0x13c)) as u16,
        );
        wr::<u32>(b().wrapping_add(0x59), rd::<u32>(i_var2.wrapping_add(0x50)));
        wr::<u32>(b().wrapping_add(0x5d), rd::<u32>(i_var2.wrapping_add(0x54)));
        wr::<u32>(b().wrapping_add(0x61), rd::<u32>(i_var2.wrapping_add(0x58)));
        wr::<u32>(b().wrapping_add(0x65), rd::<u32>(i_var2.wrapping_add(0x5c)));
        wr::<u32>(b().wrapping_add(0x69), rd::<u32>(i_var2.wrapping_add(0x60)));
        wr::<u32>(b().wrapping_add(0x6d), rd::<u32>(i_var2.wrapping_add(100)));
        wr::<u32>(
            b().wrapping_add(0x71),
            rd::<u32>(pi_var3.wrapping_add(0xb * 4)),
        );
        wr::<u32>(
            b().wrapping_add(0x75),
            rd::<u32>(pi_var3.wrapping_add(0x13 * 4)),
        );
        wr::<u32>(
            b().wrapping_add(0x79),
            rd::<u32>(pi_var3.wrapping_add(0xe * 4)),
        );
        wr::<u32>(
            b().wrapping_add(0x7d),
            rd::<u32>(pi_var3.wrapping_add(2 * 4)),
        );
        wr::<u32>(
            b().wrapping_add(0x81),
            rd::<u32>(pi_var3.wrapping_add(6 * 4)),
        );
        let w = (rd::<u32>(pi_var3.wrapping_add(0xf5 * 4)) as u16 & 0xf)
            | rd::<u16>(b().wrapping_add(7));
        wr::<u16>(b().wrapping_add(7), w);
        if (rd::<u16>(pi_var3.wrapping_add(0x110 * 4)) & 0x80) != 0 {
            let o = rd::<u16>(b().wrapping_add(7));
            wr::<u16>(b().wrapping_add(7), o | 0x800);
        }
        let p0: u32 = rd::<u32>(pi_var3);
        let p4c: u32 = rd::<u32>(p0.wrapping_add(0x4c));
        if rd::<u8>(p4c.wrapping_add(0x30)) != 0 {
            let o = rd::<u16>(b().wrapping_add(7));
            wr::<u16>(b().wrapping_add(7), o | 0x10);
        }
        if 0 < rd::<i32>(pi_var3.wrapping_add(0xfd * 4))
            && rd::<i32>(pi_var3.wrapping_add(0xf5 * 4)) == 2
        {
            let p0: u32 = rd::<u32>(pi_var3);
            let i2: u32 = rd::<u32>(p0.wrapping_add(0x4c));
            if rd::<u8>(i2.wrapping_add(0x1d)) != 0 {
                let o = rd::<u16>(b().wrapping_add(7));
                wr::<u16>(b().wrapping_add(7), o | 0x20);
            }
            if rd::<u8>(i2.wrapping_add(0x1e)) != 0 {
                let o = rd::<u16>(b().wrapping_add(7));
                wr::<u16>(b().wrapping_add(7), o | 0x40);
            }
            if rd::<u8>(i2.wrapping_add(0x1f)) != 0 {
                let o = rd::<u16>(b().wrapping_add(7));
                wr::<u16>(b().wrapping_add(7), o | 0x80);
            }
            if rd::<u8>(i2.wrapping_add(0x20)) != 0 {
                let o = rd::<u16>(b().wrapping_add(7));
                wr::<u16>(b().wrapping_add(7), o | 0x100);
            }
            if rd::<u8>(i2.wrapping_add(0x21)) != 0 {
                let o = rd::<u16>(b().wrapping_add(7));
                wr::<u16>(b().wrapping_add(7), o | 0x200);
            }
        }
        wr::<u32>(
            b().wrapping_add(0x55),
            mech_trace::gr!("1001261d", "100b38fc", DAT_100b38fc),
        );
        let mut local_14: i32 = 0;
        while local_14 < 8 {
            let pu_var1: u32 = rd::<u32>(pi_var3.wrapping_add(0x16 * 4))
                .wrapping_add((local_14 as u32).wrapping_mul(0x28));
            if local_14 == 1 {
                wr::<u32>(b().wrapping_add(0x11), rd::<u32>(pu_var1.wrapping_add(4)));
            } else if local_14 == 2 {
                wr::<u32>(b().wrapping_add(0xd), rd::<u32>(pu_var1.wrapping_add(4)));
            } else if local_14 == 3 {
                wr::<u32>(b().wrapping_add(9), rd::<u32>(pu_var1.wrapping_add(4)));
            }
            wr::<u32>(
                b().wrapping_add((local_14 as u32).wrapping_mul(4))
                    .wrapping_add(0x15),
                rd::<u32>(pu_var1),
            );
            wr::<u32>(
                b().wrapping_add((local_14 as u32).wrapping_mul(4))
                    .wrapping_add(0x35),
                rd::<u32>(pu_var1.wrapping_add(8)),
            );
            local_14 = local_14.wrapping_add(1);
        }
        mech_trace::cr!(
            "1001261d",
            "10006e00",
            FUN_10006e00(
                mech_trace::gr!("1001261d", "100aa1dc", DAT_100aa1dc) as u32,
                mech_trace::gr!("1001261d", "100aa1e0", DAT_100aa1e0)
            )
        );
    }
}
