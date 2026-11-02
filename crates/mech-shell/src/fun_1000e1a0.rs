// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut c_void;
    static mut DAT_1008d154: u32;
    static mut DAT_1008d158: [u8; 0];
    static mut DAT_1008d159: [u8; 0];
    static mut DAT_1008d15a: [u8; 0];
    static mut DAT_1008d570: [u8; 0];
    static mut DAT_1008d574: [u8; 0];
    static mut DAT_1008d575: [u8; 0];
    static mut DAT_1008d576: [u8; 0];
    static mut DAT_1008d577: [u8; 0];
    static mut DAT_100a7260: [i8; 0];
    static mut DAT_100a7560: [u8; 0];
    static mut DAT_100d1548: u32;
    static mut DAT_100d154c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn CreatePalette(p: *const c_void) -> *mut c_void;
    fn SelectPalette(hdc: *mut c_void, pal: *mut c_void, bg: i32) -> *mut c_void;
    fn SetPaletteEntries(pal: *mut c_void, start: u32, count: u32, entries: *const c_void) -> u32;
    fn RealizePalette(hdc: *mut c_void) -> u32;
}

#[inline(always)]
unsafe fn store(off: i32, c: i8, a: *mut u8, b: *mut u8) {
    let v = (c as i32 as u32).wrapping_shl(2) as u8;
    // SAFETY: the game's own palette tables, indexed as the original does.
    unsafe {
        mech_trace::wr!("1000e1a0", a.offset(off as isize), write, v);
        mech_trace::wr!("1000e1a0", b.offset(off as isize), write, v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e1a0(
    first_index: i32,
    entry_count: i32,
    source_colors: *mut u16,
    update_all: i32,
) -> u32 {
    mech_trace::enter!(
        "1000e1a0",
        first_index,
        entry_count,
        source_colors,
        update_all
    );
    let mut entry_count = entry_count;
    if !source_colors.is_null()
        && first_index > -1
        && first_index < 0x100
        && entry_count > 0
        && entry_count <= 0x100i32.wrapping_sub(first_index)
    {
        // SAFETY: all globals are the game's own memory; the pointers are
        // bounds-checked by the original's range test above.
        unsafe {
            let table = (&raw mut DAT_100a7260).cast::<i8>();
            let mut src = source_colors.cast::<u8>();
            let mut dest = table
                .offset(first_index.wrapping_mul(3) as isize)
                .cast::<u8>();
            loop {
                mech_trace::wr!(
                    "1000e1a0",
                    (dest.cast::<u16>()),
                    write_unaligned,
                    mech_trace::rd!("1000e1a0", (src.cast::<u16>()), read_unaligned)
                );
                entry_count = entry_count.wrapping_sub(1);
                mech_trace::wr!(
                    "1000e1a0",
                    dest.add(2),
                    write,
                    mech_trace::rd!("1000e1a0", src.add(2), read)
                );
                dest = dest.add(3);
                src = src.add(3);
                if entry_count == 0 {
                    break;
                }
            }

            let p58 = (&raw mut DAT_1008d158).cast::<u8>();
            let p59 = (&raw mut DAT_1008d159).cast::<u8>();
            let p5a = (&raw mut DAT_1008d15a).cast::<u8>();
            let p74 = (&raw mut DAT_1008d574).cast::<u8>();
            let p75 = (&raw mut DAT_1008d575).cast::<u8>();
            let p76 = (&raw mut DAT_1008d576).cast::<u8>();
            let p77 = (&raw mut DAT_1008d577).cast::<u8>();

            if update_all == 0 {
                let d = mech_trace::gr!("1000e1a0", "100d1548", DAT_100d1548) as i32;
                if d < 0x100i32.wrapping_sub(d) {
                    let mut remaining = d.wrapping_mul(-2).wrapping_add(0x100);
                    let mut t = table.offset(d.wrapping_mul(3) as isize);
                    let mut off = d.wrapping_mul(4);
                    loop {
                        store(off, mech_trace::dr!("1000e1a0", t), p5a, p74);
                        store(off, mech_trace::dr!("1000e1a0", t.add(1)), p59, p75);
                        remaining = remaining.wrapping_sub(1);
                        store(off, mech_trace::dr!("1000e1a0", t.add(2)), p58, p76);
                        mech_trace::wr!("1000e1a0", p77.offset(off as isize), write, 4);
                        t = t.add(3);
                        off = off.wrapping_add(4);
                        if remaining == 0 {
                            break;
                        }
                    }
                }
            } else {
                let mut off: i32 = 0;
                let mut t = table;
                let end = (&raw mut DAT_100a7560).cast::<i8>();
                loop {
                    let next = t.add(3);
                    store(off, mech_trace::dr!("1000e1a0", t), p5a, p74);
                    store(off, mech_trace::dr!("1000e1a0", t.add(1)), p59, p75);
                    store(off, mech_trace::dr!("1000e1a0", t.add(2)), p58, p76);
                    mech_trace::wr!("1000e1a0", p77.offset(off as isize), write, 4);
                    off = off.wrapping_add(4);
                    t = next;
                    if !(next < end) {
                        break;
                    }
                }
            }

            if mech_trace::gr!("1000e1a0", "1008d154", DAT_1008d154) == 0 {
                mech_trace::gw!(
                    "1000e1a0",
                    "1008d154",
                    DAT_1008d154,
                    mech_trace::cr!(
                        "1000e1a0",
                        "CreatePalette",
                        CreatePalette((&raw const DAT_1008d570).cast::<c_void>())
                    ) as usize as u32
                );
                mech_trace::gw!(
                    "1000e1a0",
                    "100d154c",
                    DAT_100d154c,
                    mech_trace::cr!(
                        "1000e1a0",
                        "SelectPalette",
                        SelectPalette(
                            mech_trace::gr!("1000e1a0", "1008d150", DAT_1008d150),
                            mech_trace::gr!("1000e1a0", "1008d154", DAT_1008d154) as usize
                                as *mut c_void,
                            0
                        )
                    ) as usize as u32
                );
            } else {
                mech_trace::cr!(
                    "1000e1a0",
                    "SetPaletteEntries",
                    SetPaletteEntries(
                        mech_trace::gr!("1000e1a0", "1008d154", DAT_1008d154) as usize
                            as *mut c_void,
                        0,
                        0x100,
                        (&raw const DAT_1008d574).cast::<c_void>(),
                    )
                );
            }
            mech_trace::cr!(
                "1000e1a0",
                "RealizePalette",
                RealizePalette(mech_trace::gr!("1000e1a0", "1008d150", DAT_1008d150))
            );
        }
        return mech_trace::rt!("1000e1a0", 1);
    }
    mech_trace::rt!("1000e1a0", 0)
}
