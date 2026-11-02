// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
    static mut DAT_100b5140: u8;
    static mut DAT_100b51b0: u32;
    static mut DAT_100b51b4: u32;
    static mut DAT_100b51bc: u32;
    static mut DAT_100b51c0: u32;
    static mut DAT_100b51cc: u32;
    static mut DAT_100b51d0: u32;
    static mut DAT_100b51d4: u32;
    static mut DAT_100b51d8: u32;
    static mut DAT_100b51dc: u32;
    static mut DAT_100b51e0: u32;
    static mut DAT_100b51e4: u32;
    static mut DAT_100b5214: u32;
    static mut DAT_100b5218: u32;
    static mut DAT_100b521c: u32;
    static mut DAT_100b5220: u8;
    static mut DAT_100b5284: u8;
    static mut DAT_100b5288: u32;
    static mut DAT_100b528c: u32;
    static mut DAT_100b5294: u32;
}

#[inline(always)]
unsafe fn rd(off: u32) -> u32 {
    // SAFETY: the caller guarantees DAT_10092000 points at the game's live structure.
    unsafe {
        mech_trace::rd!(
            "10031b20",
            ((mech_trace::gr!("10031b20", "10092000", DAT_10092000).wrapping_add(off))
                as *const u32),
            read_unaligned
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10031b20() {
    mech_trace::enter!("10031b20");
    // SAFETY: all globals and the structure behind DAT_10092000 are the
    // game's own memory; reads and writes occur in the original's order.
    unsafe {
        mech_trace::gw!("10031b20", "100b5214", DAT_100b5214, rd(0x7f0));
        let mut p3 =
            mech_trace::gr!("10031b20", "10092000", DAT_10092000).wrapping_add(0x28c) as *const u32;
        let mut p2 = &raw mut DAT_100b5220 as *mut u32;
        loop {
            let v = mech_trace::rd!("10031b20", p3, read_unaligned);
            p3 = p3.add(1);
            mech_trace::wr!("10031b20", p2, write_unaligned, v);
            p2 = p2.add(1);
            if !(p2 < (&raw mut DAT_100b5284 as *mut u32)) {
                break;
            }
        }
        let mut q3 = &raw mut DAT_100b5140 as *mut u32;
        let mut q2 =
            mech_trace::gr!("10031b20", "10092000", DAT_10092000).wrapping_add(0x2f0) as *const u32;
        loop {
            let v = mech_trace::rd!("10031b20", q2.add(1), read_unaligned);
            mech_trace::wr!(
                "10031b20",
                q3,
                write_unaligned,
                mech_trace::rd!("10031b20", q2, read_unaligned)
            );
            q2 = q2.add(2);
            mech_trace::wr!("10031b20", q3.add(1), write_unaligned, v);
            q3 = q3.add(2);
            if !(q3 < (&raw mut DAT_100b51b0)) {
                break;
            }
        }
        mech_trace::gw!("10031b20", "100b51b0", DAT_100b51b0, rd(0x280));
        mech_trace::gw!("10031b20", "100b5294", DAT_100b5294, rd(0x284));
        mech_trace::gw!("10031b20", "100b51dc", DAT_100b51dc, rd(0x288));
        mech_trace::gw!("10031b20", "100b51e4", DAT_100b51e4, rd(0x234));
        mech_trace::gw!("10031b20", "100b5218", DAT_100b5218, rd(0x238));
        mech_trace::gw!("10031b20", "100b5288", DAT_100b5288, rd(0x23c));
        mech_trace::gw!("10031b20", "100b51d8", DAT_100b51d8, rd(0x228));
        mech_trace::gw!("10031b20", "100b51bc", DAT_100b51bc, rd(0x22c));
        mech_trace::gw!("10031b20", "100b521c", DAT_100b521c, rd(0x230));
        mech_trace::gw!("10031b20", "100b528c", DAT_100b528c, rd(0x278));
        mech_trace::gw!("10031b20", "100b51d0", DAT_100b51d0, rd(0x27c));
        mech_trace::gw!("10031b20", "100b51d4", DAT_100b51d4, rd(0x20c));
        mech_trace::gw!("10031b20", "100b51b4", DAT_100b51b4, rd(0x210));
        mech_trace::gw!("10031b20", "100b51cc", DAT_100b51cc, rd(0x214));
        mech_trace::gw!("10031b20", "100b51c0", DAT_100b51c0, rd(0x218));
        mech_trace::gw!("10031b20", "100b51e0", DAT_100b51e0, rd(0x21c));
    }
}
