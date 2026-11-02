// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100af0b8: *mut i32;
    static mut DAT_10114e18: i32;
    static mut DAT_100eebb0: [u8; 0];
    static mut _DAT_100a1140: f64;
    static mut DAT_100ada38: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(a: i32, b: i32, c: *mut i32) -> *mut u16;
    fn FUN_1005aae4(a: i32) -> i32;
    fn FUN_1005acaa(a: *mut i32, b: i32, c: i32, d: i32, e: *mut u8, f: i32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1008664f(a: i32, b: *mut i32);
    fn FUN_100169fb(a: i32);
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the address is a game-owned pointer value read as the original does.
    unsafe { mech_trace::rd!("1002eded", (addr as *const i32), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002eded(param_1: i32) {
    mech_trace::enter!("1002eded", param_1);
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed in the same order as the original.
    unsafe {
        let tbl = (&raw mut DAT_100eef40).cast::<i32>();
        let pi_var1 = rd(mech_trace::rd!(
            "1002eded",
            tbl.offset(mech_trace::gr!("1002eded", "100adf38", DAT_100adf38) as isize),
            read_unaligned
        )
        .wrapping_add(0x20));
        if mech_trace::rd!(
            "1002eded",
            ((param_1.wrapping_add(4)) as *const i16),
            read_unaligned
        ) != 0
        {
            let puvar2 = mech_trace::cr!(
                "1002eded",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1002eded", "100af17c", DAT_100af17c),
                    mech_trace::gr!("1002eded", "101686f0", DAT_101686f0).wrapping_add(1),
                    mech_trace::gr!("1002eded", "100af0b8", DAT_100af0b8)
                )
            );
            if !puvar2.is_null() {
                let local_8: i32 = (rd(rd(pi_var1).wrapping_add(0x54)) as u32)
                    .wrapping_sub(rd(pi_var1.wrapping_add(0x400)) as u32)
                    .wrapping_mul(1000) as i32;
                let f = mech_trace::rd!(
                    "1002eded",
                    (param_1.wrapping_add(0xcc) as *const usize),
                    read_unaligned
                );
                let f: extern "C" fn(i32) = core::mem::transmute(f);
                mech_trace::cr!("1002eded", "ind", f(param_1));
                let mut pbvar3 = param_1.wrapping_add(0x10) as *mut u8;
                let mut puvar8 = (&raw mut DAT_100eebb0).cast::<u8>() as i32;
                let mut puvar7 = puvar2 as i32;
                let mut i_var4 =
                    mech_trace::cr!("1002eded", "1005aae4", FUN_1005aae4(puvar2 as i32));
                let mut i_var5 =
                    mech_trace::cr!("1002eded", "1005aae4", FUN_1005aae4(puvar2 as i32));
                let a_base =
                    rd(mech_trace::gr!("1002eded", "10114e18", DAT_10114e18).wrapping_add(0xa8));
                FUN_1005acaa(
                    rd(param_1.wrapping_add(0xa4)) as *mut i32,
                    rd(a_base),
                    (i_var4 as u32)
                        .wrapping_add(i_var5 as u32)
                        .wrapping_add(rd(a_base.wrapping_add(4)) as u32) as i32,
                    puvar7,
                    pbvar3,
                    puvar8,
                );
                let mut local_48 = [0u8; 64];
                mech_trace::cr!(
                    "1002eded",
                    "_sprintf",
                    _sprintf(
                        local_48.as_mut_ptr().cast::<i8>(),
                        (&raw const DAT_100ada38).cast::<i8>(),
                        (local_8 as f64)
                            / mech_trace::gr!("1002eded", "_DAT_100a1140", _DAT_100a1140),
                    )
                );
                pbvar3 = local_48.as_mut_ptr();
                puvar8 = (&raw mut DAT_100eebb0).cast::<u8>() as i32;
                puvar7 = puvar2 as i32;
                i_var4 = mech_trace::cr!("1002eded", "1005aae4", FUN_1005aae4(puvar2 as i32));
                i_var5 = mech_trace::cr!("1002eded", "1005aae4", FUN_1005aae4(puvar2 as i32));
                let i_var6 = mech_trace::cr!("1002eded", "1005aae4", FUN_1005aae4(puvar2 as i32));
                let a_base =
                    rd(mech_trace::gr!("1002eded", "10114e18", DAT_10114e18).wrapping_add(0xa8));
                FUN_1005acaa(
                    rd(param_1.wrapping_add(0xa4)) as *mut i32,
                    rd(a_base),
                    (i_var4 as u32)
                        .wrapping_add(i_var5 as u32)
                        .wrapping_add(i_var6 as u32)
                        .wrapping_add(rd(a_base.wrapping_add(4)) as u32) as i32,
                    puvar7,
                    pbvar3,
                    puvar8,
                );
                let cur = mech_trace::rd!(
                    "1002eded",
                    tbl.offset(mech_trace::gr!("1002eded", "100adf38", DAT_100adf38) as isize),
                    read_unaligned
                );
                if -1 < rd(rd(cur.wrapping_add(0x20)).wrapping_add(0x3f4)) {
                    let f = mech_trace::rd!(
                        "1002eded",
                        (param_1.wrapping_add(0xcc) as *const usize),
                        read_unaligned
                    );
                    let f: extern "C" fn(i32) = core::mem::transmute(f);
                    mech_trace::cr!("1002eded", "ind", f(param_1));
                    let a_base =
                        rd(mech_trace::gr!("1002eded", "10114e18", DAT_10114e18)
                            .wrapping_add(0xa8));
                    FUN_1005acaa(
                        rd(param_1.wrapping_add(0xa4)) as *mut i32,
                        rd(a_base),
                        rd(a_base.wrapping_add(4)),
                        puvar2 as i32,
                        param_1.wrapping_add(0x10) as *mut u8,
                        (&raw mut DAT_100eebb0).cast::<u8>() as i32,
                    );
                    FUN_1008664f(
                        mech_trace::gr!("1002eded", "101686f0", DAT_101686f0).wrapping_add(1),
                        mech_trace::gr!("1002eded", "100af0b8", DAT_100af0b8),
                    );
                    FUN_100169fb(param_1);
                }
            }
        }
    }
}
