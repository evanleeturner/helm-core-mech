// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_101686f0: u32;
    static mut PTR_DAT_100af0b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f47f(param_1: i32) {
    mech_trace::enter!("1000f47f", param_1);
    // SAFETY: param_1 is the caller's object pointer and the globals and
    // callees are the game's own; reads and calls happen in the original order.
    unsafe {
        let i_var1 = mech_trace::gr!("1000f47f", "100b38fc", DAT_100b38fc) as i32;
        let p = param_1 as u32 as usize;
        if (p + 4) as *const i16 as usize == 0 {
            return;
        }
        if mech_trace::rd!(
            "1000f47f",
            ((p.wrapping_add(4)) as *const i16),
            read_unaligned
        ) == 0
        {
            return;
        }
        let idx = mech_trace::rd!(
            "1000f47f",
            ((p.wrapping_add(0xc)) as *const i32),
            read_unaligned
        );
        if idx < 0 {
            return;
        }
        let e = mech_trace::rd!(
            "1000f47f",
            (&raw const DAT_100eef40).cast::<u32>().add(mech_trace::gr!(
                "1000f47f",
                "100adf38",
                DAT_100adf38
            ) as usize),
            read_unaligned
        );
        let a = mech_trace::rd!(
            "1000f47f",
            ((e.wrapping_add(0x20)) as *const u32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "1000f47f",
            ((a.wrapping_add(0x54)) as *const u32),
            read_unaligned
        );
        let addr = (idx.wrapping_mul(0x40) as u32)
            .wrapping_add(b)
            .wrapping_add(4);
        if mech_trace::rd!("1000f47f", (addr as *const i32), read_unaligned) < 0 {
            return;
        }
        let puv = mech_trace::cr!(
            "1000f47f",
            "1008668b",
            FUN_1008668b(
                mech_trace::gr!("1000f47f", "100af17c", DAT_100af17c) as i32,
                mech_trace::gr!("1000f47f", "101686f0", DAT_101686f0).wrapping_add(1),
                mech_trace::gr!("1000f47f", "PTR_DAT_100af0b8", PTR_DAT_100af0b8) as *mut i32,
            )
        );
        if puv.is_null() {
            return;
        }
        if mech_trace::rd!(
            "1000f47f",
            ((p.wrapping_add(4)) as *const i16),
            read_unaligned
        ) != 0
            && mech_trace::rd!(
                "1000f47f",
                ((p.wrapping_add(8)) as *const i32),
                read_unaligned
            ) < i_var1
        {
            mech_trace::cr!(
                "1000f47f",
                "1005acaa",
                FUN_1005acaa(
                    mech_trace::rd!(
                        "1000f47f",
                        ((p.wrapping_add(0xa4)) as *const u32),
                        read_unaligned
                    ) as *mut i32,
                    0,
                    0,
                    puv as u32 as i32,
                    (p.wrapping_add(0x10)) as *mut u8,
                    0x100eebb0,
                )
            );
        }
        mech_trace::cr!(
            "1000f47f",
            "1008664f",
            FUN_1008664f(
                mech_trace::gr!("1000f47f", "101686f0", DAT_101686f0).wrapping_add(1),
                mech_trace::gr!("1000f47f", "PTR_DAT_100af0b8", PTR_DAT_100af0b8) as *mut i32
            )
        );
    }
}
