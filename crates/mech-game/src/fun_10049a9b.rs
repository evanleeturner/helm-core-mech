// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101ec754: u8;
    static mut DAT_101ec758: u8;
    static mut DAT_101ec75c: u8;
    static mut DAT_101ec760: u8;
    static mut DAT_101ec764: u8;
    static mut DAT_101ec768: u8;
    static mut DAT_101ec76c: u8;
    static mut DAT_101ec770: u8;
    static mut DAT_101ec774: u8;
    static mut DAT_101ec778: u8;
    static mut DAT_101ec77c: u8;
    static mut DAT_101ec784: u8;
    static mut DAT_101ec788: u8;
    static mut DAT_101ec78c: u8;
    static mut DAT_101ec790: u8;
    static mut DAT_101ec798: u8;
    static mut DAT_101ec79c: u8;
    static mut DAT_101ec7a0: u8;
    static mut DAT_101ec7a4: u8;
    static mut DAT_101ec7a8: u8;
    static mut DAT_101ec7ac: u8;
}

#[inline(always)]
fn slot(base: *mut u8, off: i32) -> *mut u32 {
    (base as i32).wrapping_add(off) as *mut u32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049a9b(param_1: i32) {
    mech_trace::enter!("10049a9b", param_1);
    let i_var1 = param_1.wrapping_mul(0x60);
    // SAFETY: the globals are the game's own arrays; each slot is the
    // global's address plus the record offset, written where the original does.
    unsafe {
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec764, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec760, i_var1),
            write_unaligned,
            mech_trace::rd!(
                "10049a9b",
                slot(&raw mut DAT_101ec764, i_var1),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec75c, i_var1),
            write_unaligned,
            mech_trace::rd!(
                "10049a9b",
                slot(&raw mut DAT_101ec760, i_var1),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec770, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec76c, i_var1),
            write_unaligned,
            mech_trace::rd!(
                "10049a9b",
                slot(&raw mut DAT_101ec770, i_var1),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec768, i_var1),
            write_unaligned,
            mech_trace::rd!(
                "10049a9b",
                slot(&raw mut DAT_101ec76c, i_var1),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec778, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec774, i_var1),
            write_unaligned,
            mech_trace::rd!(
                "10049a9b",
                slot(&raw mut DAT_101ec778, i_var1),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec77c, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec758, i_var1),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec784, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec788, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec78c, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec790, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec79c, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec798, i_var1),
            write_unaligned,
            mech_trace::rd!(
                "10049a9b",
                slot(&raw mut DAT_101ec79c, i_var1),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec7a0, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec7a4, i_var1),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec754, i_var1),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec7a8, i_var1),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10049a9b",
            slot(&raw mut DAT_101ec7ac, i_var1),
            write_unaligned,
            0xffffffff
        );
    }
}
