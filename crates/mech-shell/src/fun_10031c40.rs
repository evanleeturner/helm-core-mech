// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
    static mut DAT_100b5140: [u8; 1];
    static mut DAT_100b5144: [u8; 1];
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
    static mut DAT_100b5220: [u8; 1];
    static mut DAT_100b5288: u32;
    static mut DAT_100b528c: u32;
    static mut DAT_100b5294: u32;
}

#[allow(non_snake_case)]
unsafe fn put(off: u32, v: u32) {
    // SAFETY: DAT_10092000 holds the game's object pointer; the write lands in that object.
    unsafe {
        let p = mech_trace::gr!("10031c40", "10092000", DAT_10092000).wrapping_add(off) as *mut u32;
        mech_trace::wr!("10031c40", p, write_unaligned, v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10031c40() {
    mech_trace::enter!("10031c40");
    // SAFETY: all globals and the object they point to are the game's own memory;
    // reads and writes occur in the original's order.
    unsafe {
        put(0x7f0, mech_trace::gr!("10031c40", "100b5214", DAT_100b5214));
        let mut i3: i32 = 0;
        loop {
            let v = mech_trace::rd!(
                "10031c40",
                (&raw mut DAT_100b5220)
                    .cast::<u8>()
                    .byte_add(i3 as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            i3 = i3.wrapping_add(4);
            put(0x288u32.wrapping_add(i3 as u32), v);
            if !(i3 < 100) {
                break;
            }
        }
        i3 = 0;
        loop {
            let a = mech_trace::rd!(
                "10031c40",
                (&raw mut DAT_100b5140)
                    .cast::<u8>()
                    .byte_add(i3 as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "10031c40",
                (&raw mut DAT_100b5144)
                    .cast::<u8>()
                    .byte_add(i3 as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            let off = i3 as u32;
            i3 = i3.wrapping_add(8);
            put(off.wrapping_add(0x2f0), a);
            put(off.wrapping_add(0x2f4), b);
            if !(i3 < 0x70) {
                break;
            }
        }
        put(0x7f0, mech_trace::gr!("10031c40", "100b5214", DAT_100b5214));
        put(0x280, mech_trace::gr!("10031c40", "100b51b0", DAT_100b51b0));
        put(0x284, mech_trace::gr!("10031c40", "100b5294", DAT_100b5294));
        put(0x288, mech_trace::gr!("10031c40", "100b51dc", DAT_100b51dc));
        put(0x20c, mech_trace::gr!("10031c40", "100b51d4", DAT_100b51d4));
        put(0x210, mech_trace::gr!("10031c40", "100b51b4", DAT_100b51b4));
        put(0x214, mech_trace::gr!("10031c40", "100b51cc", DAT_100b51cc));
        put(0x218, mech_trace::gr!("10031c40", "100b51c0", DAT_100b51c0));
        put(0x21c, mech_trace::gr!("10031c40", "100b51e0", DAT_100b51e0));
        put(0x234, mech_trace::gr!("10031c40", "100b51e4", DAT_100b51e4));
        put(0x238, mech_trace::gr!("10031c40", "100b5218", DAT_100b5218));
        put(0x23c, mech_trace::gr!("10031c40", "100b5288", DAT_100b5288));
        put(0x228, mech_trace::gr!("10031c40", "100b51d8", DAT_100b51d8));
        put(0x22c, mech_trace::gr!("10031c40", "100b51bc", DAT_100b51bc));
        put(0x230, mech_trace::gr!("10031c40", "100b521c", DAT_100b521c));
        put(0x278, mech_trace::gr!("10031c40", "100b528c", DAT_100b528c));
        put(0x27c, mech_trace::gr!("10031c40", "100b51d0", DAT_100b51d0));
    }
}
