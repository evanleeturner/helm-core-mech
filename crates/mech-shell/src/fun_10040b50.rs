// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100982a8: u32;
    static mut DAT_100982ac: u32;
    static mut DAT_100982b0: u32;
    static mut DAT_100982b4: u32;
    static mut DAT_100982b8: u32;
    static mut DAT_100982bc: u8;
    static mut DAT_100982c0: u32;
    static mut DAT_100982c4: u32;
    static mut DAT_100982c8: u32;
    static mut DAT_100982cc: u32;
    static mut DAT_100982d0: u32;
    static mut DAT_1009831c: u8;
    static mut DAT_100ba008: u32;
    static mut DAT_100ba00c: u32;
    static mut DAT_100ba010: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10040b50() {
    mech_trace::enter!("10040b50");
    // SAFETY: all globals and the table are the game's own memory; the loop
    // writes 44-byte records within the table and copies the string at
    // DAT_1009831c into each, exactly as the original does.
    unsafe {
        mech_trace::gw!("10040b50", "100982a8", DAT_100982a8, 0);
        mech_trace::gw!("10040b50", "100982ac", DAT_100982ac, 0);
        mech_trace::gw!("10040b50", "100982b0", DAT_100982b0, 0);
        mech_trace::gw!("10040b50", "100982b4", DAT_100982b4, 0);
        mech_trace::gw!("10040b50", "100982b8", DAT_100982b8, 0);
        mech_trace::gw!("10040b50", "100982bc", DAT_100982bc, 0);
        mech_trace::gw!("10040b50", "100982c0", DAT_100982c0, 0);
        mech_trace::gw!("10040b50", "100982c4", DAT_100982c4, 0);
        mech_trace::gw!("10040b50", "100982c8", DAT_100982c8, 0);
        let mut p4 = (&raw mut DAT_100ba010) as *mut u32;
        mech_trace::gw!("10040b50", "100ba008", DAT_100ba008, 0);
        mech_trace::gw!("10040b50", "100ba00c", DAT_100ba00c, 0);
        mech_trace::gw!("10040b50", "100982cc", DAT_100982cc, 0);
        mech_trace::gw!("10040b50", "100982d0", DAT_100982d0, 0);
        loop {
            let mut n: u32 = 0xffffffff;
            mech_trace::wr!("10040b50", p4, write_unaligned, 0);
            mech_trace::wr!("10040b50", p4.add(1), write_unaligned, 0);
            mech_trace::wr!("10040b50", p4.add(2), write_unaligned, 0xffffffff);
            let src = (&raw const DAT_1009831c) as *const u8;
            let mut q = src;
            loop {
                if n == 0 {
                    break;
                }
                n = n.wrapping_sub(1);
                let c = mech_trace::rd!("10040b50", q, read);
                q = q.add(1);
                if c == 0 {
                    break;
                }
            }
            let len = !n;
            let mut dst = p4.add(3) as *mut u8;
            p4 = p4.add(11);
            let mut s = src;
            let mut k = len >> 2;
            while k != 0 {
                mech_trace::wr!(
                    "10040b50",
                    (dst as *mut u32),
                    write_unaligned,
                    mech_trace::rd!("10040b50", (s as *const u32), read_unaligned)
                );
                s = s.add(4);
                dst = dst.add(4);
                k = k.wrapping_sub(1);
            }
            let mut k = len & 3;
            while k != 0 {
                mech_trace::wr!("10040b50", dst, write, mech_trace::rd!("10040b50", s, read));
                s = s.add(1);
                dst = dst.add(1);
                k = k.wrapping_sub(1);
            }
            if (p4 as usize) > 0x100ba1c7 {
                return;
            }
        }
    }
}
