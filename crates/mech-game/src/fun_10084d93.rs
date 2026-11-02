// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f80: *mut i32;
    static mut DAT_100b9a2c: *mut i32;
    static mut DAT_100b9f8c: i32;
    static mut DAT_100c9070: i32;
    static mut DAT_100c91f4: i32;
    static mut DAT_100c9060: i32;
    static mut DAT_100ba430: [i8; 1];
    static mut DAT_100ba454: [i8; 1];
    static mut DAT_100ba478: [i8; 1];
    static mut DAT_100ba498: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *const i8, ...);
}

/// Reads the vtable slot at byte offset `off` of the COM-style object `obj`.
unsafe fn vt(obj: *mut i32, off: usize) -> usize {
    // SAFETY: obj points at a live object whose first word is its vtable pointer.
    unsafe {
        let table = mech_trace::rd!("10084d93", obj, read_unaligned) as u32 as usize;
        mech_trace::rd!("10084d93", ((table + off) as *const usize), read_unaligned)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10084d93(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> u32 {
    mech_trace::enter!("10084d93", param_1, param_2, param_3, param_4);
    // SAFETY: the globals, vtables and callees are the game's own memory; the
    // calls are made in the original's order with the original's arguments.
    unsafe {
        if mech_trace::gr!("10084d93", "100b9f8c", DAT_100b9f8c) == 0 {
            mech_trace::gw!("10084d93", "100c9070", DAT_100c9070, 0);
        } else {
            let f: extern "stdcall" fn(*mut i32, i32) -> i32 = core::mem::transmute(vt(
                mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c),
                0x80,
            ));
            mech_trace::gw!(
                "10084d93",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084d93",
                    "ind",
                    f(
                        mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c),
                        mech_trace::gr!("10084d93", "100c91f4", DAT_100c91f4)
                    )
                )
            );
            mech_trace::wr!(
                "10084d93",
                mech_trace::gr!("10084d93", "100b9a2c", DAT_100b9a2c),
                write_unaligned,
                0
            );
            mech_trace::gw!("10084d93", "100b9f8c", DAT_100b9f8c, 0);
        }
        if mech_trace::gr!("10084d93", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw const DAT_100ba430) as *const i8,
                mech_trace::gr!("10084d93", "100c9070", DAT_100c9070),
            );
        }
        let f: extern "stdcall" fn(*mut i32) -> i32 = core::mem::transmute(vt(
            mech_trace::gr!("10084d93", "100b9f74", DAT_100b9f74),
            0x60,
        ));
        let i_var1 = mech_trace::cr!(
            "10084d93",
            "ind",
            f(mech_trace::gr!("10084d93", "100b9f74", DAT_100b9f74))
        );
        if i_var1 == -0x7789fe3e {
            let g: extern "stdcall" fn(*mut i32) -> i32 = core::mem::transmute(vt(
                mech_trace::gr!("10084d93", "100b9f74", DAT_100b9f74),
                0x6c,
            ));
            mech_trace::gw!(
                "10084d93",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084d93",
                    "ind",
                    g(mech_trace::gr!("10084d93", "100b9f74", DAT_100b9f74))
                )
            );
            let h: extern "stdcall" fn(*mut i32) -> i32 = core::mem::transmute(vt(
                mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c),
                0x60,
            ));
            let i_var1 = mech_trace::cr!(
                "10084d93",
                "ind",
                h(mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c))
            );
            if i_var1 == -0x7789fe3e {
                let k: extern "stdcall" fn(*mut i32) -> i32 = core::mem::transmute(vt(
                    mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c),
                    0x6c,
                ));
                mech_trace::gw!(
                    "10084d93",
                    "100c9070",
                    DAT_100c9070,
                    mech_trace::cr!(
                        "10084d93",
                        "ind",
                        k(mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c))
                    )
                );
            }
        } else {
            mech_trace::gw!("10084d93", "100c9070", DAT_100c9070, 0);
        }
        if mech_trace::gr!("10084d93", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw const DAT_100ba454) as *const i8,
                mech_trace::gr!("10084d93", "100c9070", DAT_100c9070),
            );
        }
        let mut local_14: [u32; 4] = [param_1, param_2, param_3, param_4];
        let b: extern "stdcall" fn(*mut i32, *mut i32, *mut i32, *mut u32, i32, i32) -> i32 =
            core::mem::transmute(vt(
                mech_trace::gr!("10084d93", "100b9f80", DAT_100b9f80),
                0x14,
            ));
        mech_trace::gw!(
            "10084d93",
            "100c9070",
            DAT_100c9070,
            mech_trace::cr!(
                "10084d93",
                "ind",
                b(
                    mech_trace::gr!("10084d93", "100b9f80", DAT_100b9f80),
                    &raw mut DAT_100c9060,
                    mech_trace::gr!("10084d93", "100b9f7c", DAT_100b9f7c),
                    local_14.as_mut_ptr(),
                    0x1000000,
                    0,
                )
            )
        );
        if mech_trace::gr!("10084d93", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw const DAT_100ba478) as *const i8,
                mech_trace::gr!("10084d93", "100c9070", DAT_100c9070),
            );
        }
        let fl: extern "stdcall" fn(*mut i32, *mut i32, i32) -> i32 = core::mem::transmute(vt(
            mech_trace::gr!("10084d93", "100b9f74", DAT_100b9f74),
            0x2c,
        ));
        mech_trace::gw!(
            "10084d93",
            "100c9070",
            DAT_100c9070,
            mech_trace::cr!(
                "10084d93",
                "ind",
                fl(
                    mech_trace::gr!("10084d93", "100b9f74", DAT_100b9f74),
                    mech_trace::gr!("10084d93", "100b9f80", DAT_100b9f80),
                    1
                )
            )
        );
        if mech_trace::gr!("10084d93", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw const DAT_100ba498) as *const i8,
                mech_trace::gr!("10084d93", "100c9070", DAT_100c9070),
            );
        }
        mech_trace::rt!(
            "10084d93",
            if mech_trace::gr!("10084d93", "100c9070", DAT_100c9070) == 0 {
                0
            } else {
                0xffff_ffff
            }
        )
    }
}
