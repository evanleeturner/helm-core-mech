// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut c_void;
    static mut DAT_100afbc4: u32;
    static mut DAT_100affe0: [u8; 0];
    static mut DAT_100affe4: [u8; 0];
    static mut DAT_100affe5: [u8; 0];
    static mut DAT_100affe6: [u8; 0];
    static mut DAT_100affe7: [u8; 0];
    static mut DAT_100b9a30: [u8; 0];
    static mut DAT_100b9a31: [u8; 0];
    static mut DAT_100b9a32: [u8; 0];
    static mut DAT_10160bf4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn CreatePalette(param_1: *const c_void) -> *mut c_void;
    fn SelectPalette(param_1: *mut c_void, param_2: *mut c_void, param_3: i32) -> *mut c_void;
    fn SetPaletteEntries(
        param_1: *mut c_void,
        param_2: u32,
        param_3: u32,
        param_4: *const c_void,
    ) -> u32;
    fn RealizePalette(param_1: *mut c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045873(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    mech_trace::enter!("10045873", param_1, param_2, param_3);
    if param_3 == 0
        || param_1 < 0
        || 0xff < param_1
        || param_2 < 1
        || 0x100i32.wrapping_sub(param_1) < param_2
    {
        return mech_trace::rt!("10045873", 0);
    }
    // SAFETY: the globals are the game's own memory; param_3 points at the
    // caller's palette bytes, and all accesses mirror the original's offsets.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < param_2 {
            let src = param_3.wrapping_add(local_8.wrapping_mul(3)) as *const u8;
            let i_var1 = local_8.wrapping_add(param_1).wrapping_mul(3);
            let base = (&raw mut DAT_100b9a30).cast::<u8>();
            mech_trace::wr!(
                "10045873",
                base.offset(i_var1 as isize).cast::<u16>(),
                write_unaligned,
                mech_trace::rd!("10045873", src.cast::<u16>(), read_unaligned)
            );
            let b2 = (&raw mut DAT_100b9a32).cast::<u8>();
            mech_trace::wr!(
                "10045873",
                b2.offset(i_var1 as isize),
                write,
                mech_trace::rd!("10045873", src.byte_add(2), read)
            );
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0x100 {
            let s0 = (&raw mut DAT_100b9a30).cast::<u8>();
            let s1 = (&raw mut DAT_100b9a31).cast::<u8>();
            let s2 = (&raw mut DAT_100b9a32).cast::<u8>();
            let d0 = (&raw mut DAT_100affe4).cast::<u8>();
            let d1 = (&raw mut DAT_100affe5).cast::<u8>();
            let d2 = (&raw mut DAT_100affe6).cast::<u8>();
            let d3 = (&raw mut DAT_100affe7).cast::<u8>();
            let i3 = local_8.wrapping_mul(3) as isize;
            let i4 = local_8.wrapping_mul(4) as isize;
            mech_trace::wr!(
                "10045873",
                d0.offset(i4),
                write,
                mech_trace::rd!("10045873", s0.offset(i3), read).wrapping_mul(4)
            );
            mech_trace::wr!(
                "10045873",
                d1.offset(i4),
                write,
                mech_trace::rd!("10045873", s1.offset(i3), read).wrapping_mul(4)
            );
            mech_trace::wr!(
                "10045873",
                d2.offset(i4),
                write,
                mech_trace::rd!("10045873", s2.offset(i3), read).wrapping_mul(4)
            );
            mech_trace::wr!("10045873", d3.offset(i4), write, 4);
            local_8 = local_8.wrapping_add(1);
        }
        if mech_trace::gr!("10045873", "100afbc4", DAT_100afbc4) == 0 {
            mech_trace::gw!(
                "10045873",
                "100afbc4",
                DAT_100afbc4,
                mech_trace::cr!(
                    "10045873",
                    "CreatePalette",
                    CreatePalette((&raw mut DAT_100affe0).cast::<c_void>())
                ) as usize as u32
            );
            mech_trace::gw!(
                "10045873",
                "10160bf4",
                DAT_10160bf4,
                mech_trace::cr!(
                    "10045873",
                    "SelectPalette",
                    SelectPalette(
                        mech_trace::gr!("10045873", "100afbc0", DAT_100afbc0),
                        mech_trace::gr!("10045873", "100afbc4", DAT_100afbc4) as usize
                            as *mut c_void,
                        0
                    )
                ) as usize as u32
            );
        } else {
            mech_trace::cr!(
                "10045873",
                "SetPaletteEntries",
                SetPaletteEntries(
                    mech_trace::gr!("10045873", "100afbc4", DAT_100afbc4) as usize as *mut c_void,
                    0,
                    0x100,
                    (&raw mut DAT_100affe4).cast::<c_void>(),
                )
            );
        }
        mech_trace::cr!(
            "10045873",
            "RealizePalette",
            RealizePalette(mech_trace::gr!("10045873", "100afbc0", DAT_100afbc0))
        );
    }
    mech_trace::rt!("10045873", 1)
}
