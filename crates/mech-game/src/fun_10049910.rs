// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101ec750: u32;
    static mut DAT_101ec780: u32;
    static mut DAT_101f26f0: u32;
    static mut DAT_101f26f4: u32;
    static mut DAT_101f270c: u32;
    static mut DAT_101f2714: u32;
    static mut DAT_101f2718: u32;
    static mut DAT_100c7e18: u32;
    static mut DAT_100c7e14: u32;
    static mut _DAT_100c7e10: u32;
    static mut DAT_100c7e20: u32;
    static mut DAT_100c7e24: u32;
    static mut _DAT_100c7e1c: u32;
    static mut _DAT_100c7e28: u32;
    static mut DAT_10165850: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10049a9b(param_1: i32);
    fn FUN_10049a38(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10049910() {
    mech_trace::enter!("10049910");
    // SAFETY: the globals and callees are the game's own memory; offsets
    // stay within the original's arrays exactly as the C does.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < 0xff {
            let off = local_8.wrapping_mul(0x60) as usize;
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101ec750).byte_add(off),
                write_unaligned,
                0xffff_ffff
            );
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101ec780).byte_add(off),
                write_unaligned,
                0
            );
            FUN_10049a9b(local_8);
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0xff {
            let off = local_8.wrapping_mul(0x2c) as usize;
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101f26f0).byte_add(off),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101f26f4).byte_add(off),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101f270c).byte_add(off),
                write_unaligned,
                0xffff_ffff
            );
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101f2714).byte_add(off),
                write_unaligned,
                0xffff_ffff
            );
            mech_trace::wr!(
                "10049910",
                (&raw mut DAT_101f2718).byte_add(off),
                write_unaligned,
                0xffff_ffff
            );
            FUN_10049a38(local_8);
            local_8 = local_8.wrapping_add(1);
        }
        mech_trace::gw!("10049910", "100c7e18", DAT_100c7e18, 0);
        mech_trace::gw!("10049910", "100c7e14", DAT_100c7e14, 0);
        mech_trace::gw!("10049910", "_DAT_100c7e10", _DAT_100c7e10, 0);
        mech_trace::gw!("10049910", "100c7e20", DAT_100c7e20, 0);
        mech_trace::gw!("10049910", "100c7e24", DAT_100c7e24, 0);
        mech_trace::gw!("10049910", "_DAT_100c7e1c", _DAT_100c7e1c, 0);
        mech_trace::gw!("10049910", "_DAT_100c7e28", _DAT_100c7e28, 0);
        let mut p: *mut u32 = &raw mut DAT_10165850;
        let mut i: i32 = 0x35;
        while i != 0 {
            mech_trace::wr!("10049910", p, write_unaligned, 0);
            p = p.add(1);
            i = i.wrapping_sub(1);
        }
        mech_trace::wr!("10049910", p.cast::<u16>(), write_unaligned, 0);
    }
}
