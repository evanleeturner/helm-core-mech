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
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointer in DAT_100aaf68 is the game's own structure.
    unsafe {
        if DAT_100ab07c != 0 {
            _DAT_101eb450 = DAT_100b38fc;
            _DAT_101eb458 = 0;
            _DAT_101eb3a0 = DAT_100b38fc;
            _DAT_101eb3a8 = 0;
            FUN_1003e938();
            FUN_1001a635(0);
        }
        DAT_100ab094 = 1;
        DAT_100ab090 = 0;
        if DAT_100ba9ce == 0 {
            if DAT_100ba9cf == 0 {
                if DAT_100ba9d0 == 0 {
                    if DAT_100ba9d1 == 0 {
                        if DAT_100ab0bc == 0 {
                            DAT_100ba9ac = 0;
                            DAT_100ba9a8 = 0;
                        }
                    } else {
                        DAT_100ba9a8 = 0x280000;
                    }
                } else {
                    DAT_100ba9a8 = 0xffce0000u32 as i32;
                }
            } else {
                DAT_100ba9ac = 0x460000;
            }
        } else {
            DAT_100ba9ac = 0xffba0000u32 as i32;
        }
        if DAT_100ba9cf == 0 && DAT_100ba9ce == 0 && DAT_100ba9d0 == 0 && DAT_100ba9d1 == 0 {
            DAT_100ab0bc = 1;
        } else {
            DAT_100ab0bc = 0;
        }
        let i_var1 = FUN_1003e5c0();
        if i_var1 == 0 {
            let p = DAT_100aaf68;
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
