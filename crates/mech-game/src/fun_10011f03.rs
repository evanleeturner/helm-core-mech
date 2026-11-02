// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf98: u32;
    static mut DAT_100aa1d8: *mut i32;
    static mut DAT_101eb4d0: [i32; 8];
    static mut DAT_101eb520: [u8; 8];
    static mut DAT_101eb510: [u8; 8];
    static mut DAT_101658a0: u16;
    static mut DAT_100b38d8: u32;
    static mut DAT_100aa1e0: u32;
    static mut DAT_100aa1dc: *mut u32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100aa1e4: *mut u32;
    static mut DAT_101eb51c: u32;
    static mut DAT_100aa1e8: *mut u32;
    static mut DAT_100aa1ec: *mut u32;
    static mut DAT_101eb4c4: *mut core::ffi::c_void;
    static mut DAT_101eb4c0: *mut core::ffi::c_void;
    static mut DAT_101eb4f0: [i32; 8];
    static mut DAT_101eb530: [i32; 8];
    static mut DAT_100adf28: i32;
    static mut DAT_100adf2c: i32;
    static mut DAT_100aa20c: i32;
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        h_heap: *mut core::ffi::c_void,
        dw_flags: u32,
        dw_bytes: u32,
    ) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10013be9(param_1: *mut i32) -> i32;
    fn FUN_10011e80(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011f03(param_1: *mut i32) {
    mech_trace::enter!("10011f03", param_1);
    // SAFETY: globals and callees are the game's own; the heap blocks are
    // freshly allocated with sizes covering the zeroing writes below.
    unsafe {
        if mech_trace::gr!("10011f03", "100aa1ac", DAT_100aa1ac) == 0 {
            mech_trace::gu!("10011f03", "100adf98", DAT_100adf98, |=, 1);
        } else {
            mech_trace::gw!("10011f03", "100aa1d8", DAT_100aa1d8, param_1);
            let mut local_8: usize = 0;
            while local_8 < 8 {
                mech_trace::gw!("10011f03", "101eb4d0", DAT_101eb4d0[local_8], 0);
                mech_trace::gw!("10011f03", "101eb520", DAT_101eb520[local_8], 0);
                mech_trace::gw!("10011f03", "101eb510", DAT_101eb510[local_8], 0);
                local_8 += 1;
            }
            mech_trace::gw!(
                "10011f03",
                "101658a0",
                DAT_101658a0,
                mech_trace::gr!("10011f03", "100b38d8", DAT_100b38d8) as u16
            );
            mech_trace::gw!("10011f03", "100aa1e0", DAT_100aa1e0, 0x85);
            mech_trace::gw!(
                "10011f03",
                "100aa1dc",
                DAT_100aa1dc,
                mech_trace::cr!(
                    "10011f03",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("10011f03", "100adf74", DAT_100adf74),
                        1,
                        0x85
                    )
                ) as *mut u32
            );
            let mut u_var2 = mech_trace::gr!("10011f03", "100aa1e0", DAT_100aa1e0);
            let mut puvar4 = mech_trace::gr!("10011f03", "100aa1dc", DAT_100aa1dc);
            let mut u_var1 = mech_trace::gr!("10011f03", "100aa1e0", DAT_100aa1e0) >> 2;
            while u_var1 != 0 {
                mech_trace::wr!("10011f03", puvar4, write_unaligned, 0);
                puvar4 = puvar4.add(1);
                u_var1 = u_var1.wrapping_sub(1);
            }
            u_var2 &= 3;
            while u_var2 != 0 {
                mech_trace::wr!("10011f03", (puvar4 as *mut u8), write, 0);
                puvar4 = (puvar4 as *mut u8).add(1) as *mut u32;
                u_var2 = u_var2.wrapping_sub(1);
            }
            mech_trace::gw!(
                "10011f03",
                "100aa1e4",
                DAT_100aa1e4,
                mech_trace::gr!("10011f03", "100aa1dc", DAT_100aa1dc)
            );
            mech_trace::gw!("10011f03", "101eb51c", DAT_101eb51c, 0x27);
            mech_trace::gw!(
                "10011f03",
                "100aa1e8",
                DAT_100aa1e8,
                mech_trace::cr!(
                    "10011f03",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("10011f03", "100adf74", DAT_100adf74),
                        1,
                        0x27
                    )
                ) as *mut u32
            );
            u_var2 = mech_trace::gr!("10011f03", "101eb51c", DAT_101eb51c);
            puvar4 = mech_trace::gr!("10011f03", "100aa1e8", DAT_100aa1e8);
            u_var1 = mech_trace::gr!("10011f03", "101eb51c", DAT_101eb51c) >> 2;
            while u_var1 != 0 {
                mech_trace::wr!("10011f03", puvar4, write_unaligned, 0);
                puvar4 = puvar4.add(1);
                u_var1 = u_var1.wrapping_sub(1);
            }
            u_var2 &= 3;
            while u_var2 != 0 {
                mech_trace::wr!("10011f03", (puvar4 as *mut u8), write, 0);
                puvar4 = (puvar4 as *mut u8).add(1) as *mut u32;
                u_var2 = u_var2.wrapping_sub(1);
            }
            mech_trace::gw!(
                "10011f03",
                "100aa1ec",
                DAT_100aa1ec,
                mech_trace::cr!(
                    "10011f03",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("10011f03", "100adf74", DAT_100adf74),
                        1,
                        0x52
                    )
                ) as *mut u32
            );
            puvar4 = mech_trace::gr!("10011f03", "100aa1ec", DAT_100aa1ec);
            let mut i_var3: i32 = 0x14;
            while i_var3 != 0 {
                mech_trace::wr!("10011f03", puvar4, write_unaligned, 0);
                puvar4 = puvar4.add(1);
                i_var3 = i_var3.wrapping_sub(1);
            }
            mech_trace::wr!("10011f03", (puvar4 as *mut u16), write_unaligned, 0);
            mech_trace::gw!(
                "10011f03",
                "101eb4c4",
                DAT_101eb4c4,
                mech_trace::cr!(
                    "10011f03",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("10011f03", "100adf74", DAT_100adf74),
                        1,
                        0x100
                    )
                )
            );
            mech_trace::gw!(
                "10011f03",
                "101eb4c0",
                DAT_101eb4c0,
                mech_trace::cr!(
                    "10011f03",
                    "HeapAlloc",
                    HeapAlloc(
                        mech_trace::gr!("10011f03", "100adf74", DAT_100adf74),
                        1,
                        0x100
                    )
                )
            );
            local_8 = 0;
            while local_8 < 8 {
                mech_trace::gw!("10011f03", "101eb4f0", DAT_101eb4f0[local_8], 0);
                mech_trace::gw!("10011f03", "101eb530", DAT_101eb530[local_8], 0);
                local_8 += 1;
            }
            i_var3 = mech_trace::cr!("10011f03", "10013be9", FUN_10013be9(param_1));
            if i_var3 == 0 {
                mech_trace::gw!("10011f03", "100adf28", DAT_100adf28, 1);
                mech_trace::gw!(
                    "10011f03",
                    "100adf2c",
                    DAT_100adf2c,
                    mech_trace::gr!("10011f03", "100adf2c", DAT_100adf2c).wrapping_add(0x29a)
                );
            } else if mech_trace::gr!("10011f03", "100aa20c", DAT_100aa20c) != 0 {
                mech_trace::gw!(
                    "10011f03",
                    "100adf38",
                    DAT_100adf38,
                    mech_trace::cr!(
                        "10011f03",
                        "10011e80",
                        FUN_10011e80(mech_trace::gr!("10011f03", "100aa20c", DAT_100aa20c))
                    )
                );
            }
        }
    }
}
