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
    unsafe { ((DAT_10092000.wrapping_add(off)) as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10031b20() {
    // SAFETY: all globals and the structure behind DAT_10092000 are the
    // game's own memory; reads and writes occur in the original's order.
    unsafe {
        DAT_100b5214 = rd(0x7f0);
        let mut p3 = DAT_10092000.wrapping_add(0x28c) as *const u32;
        let mut p2 = &raw mut DAT_100b5220 as *mut u32;
        loop {
            let v = p3.read_unaligned();
            p3 = p3.add(1);
            p2.write_unaligned(v);
            p2 = p2.add(1);
            if !(p2 < (&raw mut DAT_100b5284 as *mut u32)) {
                break;
            }
        }
        let mut q3 = &raw mut DAT_100b5140 as *mut u32;
        let mut q2 = DAT_10092000.wrapping_add(0x2f0) as *const u32;
        loop {
            let v = q2.add(1).read_unaligned();
            q3.write_unaligned(q2.read_unaligned());
            q2 = q2.add(2);
            q3.add(1).write_unaligned(v);
            q3 = q3.add(2);
            if !(q3 < (&raw mut DAT_100b51b0)) {
                break;
            }
        }
        DAT_100b51b0 = rd(0x280);
        DAT_100b5294 = rd(0x284);
        DAT_100b51dc = rd(0x288);
        DAT_100b51e4 = rd(0x234);
        DAT_100b5218 = rd(0x238);
        DAT_100b5288 = rd(0x23c);
        DAT_100b51d8 = rd(0x228);
        DAT_100b51bc = rd(0x22c);
        DAT_100b521c = rd(0x230);
        DAT_100b528c = rd(0x278);
        DAT_100b51d0 = rd(0x27c);
        DAT_100b51d4 = rd(0x20c);
        DAT_100b51b4 = rd(0x210);
        DAT_100b51cc = rd(0x214);
        DAT_100b51c0 = rd(0x218);
        DAT_100b51e0 = rd(0x21c);
    }
}
