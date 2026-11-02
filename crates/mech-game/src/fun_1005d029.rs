// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b3188: [u8; 0];
    static mut DAT_100b3220: [u8; 0];
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_10114db0: [u32; 0];
    static mut DAT_10114e40: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005c34a(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005d029() {
    mech_trace::enter!("1005d029");
    // SAFETY: the globals, objects and callees are the game's own memory;
    // reads and calls occur in the original's order.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let entry = mech_trace::rd!(
            "1005d029",
            base.add(mech_trace::gr!("1005d029", "100adf38", DAT_100adf38) as usize),
            read_unaligned
        );
        mech_trace::cr!(
            "1005d029",
            "1005c34a",
            FUN_1005c34a(mech_trace::rd!(
                "1005d029",
                (entry.wrapping_add(0x20) as *const i32),
                read_unaligned
            ))
        );

        let objs = (&raw mut DAT_10114db0).cast::<u32>();
        let a220 = (&raw mut DAT_100b3220).cast::<u8>();
        let a188 = (&raw mut DAT_100b3188).cast::<u8>();
        let ae40 = (&raw mut DAT_10114e40).cast::<u8>();

        let mut i: i32 = 0;
        while i < 0x22 {
            let idx = i as usize;
            let off = idx.wrapping_mul(4);

            let obj = mech_trace::rd!("1005d029", objs.add(idx), read_unaligned);
            let f1: unsafe extern "C" fn(u32, u32) = core::mem::transmute(mech_trace::rd!(
                "1005d029",
                (obj.wrapping_add(0xc4) as *const u32),
                read_unaligned
            ));
            let arg1 = mech_trace::rd!("1005d029", a220.add(off).cast::<u32>(), read_unaligned);
            mech_trace::cr!(
                "1005d029",
                "ind",
                f1(
                    mech_trace::rd!("1005d029", objs.add(idx), read_unaligned),
                    arg1
                )
            );

            let obj = mech_trace::rd!("1005d029", objs.add(idx), read_unaligned);
            let f2: unsafe extern "C" fn(u32, u32) = core::mem::transmute(mech_trace::rd!(
                "1005d029",
                (obj.wrapping_add(0xdc) as *const u32),
                read_unaligned
            ));
            let arg2 = mech_trace::rd!("1005d029", a188.add(off).cast::<u32>(), read_unaligned);
            mech_trace::cr!(
                "1005d029",
                "ind",
                f2(
                    mech_trace::rd!("1005d029", objs.add(idx), read_unaligned),
                    arg2
                )
            );

            let obj = mech_trace::rd!("1005d029", objs.add(idx), read_unaligned);
            let f3: unsafe extern "C" fn(u32, i32) = core::mem::transmute(mech_trace::rd!(
                "1005d029",
                (obj.wrapping_add(0xe4) as *const u32),
                read_unaligned
            ));
            mech_trace::cr!(
                "1005d029",
                "ind",
                f3(
                    mech_trace::rd!("1005d029", objs.add(idx), read_unaligned),
                    0
                )
            );

            if mech_trace::rd!("1005d029", ae40.add(off).cast::<i32>(), read_unaligned) != 0 {
                let obj = mech_trace::rd!("1005d029", objs.add(idx), read_unaligned);
                let f4: unsafe extern "C" fn(u32) = core::mem::transmute(mech_trace::rd!(
                    "1005d029",
                    (obj.wrapping_add(0xe8) as *const u32),
                    read_unaligned
                ));
                mech_trace::cr!(
                    "1005d029",
                    "ind",
                    f4(mech_trace::rd!("1005d029", objs.add(idx), read_unaligned))
                );
            }

            i = i.wrapping_add(1);
        }
    }
}
