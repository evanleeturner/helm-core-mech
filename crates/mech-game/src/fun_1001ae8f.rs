// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab07c: i32;
    static mut DAT_100b38fc: i32;
    static mut _DAT_101eb450: i32;
    static mut _DAT_101eb458: i32;
    static mut _DAT_101eb3a0: i32;
    static mut _DAT_101eb3a8: i32;
    static mut DAT_100ab094: i32;
    static mut DAT_100ab090: i32;
    static mut DAT_100ba9ce: i32;
    static mut DAT_100ba9cf: i32;
    static mut DAT_100ba9d0: i32;
    static mut DAT_100ba9d1: i32;
    static mut DAT_100ab0bc: i32;
    static mut DAT_100ba9ac: i32;
    static mut DAT_100ba9a8: i32;
    static mut DAT_100aaf68: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e938();
    fn FUN_1001a635(param_1: i32);
    fn FUN_1001b024(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003e5c0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001ae8f() {
    mech_trace::enter!("1001ae8f");
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointer in DAT_100aaf68 is the game's own structure.
    unsafe {
        if mech_trace::gr!("1001ae8f", "100ab07c", DAT_100ab07c) != 0 {
            mech_trace::gw!(
                "1001ae8f",
                "_DAT_101eb450",
                _DAT_101eb450,
                mech_trace::gr!("1001ae8f", "100b38fc", DAT_100b38fc)
            );
            mech_trace::gw!("1001ae8f", "_DAT_101eb458", _DAT_101eb458, 0);
            mech_trace::gw!(
                "1001ae8f",
                "_DAT_101eb3a0",
                _DAT_101eb3a0,
                mech_trace::gr!("1001ae8f", "100b38fc", DAT_100b38fc)
            );
            mech_trace::gw!("1001ae8f", "_DAT_101eb3a8", _DAT_101eb3a8, 0);
            FUN_1003e938();
            FUN_1001a635(0);
        }
        mech_trace::gw!("1001ae8f", "100ab094", DAT_100ab094, 1);
        mech_trace::gw!("1001ae8f", "100ab090", DAT_100ab090, 0);
        if mech_trace::gr!("1001ae8f", "100ba9ce", DAT_100ba9ce) == 0 {
            if mech_trace::gr!("1001ae8f", "100ba9cf", DAT_100ba9cf) == 0 {
                if mech_trace::gr!("1001ae8f", "100ba9d0", DAT_100ba9d0) == 0 {
                    if mech_trace::gr!("1001ae8f", "100ba9d1", DAT_100ba9d1) == 0 {
                        if mech_trace::gr!("1001ae8f", "100ab0bc", DAT_100ab0bc) == 0 {
                            mech_trace::gw!("1001ae8f", "100ba9ac", DAT_100ba9ac, 0);
                            mech_trace::gw!("1001ae8f", "100ba9a8", DAT_100ba9a8, 0);
                        }
                    } else {
                        mech_trace::gw!("1001ae8f", "100ba9a8", DAT_100ba9a8, 0x280000);
                    }
                } else {
                    mech_trace::gw!("1001ae8f", "100ba9a8", DAT_100ba9a8, 0xffce0000u32 as i32);
                }
            } else {
                mech_trace::gw!("1001ae8f", "100ba9ac", DAT_100ba9ac, 0x460000);
            }
        } else {
            mech_trace::gw!("1001ae8f", "100ba9ac", DAT_100ba9ac, 0xffba0000u32 as i32);
        }
        if mech_trace::gr!("1001ae8f", "100ba9cf", DAT_100ba9cf) == 0
            && mech_trace::gr!("1001ae8f", "100ba9ce", DAT_100ba9ce) == 0
            && mech_trace::gr!("1001ae8f", "100ba9d0", DAT_100ba9d0) == 0
            && mech_trace::gr!("1001ae8f", "100ba9d1", DAT_100ba9d1) == 0
        {
            mech_trace::gw!("1001ae8f", "100ab0bc", DAT_100ab0bc, 1);
        } else {
            mech_trace::gw!("1001ae8f", "100ab0bc", DAT_100ab0bc, 0);
        }
        let i_var1 = mech_trace::cr!("1001ae8f", "1003e5c0", FUN_1003e5c0());
        if i_var1 == 0 {
            let p = mech_trace::gr!("1001ae8f", "100aaf68", DAT_100aaf68);
            FUN_1001b024(
                p.byte_add(0x10).cast::<i32>(),
                p.byte_add(0xc).cast::<i32>(),
                p.byte_add(0x14).cast::<i32>(),
                p.cast::<i32>(),
                p.byte_add(4).cast::<i32>(),
                p.byte_add(8).cast::<i32>(),
            );
        }
    }
}
