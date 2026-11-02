// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef030: [u8; 0];
    static mut DAT_100b3788: i32;
    static mut DAT_100b06fc: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_1016585d: [u8; 0];
    static mut DAT_1016585f: [u8; 0];
    static mut DAT_10165861: [u8; 0];
    static mut DAT_10165874: u16;
    static mut DAT_10165876: u16;
    static mut DAT_10165878: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d60d(param_1: u32) -> i32;
    fn FUN_1004fed5();
    fn FUN_1000d59a(param_1: i32);
}

unsafe fn inc16_unaligned(p: *mut u8) {
    // SAFETY: p points at a game-owned 2-byte counter.
    unsafe {
        let v = mech_trace::rd!("1004bbc3", p.cast::<u16>(), read_unaligned);
        mech_trace::wr!(
            "1004bbc3",
            p.cast::<u16>(),
            write_unaligned,
            v.wrapping_add(1)
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004bbc3(param_1: u32) {
    mech_trace::enter!("1004bbc3", param_1);
    // SAFETY: globals and callees are the game's own memory; pointers are
    // computed and dereferenced exactly as the original does.
    unsafe {
        let puvar1: *mut u8 = (&raw mut DAT_100ef030)
            .cast::<u8>()
            .byte_add(param_1.wrapping_mul(0x40) as usize);
        if mech_trace::gr!("1004bbc3", "100b3788", DAT_100b3788) != 0
            && (mech_trace::rd!(
                "1004bbc3",
                (mech_trace::gr!("1004bbc3", "100b3788", DAT_100b3788).wrapping_add(2)
                    as *const u16),
                read_unaligned
            ) & 0x200)
                != 0
            && mech_trace::rd!(
                "1004bbc3",
                (mech_trace::gr!("1004bbc3", "100b3788", DAT_100b3788).wrapping_add(0x14)
                    as *const u16),
                read_unaligned
            ) as u32
                == param_1
        {
            mech_trace::gw!("1004bbc3", "100b3788", DAT_100b3788, 0);
        }
        if (mech_trace::rd!("1004bbc3", puvar1.cast::<u16>(), read_unaligned) & 4) == 0 {
            if -1 < mech_trace::gr!("1004bbc3", "100b06fc", DAT_100b06fc) {
                if mech_trace::gr!("1004bbc3", "100b06fc", DAT_100b06fc)
                    == mech_trace::gr!("1004bbc3", "100adf38", DAT_100adf38)
                {
                    let i_var2 = mech_trace::cr!("1004bbc3", "1004d60d", FUN_1004d60d(param_1));
                    if i_var2 == 0 {
                        inc16_unaligned((&raw mut DAT_10165861).cast::<u8>());
                    } else if i_var2 == 1 {
                        inc16_unaligned((&raw mut DAT_1016585d).cast::<u8>());
                    } else if i_var2 == 2 {
                        inc16_unaligned((&raw mut DAT_1016585f).cast::<u8>());
                    }
                }
                let base = (&raw mut DAT_100eef40).cast::<i32>();
                let a = mech_trace::rd!(
                    "1004bbc3",
                    base.wrapping_offset(
                        mech_trace::gr!("1004bbc3", "100adf38", DAT_100adf38) as isize
                    ),
                    read_unaligned
                );
                let b = mech_trace::rd!(
                    "1004bbc3",
                    base.wrapping_offset(
                        mech_trace::gr!("1004bbc3", "100b06fc", DAT_100b06fc) as isize
                    ),
                    read_unaligned
                );
                if mech_trace::rd!(
                    "1004bbc3",
                    (a.wrapping_add(8) as *const i32),
                    read_unaligned
                ) == mech_trace::rd!(
                    "1004bbc3",
                    (b.wrapping_add(8) as *const i32),
                    read_unaligned
                ) {
                    let i_var2 = mech_trace::cr!("1004bbc3", "1004d60d", FUN_1004d60d(param_1));
                    if i_var2 == 0 {
                        mech_trace::gw!(
                            "1004bbc3",
                            "10165878",
                            DAT_10165878,
                            mech_trace::gr!("1004bbc3", "10165878", DAT_10165878).wrapping_add(1)
                        );
                    } else if i_var2 == 1 {
                        mech_trace::gw!(
                            "1004bbc3",
                            "10165874",
                            DAT_10165874,
                            mech_trace::gr!("1004bbc3", "10165874", DAT_10165874).wrapping_add(1)
                        );
                    } else if i_var2 == 2 {
                        mech_trace::gw!(
                            "1004bbc3",
                            "10165876",
                            DAT_10165876,
                            mech_trace::gr!("1004bbc3", "10165876", DAT_10165876).wrapping_add(1)
                        );
                    }
                }
            }
            if (mech_trace::rd!("1004bbc3", puvar1.cast::<u16>(), read_unaligned) & 0x40) != 0 {
                FUN_1004fed5();
            }
            FUN_1000d59a(puvar1 as usize as i32);
        }
    }
}
