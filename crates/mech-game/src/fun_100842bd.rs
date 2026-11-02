// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a18: *mut i32;
    static mut PTR_DAT_100b99f0: *mut i32;
    static mut DAT_100b9d4c: i32;
    static mut DAT_100b9a1c: *mut i32;
    static mut DAT_100b9f70: *mut i32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f78: *mut i32;
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f88: i32;
    static mut DAT_100b9f90: i32;
    static mut DAT_100adf6c: i32;
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100c9070: i32;
    static mut DAT_100c9080: i32;
    static mut DAT_100c9060: i32;
    static mut DAT_100c9064: i32;
    static mut DAT_100c9068: i32;
    static mut DAT_100c906c: i32;
    static mut DAT_100c90f0: [i32; 0x1b];
    static mut DAT_100c91d0: [i32; 0x1b];
    static mut DAT_100c9160: [i32; 0x1b];
    static mut DAT_100c9170: i32;
    static mut DAT_100c9250: i32;

    static mut s_DDRAW_Init_DirectDrawCreate______100ba1d8: [i8; 1];
    static mut s_DDRAW_Init_SetCooperativeLevel___100ba1fc: [i8; 1];
    static mut s_DDRAW_Init_SetDisplayMode_____d_100ba224: [i8; 1];
    static mut s_DDRAW_Init__bank_switched_video_c_100ba248: [i8; 1];
    static mut s_DDraw_CreatePalette_____d_100ba270: [i8; 1];
    static mut s_DDraw_SetPalette_Primary____d_100ba28c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, ...);
    fn FUN_10082279(param_1: i32);
    fn FUN_10083dc0();
    fn FUN_10083ed8(param_1: i32, param_2: i32) -> i32;
    fn FUN_10083e28();
    fn FUN_100851e2(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> i32;
    fn FUN_10084d93();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn DirectDrawCreate(lpGUID: *mut c_void, lplpDD: *mut *mut i32, pUnkOuter: *mut c_void) -> i32;
    fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: u32) -> *mut c_void;
}

unsafe fn slot(obj: *mut i32, off: usize) -> usize {
    // SAFETY: obj is a COM object whose first word is its vtable pointer.
    unsafe {
        let vt = mech_trace::rd!("100842bd", obj, read) as usize as *const u8;
        mech_trace::rd!("100842bd", vt.add(off).cast::<i32>(), read_unaligned) as usize
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100842bd(param_1: *mut i32, param_2: i32, param_3: i32) -> i32 {
    mech_trace::enter!("100842bd", param_1, param_2, param_3);
    // SAFETY: all globals, callees and COM vtables are the game's own; param_1
    // points at the caller's int array.
    unsafe {
        let mut local_b0: [u32; 0x2b] = [0; 0x2b];

        if mech_trace::rd!(
            "100842bd",
            mech_trace::gr!("100842bd", "100b9a18", DAT_100b9a18),
            read
        ) != 0
        {
            let f: unsafe extern "C" fn() = core::mem::transmute(mech_trace::rd!(
                "100842bd",
                mech_trace::gr!("100842bd", "100b9a18", DAT_100b9a18).add(4),
                read
            ) as usize);
            mech_trace::cr!("100842bd", "ind", f());
            mech_trace::gw!(
                "100842bd",
                "100b9a18",
                DAT_100b9a18,
                mech_trace::gr!("100842bd", "100b99f0", PTR_DAT_100b99f0)
            );
            if mech_trace::rd!(
                "100842bd",
                mech_trace::gr!("100842bd", "100b99f0", PTR_DAT_100b99f0).add(1),
                read
            ) != mech_trace::gr!("100842bd", "100b9d4c", DAT_100b9d4c)
            {
                FUN_10082279(mech_trace::gr!("100842bd", "100b99f0", PTR_DAT_100b99f0) as i32);
            }
        }

        if mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70).is_null() {
            mech_trace::gw!(
                "100842bd",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "100842bd",
                    "DirectDrawCreate",
                    DirectDrawCreate(
                        core::ptr::null_mut(),
                        &raw mut DAT_100b9f70,
                        core::ptr::null_mut(),
                    )
                )
            );
            if mech_trace::gr!("100842bd", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut s_DDRAW_Init_DirectDrawCreate______100ba1d8).cast::<i8>(),
                    mech_trace::gr!("100842bd", "100c9070", DAT_100c9070),
                );
                return mech_trace::rt!("100842bd", 1);
            }
            let f: unsafe extern "stdcall" fn(*mut i32, i32, i32) -> i32 = core::mem::transmute(
                slot(mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70), 0x50),
            );
            mech_trace::gw!(
                "100842bd",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "100842bd",
                    "ind",
                    f(
                        mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                        mech_trace::gr!("100842bd", "100adf6c", DAT_100adf6c),
                        0x51
                    )
                )
            );
            if mech_trace::gr!("100842bd", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut s_DDRAW_Init_SetCooperativeLevel___100ba1fc).cast::<i8>(),
                    mech_trace::gr!("100842bd", "100c9070", DAT_100c9070),
                );
                FUN_10083dc0();
                return mech_trace::rt!("100842bd", 1);
            }
            let f: unsafe extern "stdcall" fn(*mut i32, i32, i32, i32) -> i32 =
                core::mem::transmute(slot(
                    mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                    0x54,
                ));
            mech_trace::gw!(
                "100842bd",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "100842bd",
                    "ind",
                    f(
                        mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                        param_2,
                        param_3,
                        8
                    )
                )
            );
            if mech_trace::gr!("100842bd", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut s_DDRAW_Init_SetDisplayMode_____d_100ba224).cast::<i8>(),
                    mech_trace::gr!("100842bd", "100c9070", DAT_100c9070),
                );
                FUN_10083dc0();
                return mech_trace::rt!("100842bd", 1);
            }
        }

        local_b0[0] = 0xac;
        let f: unsafe extern "stdcall" fn(*mut i32, *mut c_void, *mut c_void) -> i32 =
            core::mem::transmute(slot(
                mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                0x2c,
            ));
        let mut i_var3 = mech_trace::cr!(
            "100842bd",
            "ind",
            f(
                mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                local_b0.as_mut_ptr().cast::<c_void>(),
                core::ptr::null_mut(),
            )
        );

        if i_var3 == 0
            && (mech_trace::rd!("100842bd", local_b0.as_ptr().cast::<u8>().add(7), read) & 8) != 0
            && (mech_trace::rd!(
                "100842bd",
                mech_trace::gr!("100842bd", "100b9a1c", DAT_100b9a1c),
                read
            ) == 0
                || mech_trace::rd!(
                    "100842bd",
                    mech_trace::gr!("100842bd", "100b9a1c", DAT_100b9a1c),
                    read
                ) == 2)
        {
            FUN_10002168((&raw mut s_DDRAW_Init__bank_switched_video_c_100ba248).cast::<i8>());
            return mech_trace::rt!("100842bd", 1);
        }

        i_var3 = mech_trace::cr!("100842bd", "10083ed8", FUN_10083ed8(param_2, param_3));
        if i_var3 != 0 {
            FUN_10083e28();
            return mech_trace::rt!("100842bd", 1);
        }
        mech_trace::gw!("100842bd", "100c9080", DAT_100c9080, 100);
        i_var3 = mech_trace::cr!(
            "100842bd",
            "100851e2",
            FUN_100851e2(0, 0, param_2.wrapping_sub(1), param_3.wrapping_sub(1), 0)
        );
        if i_var3 != 0 {
            FUN_10083e28();
            return mech_trace::rt!("100842bd", 1);
        }
        let g: unsafe extern "C" fn() -> i32 = core::mem::transmute(mech_trace::rd!(
            "100842bd",
            mech_trace::gr!("100842bd", "100b9a1c", DAT_100b9a1c).add(6),
            read
        ) as usize);
        i_var3 = mech_trace::cr!("100842bd", "ind", g());
        if i_var3 != 0 {
            FUN_10083e28();
            return mech_trace::rt!("100842bd", 1);
        }

        let p = (&raw mut DAT_100c90f0).cast::<i32>();
        for k in 0..0x1b {
            mech_trace::wr!("100842bd", p.add(k), write, 0);
        }
        mech_trace::wr!("100842bd", p, write, 0x6c);
        let f: unsafe extern "stdcall" fn(*mut i32, *mut i32) -> i32 = core::mem::transmute(slot(
            mech_trace::gr!("100842bd", "100b9f74", DAT_100b9f74),
            0x58,
        ));
        mech_trace::gw!(
            "100842bd",
            "100c9070",
            DAT_100c9070,
            mech_trace::cr!(
                "100842bd",
                "ind",
                f(mech_trace::gr!("100842bd", "100b9f74", DAT_100b9f74), p)
            )
        );

        let p = (&raw mut DAT_100c91d0).cast::<i32>();
        for k in 0..0x1b {
            mech_trace::wr!("100842bd", p.add(k), write, 0);
        }
        mech_trace::wr!("100842bd", p, write, 0x6c);
        let f: unsafe extern "stdcall" fn(*mut i32, *mut i32) -> i32 = core::mem::transmute(slot(
            mech_trace::gr!("100842bd", "100b9f7c", DAT_100b9f7c),
            0x58,
        ));
        mech_trace::gw!(
            "100842bd",
            "100c9070",
            DAT_100c9070,
            mech_trace::cr!(
                "100842bd",
                "ind",
                f(mech_trace::gr!("100842bd", "100b9f7c", DAT_100b9f7c), p)
            )
        );

        if !mech_trace::gr!("100842bd", "100b9f78", DAT_100b9f78).is_null() {
            let p = (&raw mut DAT_100c9160).cast::<i32>();
            for k in 0..0x1b {
                mech_trace::wr!("100842bd", p.add(k), write, 0);
            }
            mech_trace::wr!("100842bd", p, write, 0x6c);
            let f: unsafe extern "stdcall" fn(*mut i32, *mut i32) -> i32 = core::mem::transmute(
                slot(mech_trace::gr!("100842bd", "100b9f78", DAT_100b9f78), 0x58),
            );
            mech_trace::gw!(
                "100842bd",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "100842bd",
                    "ind",
                    f(mech_trace::gr!("100842bd", "100b9f78", DAT_100b9f78), p)
                )
            );
        }

        mech_trace::gw!("100842bd", "100c9060", DAT_100c9060, 0);
        mech_trace::gw!("100842bd", "100c9064", DAT_100c9064, 0);
        mech_trace::gw!("100842bd", "100c9068", DAT_100c9068, param_2);
        mech_trace::gw!("100842bd", "100c906c", DAT_100c906c, param_3);
        mech_trace::wr!("100842bd", param_1, write_unaligned, 0);
        mech_trace::wr!(
            "100842bd",
            param_1.add(1),
            write_unaligned,
            mech_trace::gr!("100842bd", "100c9170", DAT_100c9170).wrapping_sub(1)
        );
        mech_trace::wr!(
            "100842bd",
            param_1.add(2),
            write_unaligned,
            param_3.wrapping_sub(1)
        );
        mech_trace::wr!("100842bd", param_1.add(4), write_unaligned, 0);
        mech_trace::wr!(
            "100842bd",
            param_1.add(3),
            write_unaligned,
            (&raw mut DAT_100c9250) as i32
        );

        if mech_trace::gr!("100842bd", "100b9f88", DAT_100b9f88) == 0 {
            let pv_var2 = mech_trace::cr!(
                "100842bd",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("100842bd", "100adf74", DAT_100adf74),
                    9,
                    0x400
                )
            );
            if pv_var2.is_null() {
                return mech_trace::rt!("100842bd", -1);
            }
            let f: unsafe extern "stdcall" fn(
                *mut i32,
                i32,
                *mut c_void,
                *mut i32,
                *mut c_void,
            ) -> i32 = core::mem::transmute(slot(
                mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                0x14,
            ));
            mech_trace::gw!(
                "100842bd",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "100842bd",
                    "ind",
                    f(
                        mech_trace::gr!("100842bd", "100b9f70", DAT_100b9f70),
                        0x44,
                        pv_var2,
                        &raw mut DAT_100b9f88,
                        core::ptr::null_mut(),
                    )
                )
            );
            if mech_trace::gr!("100842bd", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut s_DDraw_CreatePalette_____d_100ba270).cast::<i8>(),
                    mech_trace::gr!("100842bd", "100c9070", DAT_100c9070),
                );
                FUN_10083dc0();
                return mech_trace::rt!("100842bd", -1);
            }
            mech_trace::gw!("100842bd", "100c9070", DAT_100c9070, 0);
        }

        let f: unsafe extern "stdcall" fn(*mut i32, i32) -> i32 = core::mem::transmute(slot(
            mech_trace::gr!("100842bd", "100b9f74", DAT_100b9f74),
            0x7c,
        ));
        mech_trace::gw!(
            "100842bd",
            "100c9070",
            DAT_100c9070,
            mech_trace::cr!(
                "100842bd",
                "ind",
                f(
                    mech_trace::gr!("100842bd", "100b9f74", DAT_100b9f74),
                    mech_trace::gr!("100842bd", "100b9f88", DAT_100b9f88)
                )
            )
        );
        if mech_trace::gr!("100842bd", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw mut s_DDraw_SetPalette_Primary____d_100ba28c).cast::<i8>(),
                mech_trace::gr!("100842bd", "100c9070", DAT_100c9070),
            );
        }

        if param_2 == 0x140 && param_3 == 200 {
            mech_trace::wr!(
                "100842bd",
                mech_trace::gr!("100842bd", "100b9a1c", DAT_100b9a1c).add(8),
                write,
                FUN_10084d93 as usize as i32
            );
        }
        mech_trace::gw!("100842bd", "100b9f90", DAT_100b9f90, 1);
        mech_trace::rt!("100842bd", 0)
    }
}
