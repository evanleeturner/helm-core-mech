// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a724c: *mut i32;
    static mut DAT_100a8178: *mut i32;
    static mut DAT_100a817c: *mut i32;
    static mut DAT_100a8180: *mut i32;
    static mut DAT_100a8184: *mut i32;
    static mut DAT_100a8188: *mut i32;
    static mut DAT_100c5a50: i32;

    static mut DAT_100c5ad0: i32;
    static mut DAT_100c5ad4: i32;
    static mut _DAT_100c5ae4: i32;
    static mut DAT_100c5b38: i32;

    static mut _DAT_100c5bac: i32;

    static mut DAT_100c5bb0: i32;
    static mut DAT_100c5bb4: i32;
    static mut _DAT_100c5bb8: i32;
    static mut _DAT_100c5bbc: i32;
    static mut _DAT_100c5c18: i32;

    static mut DAT_100a8264: i8;
    static mut DAT_100a8298: i8;
    static mut DAT_100a82cc: i8;
    static mut DAT_100a8304: i8;
    static mut DAT_100a8340: i8;
    static mut DAT_100a837c: i8;
    static mut DAT_100a83ac: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(param_1: *mut i8, ...);
}

type CreateFn =
    unsafe extern "stdcall" fn(*mut i32, *mut c_void, *mut *mut i32, *mut c_void) -> i32;
type AttachFn = unsafe extern "stdcall" fn(*mut i32, *mut c_void, *mut *mut i32) -> i32;

unsafe fn create_surface(desc: *mut c_void, out: *mut *mut i32) -> i32 {
    // SAFETY: the DirectDraw object and its vtable are the game's own.
    unsafe {
        let obj = mech_trace::gr!("10062520", "100a8178", DAT_100a8178);
        let vtbl = mech_trace::rd!("10062520", obj, read_unaligned) as u32 as usize;
        let f = mech_trace::rd!("10062520", ((vtbl + 0x18) as *const u32), read_unaligned);
        let f: CreateFn = core::mem::transmute::<u32, CreateFn>(f);
        mech_trace::cr!("10062520", "ind", f(obj, desc, out, core::ptr::null_mut()))
    }
}

unsafe fn get_attached(caps: *mut c_void, out: *mut *mut i32) -> i32 {
    // SAFETY: the primary surface and its vtable are the game's own.
    unsafe {
        let obj = mech_trace::gr!("10062520", "100a817c", DAT_100a817c);
        let vtbl = mech_trace::rd!("10062520", obj, read_unaligned) as u32 as usize;
        let f = mech_trace::rd!("10062520", ((vtbl + 0x30) as *const u32), read_unaligned);
        let f: AttachFn = core::mem::transmute::<u32, AttachFn>(f);
        mech_trace::cr!("10062520", "ind", f(obj, caps, out))
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10062520(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("10062520", param_1, param_2);
    // SAFETY: all globals, callees and COM objects are the game's own memory.
    unsafe {
        let mut b_var1: i32 = 0;
        if param_1 == 0x140 && param_2 == 200 {
            if mech_trace::dr!(
                "10062520",
                mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
            ) == 2
                || mech_trace::dr!(
                    "10062520",
                    mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
                ) == 3
            {
                return mech_trace::rt!("10062520", -1);
            }
            mech_trace::gw!("10062520", "100c5ad0", DAT_100c5ad0, 0x6c);
            mech_trace::gw!("10062520", "100c5ad4", DAT_100c5ad4, 0x20);
            mech_trace::gw!("10062520", "100c5b38", DAT_100c5b38, 0x218);
            mech_trace::gw!("10062520", "_DAT_100c5ae4", _DAT_100c5ae4, 2);
            mech_trace::gw!(
                "10062520",
                "100c5a50",
                DAT_100c5a50,
                create_surface((&raw mut DAT_100c5ad0).cast(), &raw mut DAT_100a817c)
            );
            if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) != 0 {
                FUN_100640a0(
                    &raw mut DAT_100a8264,
                    mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50),
                );
                return mech_trace::rt!(
                    "10062520",
                    mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50)
                );
            }
            mech_trace::gw!("10062520", "_DAT_100c5bac", _DAT_100c5bac, 4);
            mech_trace::gw!(
                "10062520",
                "100c5a50",
                DAT_100c5a50,
                get_attached((&raw mut _DAT_100c5bac).cast(), &raw mut DAT_100a8180)
            );
            if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) != 0 {
                FUN_100640a0(
                    &raw mut DAT_100a8298,
                    mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50),
                );
                return mech_trace::rt!(
                    "10062520",
                    mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50)
                );
            }
            mech_trace::gw!(
                "10062520",
                "100a8184",
                DAT_100a8184,
                mech_trace::gr!("10062520", "100a8180", DAT_100a8180)
            );
            mech_trace::gw!("10062520", "_DAT_100c5bac", _DAT_100c5bac, 0x10);
            mech_trace::gw!(
                "10062520",
                "100c5a50",
                DAT_100c5a50,
                get_attached((&raw mut _DAT_100c5bac).cast(), &raw mut DAT_100a8188)
            );
            if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) != 0 {
                FUN_100640a0(
                    &raw mut DAT_100a82cc,
                    mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50),
                );
                return mech_trace::rt!(
                    "10062520",
                    mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50)
                );
            }
        } else {
            if mech_trace::dr!(
                "10062520",
                mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
            ) == 0
                || mech_trace::dr!(
                    "10062520",
                    mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
                ) == 1
            {
                b_var1 = 1;
            }
            mech_trace::gw!("10062520", "100c5ad0", DAT_100c5ad0, 0x6c);
            mech_trace::gw!("10062520", "100c5ad4", DAT_100c5ad4, 0);
            mech_trace::gw!("10062520", "100c5b38", DAT_100c5b38, 0x4200);
            if b_var1 != 0 {
                mech_trace::gw!("10062520", "100c5ad4", DAT_100c5ad4, 0x20);
                mech_trace::gw!("10062520", "100c5b38", DAT_100c5b38, 0x4218);
                mech_trace::gw!("10062520", "_DAT_100c5ae4", _DAT_100c5ae4, 2);
            }
            mech_trace::gw!(
                "10062520",
                "100c5a50",
                DAT_100c5a50,
                create_surface((&raw mut DAT_100c5ad0).cast(), &raw mut DAT_100a817c)
            );
            if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) != 0 {
                if b_var1 != 0 {
                    FUN_100640a0(&raw mut DAT_100a8304);
                    mech_trace::gw!("10062520", "_DAT_100c5ae4", _DAT_100c5ae4, 1);
                    mech_trace::gw!(
                        "10062520",
                        "100c5a50",
                        DAT_100c5a50,
                        create_surface((&raw mut DAT_100c5ad0).cast(), &raw mut DAT_100a817c)
                    );
                } else if mech_trace::dr!(
                    "10062520",
                    mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
                ) == 3
                {
                    FUN_100640a0(&raw mut DAT_100a8340);
                    mech_trace::gu!("10062520", "100c5b38", DAT_100c5b38, &=, 0xffffbfffu32 as i32);
                    mech_trace::gw!(
                        "10062520",
                        "100c5a50",
                        DAT_100c5a50,
                        create_surface((&raw mut DAT_100c5ad0).cast(), &raw mut DAT_100a817c)
                    );
                }
            }
            if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) == 0 {
                if b_var1 != 0 {
                    mech_trace::gw!("10062520", "_DAT_100c5bac", _DAT_100c5bac, 4);
                    mech_trace::gw!(
                        "10062520",
                        "100c5a50",
                        DAT_100c5a50,
                        get_attached((&raw mut _DAT_100c5bac).cast(), &raw mut DAT_100a8180)
                    );
                    if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) != 0 {
                        FUN_100640a0(
                            &raw mut DAT_100a837c,
                            mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50),
                        );
                        return mech_trace::rt!(
                            "10062520",
                            mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50)
                        );
                    }
                }
                if mech_trace::dr!(
                    "10062520",
                    mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
                ) == 0
                {
                    mech_trace::gw!(
                        "10062520",
                        "100a8184",
                        DAT_100a8184,
                        mech_trace::gr!("10062520", "100a8180", DAT_100a8180)
                    );
                    return mech_trace::rt!(
                        "10062520",
                        mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50)
                    );
                }
                mech_trace::gw!("10062520", "_DAT_100c5bbc", _DAT_100c5bbc, param_1);
                mech_trace::gw!("10062520", "_DAT_100c5bb8", _DAT_100c5bb8, param_2);
                mech_trace::gw!("10062520", "100c5bb0", DAT_100c5bb0, 0x6c);
                mech_trace::gw!("10062520", "100c5bb4", DAT_100c5bb4, 6);
                mech_trace::gw!("10062520", "_DAT_100c5c18", _DAT_100c5c18, 0x4040);
                if mech_trace::dr!(
                    "10062520",
                    mech_trace::gr!("10062520", "100a724c", DAT_100a724c)
                ) != 2
                {
                    mech_trace::gw!("10062520", "_DAT_100c5c18", _DAT_100c5c18, 0x840);
                }
                mech_trace::gw!(
                    "10062520",
                    "100c5a50",
                    DAT_100c5a50,
                    create_surface((&raw mut DAT_100c5bb0).cast(), &raw mut DAT_100a8184)
                );
                if mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50) != 0 {
                    FUN_100640a0(
                        &raw mut DAT_100a83ac,
                        mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50),
                    );
                }
            }
        }
        mech_trace::rt!(
            "10062520",
            mech_trace::gr!("10062520", "100c5a50", DAT_100c5a50)
        )
    }
}
