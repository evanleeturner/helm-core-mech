// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a1c: *mut i32;
    static mut DAT_100b9f70: *mut i32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f78: *mut i32;
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f80: *mut i32;
    static mut DAT_100c9070: i32;
    static mut DAT_100c90f0: i32;
    static mut DAT_100c90f4: i32;
    static mut DAT_100c9104: i32;
    static mut DAT_100c9158: i32;
    static mut DAT_100c91cc: i32;
    static mut DAT_100c91d0: i32;
    static mut DAT_100c91d4: i32;
    static mut DAT_100c91d8: i32;
    static mut DAT_100c91dc: i32;
    static mut DAT_100c9238: i32;

    static mut s_DDRAW_CreateSurfaces_CreateSurfa_100ba05c: [i8; 0];
    static mut s_DDRAW_CreateSurfaces_GetAttached_100ba090: [i8; 0];
    static mut s_DDRAW_CreateSurfaces_GetAttached_100ba0c4: [i8; 0];
    static mut s_DDRAW_CreateSurfaces_primary_fai_100ba0fc: [i8; 0];
    static mut s_DDRAW_CreateSurfaces_primary_fai_100ba138: [i8; 0];
    static mut s_DDRAW_CreateSurfaces_GetAttached_100ba174: [i8; 0];
    static mut s_DDRAW_CreateSurfaces_CreateSurfa_100ba1a4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, ...);
}

type CreateFn = unsafe extern "stdcall" fn(*mut i32, *mut i32, *mut *mut i32, i32) -> i32;
type GetFn = unsafe extern "stdcall" fn(*mut i32, *mut i32, *mut *mut i32) -> i32;

unsafe fn create_surface(desc: *mut i32, out: *mut *mut i32) -> i32 {
    // SAFETY: the DirectDraw interface global is the game's own; the vtable
    // slot at +0x18 is CreateSurface.
    unsafe {
        let dd = DAT_100b9f70;
        let vt = dd.read_unaligned() as u32;
        let f: CreateFn =
            core::mem::transmute((vt.wrapping_add(0x18) as *const usize).read_unaligned());
        f(dd, desc, out, 0)
    }
}

unsafe fn get_attached(surf: *mut i32, caps: *mut i32, out: *mut *mut i32) -> i32 {
    // SAFETY: surf is a live surface interface; vtable slot +0x30 is
    // GetAttachedSurface.
    unsafe {
        let vt = surf.read_unaligned() as u32;
        let f: GetFn =
            core::mem::transmute((vt.wrapping_add(0x30) as *const usize).read_unaligned());
        f(surf, caps, out)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10083ed8(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // interface pointers are called through their vtables as the original does.
    unsafe {
        let mut b_var1 = false;
        let i_var2: i32;

        if param_1 == 0x140 && param_2 == 200 {
            let mode = DAT_100b9a1c.read_unaligned();
            if mode == 2 || mode == 3 {
                i_var2 = -1;
            } else {
                DAT_100c90f0 = 0x6c;
                DAT_100c90f4 = 0x20;
                DAT_100c9158 = 0x218;
                DAT_100c9104 = 2;
                DAT_100c9070 = create_surface(&raw mut DAT_100c90f0, &raw mut DAT_100b9f74);
                if DAT_100c9070 == 0 {
                    DAT_100c91cc = 4;
                    DAT_100c9070 =
                        get_attached(DAT_100b9f74, &raw mut DAT_100c91cc, &raw mut DAT_100b9f78);
                    if DAT_100c9070 == 0 {
                        DAT_100b9f7c = DAT_100b9f78;
                        DAT_100c91cc = 0x10;
                        let r = get_attached(
                            DAT_100b9f74,
                            &raw mut DAT_100c91cc,
                            &raw mut DAT_100b9f80,
                        );
                        DAT_100c9070 = r;
                        if r != 0 {
                            FUN_10002168(
                                &raw mut s_DDRAW_CreateSurfaces_GetAttached_100ba0c4 as *mut i8,
                                DAT_100c9070,
                            );
                            i_var2 = DAT_100c9070;
                        } else {
                            i_var2 = r;
                        }
                    } else {
                        FUN_10002168(
                            &raw mut s_DDRAW_CreateSurfaces_GetAttached_100ba090 as *mut i8,
                            DAT_100c9070,
                        );
                        i_var2 = DAT_100c9070;
                    }
                } else {
                    FUN_10002168(
                        &raw mut s_DDRAW_CreateSurfaces_CreateSurfa_100ba05c as *mut i8,
                        DAT_100c9070,
                    );
                    i_var2 = DAT_100c9070;
                }
            }
        } else {
            let mode = DAT_100b9a1c.read_unaligned();
            if mode == 0 || mode == 1 {
                b_var1 = true;
            }
            DAT_100c90f0 = 0x6c;
            DAT_100c90f4 = 0;
            DAT_100c9158 = 0x4200;
            if b_var1 {
                DAT_100c90f4 = 0x20;
                DAT_100c9158 = 0x4218;
                DAT_100c9104 = 2;
            }
            DAT_100c9070 = create_surface(&raw mut DAT_100c90f0, &raw mut DAT_100b9f74);
            if DAT_100c9070 != 0 {
                if b_var1 {
                    FUN_10002168(&raw mut s_DDRAW_CreateSurfaces_primary_fai_100ba0fc as *mut i8);
                    DAT_100c9104 = 1;
                    DAT_100c9070 = create_surface(&raw mut DAT_100c90f0, &raw mut DAT_100b9f74);
                } else if DAT_100b9a1c.read_unaligned() == 3 {
                    FUN_10002168(&raw mut s_DDRAW_CreateSurfaces_primary_fai_100ba138 as *mut i8);
                    DAT_100c9158 &= 0xffffbfffu32 as i32;
                    DAT_100c9070 = create_surface(&raw mut DAT_100c90f0, &raw mut DAT_100b9f74);
                }
            }
            let mut r = DAT_100c9070;
            if DAT_100c9070 == 0 {
                if b_var1 {
                    DAT_100c91cc = 4;
                    DAT_100c9070 =
                        get_attached(DAT_100b9f74, &raw mut DAT_100c91cc, &raw mut DAT_100b9f78);
                    if DAT_100c9070 != 0 {
                        FUN_10002168(
                            &raw mut s_DDRAW_CreateSurfaces_GetAttached_100ba174 as *mut i8,
                            DAT_100c9070,
                        );
                        return DAT_100c9070;
                    }
                }
                if DAT_100b9a1c.read_unaligned() == 0 {
                    DAT_100b9f7c = DAT_100b9f78;
                    r = DAT_100c9070;
                } else {
                    DAT_100c91d0 = 0x6c;
                    DAT_100c91d4 = 6;
                    DAT_100c91dc = param_1;
                    DAT_100c91d8 = param_2;
                    if DAT_100b9a1c.read_unaligned() == 2 {
                        DAT_100c9238 = 0x4040;
                    } else {
                        DAT_100c9238 = 0x840;
                    }
                    DAT_100c9070 = create_surface(&raw mut DAT_100c91d0, &raw mut DAT_100b9f7c);
                    r = DAT_100c9070;
                    if DAT_100c9070 != 0 {
                        FUN_10002168(
                            &raw mut s_DDRAW_CreateSurfaces_CreateSurfa_100ba1a4 as *mut i8,
                            DAT_100c9070,
                        );
                        r = DAT_100c9070;
                    }
                }
            }
            i_var2 = r;
        }
        i_var2
    }
}
