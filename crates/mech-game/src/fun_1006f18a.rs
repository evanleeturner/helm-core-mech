// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100b38dc: i32;
    static mut DAT_100aa1ac: i32;
    static mut DAT_100b0710: i32;
    static mut DAT_101600f4: u8;
    static mut DAT_100ef034: u8;
    static mut DAT_100ef040: u8;
    static mut _DAT_10165888: u16;
    static mut _DAT_1016588a: u16;
    static mut _DAT_1016588c: u16;
    static mut _DAT_1016588e: u16;
    static mut _DAT_10165890: u16;
    static mut _DAT_10165892: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> i32;
    fn FUN_1004d60d(param_1: i32) -> i32;
    fn FUN_1000dd4e(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006f18a() {
    mech_trace::enter!("1006f18a");
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table pointers are indexed and dereferenced exactly as the original does.
    unsafe {
        let mut local_c: i32 = 0;
        while local_c < mech_trace::gr!("1006f18a", "100b38d8", DAT_100b38d8) {
            if mech_trace::gr!("1006f18a", "100aa1ac", DAT_100aa1ac) != 0 {
                let off = (local_c as u32).wrapping_mul(0x3c) as usize;
                let p = (&raw mut DAT_101600f4).byte_add(off).cast::<u32>();
                if mech_trace::gr!("1006f18a", "100b0710", DAT_100b0710) == local_c {
                    mech_trace::wr!("1006f18a", p, write_unaligned, 0);
                } else {
                    mech_trace::wr!("1006f18a", p, write_unaligned, 1);
                }
            }
            let i_var1 = mech_trace::cr!("1006f18a", "1004d5e3", FUN_1004d5e3(local_c));
            if i_var1 == 0 {
                mech_trace::gw!(
                    "1006f18a",
                    "_DAT_10165888",
                    _DAT_10165888,
                    mech_trace::gr!("1006f18a", "_DAT_10165888", _DAT_10165888).wrapping_add(1)
                );
            } else if i_var1 == 1 {
                mech_trace::gw!(
                    "1006f18a",
                    "_DAT_1016588c",
                    _DAT_1016588c,
                    mech_trace::gr!("1006f18a", "_DAT_1016588c", _DAT_1016588c).wrapping_add(1)
                );
            } else if i_var1 == 2 {
                mech_trace::gw!(
                    "1006f18a",
                    "_DAT_1016588a",
                    _DAT_1016588a,
                    mech_trace::gr!("1006f18a", "_DAT_1016588a", _DAT_1016588a).wrapping_add(1)
                );
            }
            local_c = local_c.wrapping_add(1);
        }
        local_c = 0;
        while local_c < mech_trace::gr!("1006f18a", "100b38dc", DAT_100b38dc) {
            let i_var1 = mech_trace::cr!("1006f18a", "1004d60d", FUN_1004d60d(local_c));
            if i_var1 == 0 {
                mech_trace::gw!(
                    "1006f18a",
                    "_DAT_1016588e",
                    _DAT_1016588e,
                    mech_trace::gr!("1006f18a", "_DAT_1016588e", _DAT_1016588e).wrapping_add(1)
                );
            } else if i_var1 == 1 {
                mech_trace::gw!(
                    "1006f18a",
                    "_DAT_10165892",
                    _DAT_10165892,
                    mech_trace::gr!("1006f18a", "_DAT_10165892", _DAT_10165892).wrapping_add(1)
                );
            } else if i_var1 == 2 {
                mech_trace::gw!(
                    "1006f18a",
                    "_DAT_10165890",
                    _DAT_10165890,
                    mech_trace::gr!("1006f18a", "_DAT_10165890", _DAT_10165890).wrapping_add(1)
                );
            }
            let off = (local_c as u32).wrapping_mul(0x40) as usize;
            let arg = mech_trace::rd!(
                "1006f18a",
                (&raw mut DAT_100ef034).byte_add(off).cast::<i32>(),
                read_unaligned
            );
            let i_var1 = mech_trace::cr!("1006f18a", "1000dd4e", FUN_1000dd4e(arg));
            if i_var1 != 0 {
                let inner = mech_trace::rd!(
                    "1006f18a",
                    (i_var1.wrapping_add(0x6c) as u32 as usize as *const i32),
                    read_unaligned
                );
                if inner != 0 {
                    let v = mech_trace::rd!(
                        "1006f18a",
                        (inner.wrapping_add(0x40) as u32 as usize as *const u32),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "1006f18a",
                        (&raw mut DAT_100ef040).byte_add(off).cast::<u32>(),
                        write_unaligned,
                        v
                    );
                }
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
