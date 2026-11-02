// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3dfc: i32;
    static mut DAT_100b9a30: i32;
    static mut DAT_100b9a32: i32;
    static mut DAT_100c927a: i32;
    static mut DAT_100c9279: i32;
    static mut DAT_100c9278: i32;
    static mut DAT_100c9250: i32;
    static mut DAT_100b3de0: i32;
    static mut DAT_100eee30: i32;
    static mut DAT_100b3fe4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut u8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SendMessageA(h_wnd: i32, msg: u32, w_param: i32, l_param: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006fa53(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    mech_trace::enter!("1006fa53", param_1, param_2, param_3);
    // SAFETY: the globals and callees are the game's own; param_3 points at
    // the caller's palette bytes, and the destination tables are the game's
    // own arrays addressed at the same offsets as the original.
    unsafe {
        if mech_trace::gr!("1006fa53", "100b3dfc", DAT_100b3dfc) == 0 {
            return mech_trace::rt!("1006fa53", -1);
        }
        if param_3 == 0
            || param_1 < 0
            || 0xff < param_1
            || param_2 < 1
            || 0x100i32.wrapping_sub(param_1) < param_2
        {
            return mech_trace::rt!("1006fa53", -1);
        }
        let src = param_3 as usize as *const u8;
        let mut local_8: i32 = 0;
        while local_8 < param_2 {
            let off = local_8.wrapping_mul(3) as isize;
            let p3 = src.offset(off);
            let idx = local_8.wrapping_add(param_1);
            let i_var1 = idx.wrapping_mul(3) as isize;
            let dst30 = (&raw mut DAT_100b9a30).cast::<u8>().offset(i_var1);
            mech_trace::wr!(
                "1006fa53",
                dst30.cast::<i16>(),
                write_unaligned,
                mech_trace::rd!("1006fa53", p3.cast::<i16>(), read_unaligned)
            );
            let dst32 = (&raw mut DAT_100b9a32).cast::<u8>().offset(i_var1);
            mech_trace::wr!(
                "1006fa53",
                dst32,
                write,
                mech_trace::rd!("1006fa53", p3.add(2), read)
            );
            let k = idx.wrapping_mul(4) as isize;
            mech_trace::wr!(
                "1006fa53",
                (&raw mut DAT_100c927a).cast::<u8>().offset(k),
                write,
                mech_trace::rd!("1006fa53", src.offset(off), read).wrapping_mul(4)
            );
            mech_trace::wr!(
                "1006fa53",
                (&raw mut DAT_100c9279).cast::<u8>().offset(k),
                write,
                mech_trace::rd!("1006fa53", src.offset(off + 1), read).wrapping_mul(4)
            );
            mech_trace::wr!(
                "1006fa53",
                (&raw mut DAT_100c9278).cast::<u8>().offset(k),
                write,
                mech_trace::rd!("1006fa53", src.offset(off + 2), read).wrapping_mul(4)
            );
            local_8 = local_8.wrapping_add(1);
        }
        let mut local: [i32; 3] = [0x400, 0x428, (&raw mut DAT_100c9250) as usize as i32];
        mech_trace::gw!(
            "1006fa53",
            "100eee30",
            DAT_100eee30,
            mech_trace::cr!(
                "1006fa53",
                "SendMessageA",
                SendMessageA(
                    mech_trace::gr!("1006fa53", "100b3de0", DAT_100b3de0),
                    0x4a,
                    0,
                    (&raw mut local) as usize as i32
                )
            )
        );
        mech_trace::rt!(
            "1006fa53",
            if mech_trace::gr!("1006fa53", "100eee30", DAT_100eee30) == 0 {
                0
            } else {
                FUN_10002168(&raw mut DAT_100b3fe4);
                -1
            }
        )
    }
}
