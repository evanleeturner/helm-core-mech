// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8194: i32;
    static mut DAT_100a8184: *mut i32;
    static mut DAT_100a817c: *mut i32;
    static mut DAT_100c5bd4: i32;
    static mut DAT_100a725c: *mut i32;
    static mut DAT_100c5a50: i32;
    static mut DAT_100a84b4: [i8; 0];
    static mut DAT_100a84d0: [i8; 0];
    static mut DAT_100a84ec: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(format: *const i8, ...);
}

#[inline(always)]
unsafe fn slot(obj: *mut i32, off: usize) -> usize {
    // SAFETY: obj is a COM object whose first dword is its vtable address.
    unsafe {
        let vt = obj.read_unaligned() as u32 as usize as *const u8;
        vt.add(off).cast::<usize>().read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10062bf0() -> i32 {
    // SAFETY: the globals, vtables and callee are the game's own; the COM
    // objects are live DirectDraw surfaces called through their vtables.
    unsafe {
        if DAT_100a8194 == 0 {
            DAT_100c5a50 = 0;
        } else {
            let f: unsafe extern "stdcall" fn(*mut i32, i32) -> i32 =
                core::mem::transmute(slot(DAT_100a8184, 0x80));
            DAT_100c5a50 = f(DAT_100a8184, DAT_100c5bd4);
            *DAT_100a725c = 0;
            DAT_100a8194 = 0;
        }
        if DAT_100c5a50 != 0 {
            FUN_100640a0((&raw mut DAT_100a84b4).cast::<i8>(), DAT_100c5a50);
        }
        let f: unsafe extern "stdcall" fn(*mut i32) -> i32 =
            core::mem::transmute(slot(DAT_100a817c, 0x60));
        let i_var1 = f(DAT_100a817c);
        if i_var1 == -0x7789fe3e {
            let f: unsafe extern "stdcall" fn(*mut i32) -> i32 =
                core::mem::transmute(slot(DAT_100a817c, 0x6c));
            DAT_100c5a50 = f(DAT_100a817c);
            let f: unsafe extern "stdcall" fn(*mut i32) -> i32 =
                core::mem::transmute(slot(DAT_100a8184, 0x60));
            let i_var1 = f(DAT_100a8184);
            if i_var1 == -0x7789fe3e {
                let f: unsafe extern "stdcall" fn(*mut i32) -> i32 =
                    core::mem::transmute(slot(DAT_100a8184, 0x6c));
                DAT_100c5a50 = f(DAT_100a8184);
            }
        } else {
            DAT_100c5a50 = 0;
        }
        if DAT_100c5a50 != 0 {
            FUN_100640a0((&raw mut DAT_100a84d0).cast::<i8>(), DAT_100c5a50);
        }
        let f: unsafe extern "stdcall" fn(*mut i32, i32, i32) -> i32 =
            core::mem::transmute(slot(DAT_100a817c, 0x2c));
        DAT_100c5a50 = f(DAT_100a817c, 0, 1);
        if DAT_100c5a50 != 0 {
            FUN_100640a0((&raw mut DAT_100a84ec).cast::<i8>(), DAT_100c5a50);
        }
        ((DAT_100c5a50 == 0) as i32).wrapping_sub(1)
    }
}
