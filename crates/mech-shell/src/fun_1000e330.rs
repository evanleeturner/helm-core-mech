// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut core::ffi::c_void;
    static mut DAT_1008d158: [u8; 1];
    static mut DAT_1008d159: [u8; 1];
    static mut DAT_1008d15a: [u8; 1];
    static mut DAT_1008d574: [u8; 1];
    static mut DAT_1008d575: [u8; 1];
    static mut DAT_1008d576: [u8; 1];
    static mut DAT_1008d577: [u8; 1];
    static mut DAT_1008d974: [u8; 1];
    static mut DAT_100d1548: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetDeviceCaps(hdc: *mut core::ffi::c_void, index: i32) -> i32;
    fn GetSystemPaletteEntries(
        hdc: *mut core::ffi::c_void,
        start: u32,
        count: u32,
        entries: *mut core::ffi::c_void,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e330() {
    // SAFETY: the globals and the GDI calls are the game's own; the palette
    // tables are indexed in the same byte offsets the original uses.
    unsafe {
        let mut off: i32 = GetDeviceCaps(DAT_1008d150, 0xc);
        if off == 8 {
            off = GetDeviceCaps(DAT_1008d150, 0x18);
            DAT_100d1548 = (off >> 1) as u32;
            GetSystemPaletteEntries(
                DAT_1008d150,
                0,
                DAT_100d1548,
                (&raw mut DAT_1008d574).cast(),
            );
            let tail = (&raw mut DAT_1008d974)
                .cast::<u8>()
                .wrapping_offset(DAT_100d1548.wrapping_mul(4).wrapping_neg() as i32 as isize);
            GetSystemPaletteEntries(
                DAT_1008d150,
                0x100u32.wrapping_sub(DAT_100d1548),
                DAT_100d1548,
                tail.cast(),
            );
            let per_end = DAT_100d1548;
            if 0 < per_end as i32 {
                let mut remaining = per_end;
                off = 0;
                loop {
                    remaining = remaining.wrapping_sub(1);
                    let o = off as isize;
                    *(&raw mut DAT_1008d15a).cast::<u8>().offset(o) =
                        *(&raw mut DAT_1008d574).cast::<u8>().offset(o);
                    *(&raw mut DAT_1008d159).cast::<u8>().offset(o) =
                        *(&raw mut DAT_1008d575).cast::<u8>().offset(o);
                    *(&raw mut DAT_1008d158).cast::<u8>().offset(o) =
                        *(&raw mut DAT_1008d576).cast::<u8>().offset(o);
                    *(&raw mut DAT_1008d577).cast::<u8>().offset(o) = 0;
                    off = off.wrapping_add(4);
                    if remaining == 0 {
                        break;
                    }
                }
            }
            if (0x100u32.wrapping_sub(per_end) as i32) < 0x100 {
                off = (0x100u32.wrapping_sub(per_end) as i32).wrapping_mul(4);
                loop {
                    let o = off as isize;
                    *(&raw mut DAT_1008d15a).cast::<u8>().offset(o) =
                        *(&raw mut DAT_1008d574).cast::<u8>().offset(o);
                    let next = off.wrapping_add(4);
                    *(&raw mut DAT_1008d159).cast::<u8>().offset(o) =
                        *(&raw mut DAT_1008d575).cast::<u8>().offset(o);
                    *(&raw mut DAT_1008d158).cast::<u8>().offset(o) =
                        *(&raw mut DAT_1008d576).cast::<u8>().offset(o);
                    *(&raw mut DAT_1008d577).cast::<u8>().offset(o) = 0;
                    off = next;
                    if next >= 0x400 {
                        break;
                    }
                }
            }
        }
    }
}
