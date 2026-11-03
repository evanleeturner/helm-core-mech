// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100ab6e8: u8;
    static mut DAT_100a0470: u8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10054ef0(this: *mut c_void, param_1: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut c_void, format: *mut c_void, ...) -> i32;
    fn _fflush(file: *mut c_void) -> i32;
    fn FUN_1006fde0(param_1: u32);
    fn _fseek(file: *mut c_void, offset: i32, origin: i32) -> i32;
    fn FUN_1006f790(dst: *mut c_void, size: u32, count: u32, file: *mut c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10054f20(
    this: *mut c_void,
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut i32,
) -> u32 {
    mech_trace::enter!("10054f20", this, param_1, param_2, param_3);
    // SAFETY: the callees and globals are the game's own; this, param_2 and
    // param_3 point at the caller's memory, accessed where the original does.
    unsafe {
        let pl_var1 =
            mech_trace::cr!("10054f20", "10054ef0", FUN_10054ef0(this, param_1)) as *mut i32;
        if pl_var1.is_null() {
            mech_trace::wr!("10054f20", param_2, write_unaligned, 0);
            mech_trace::wr!("10054f20", param_3, write_unaligned, 0);
            return mech_trace::rt!("10054f20", 1);
        }
        let dst = mech_trace::cr!(
            "10054f20",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10054f20", "10098224", DAT_10098224),
                1,
                mech_trace::rd!("10054f20", pl_var1.byte_add(4), read_unaligned) as u32
            )
        );
        if dst.is_null() {
            mech_trace::cr!(
                "10054f20",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(
                    (&raw mut DAT_100ab6e8) as *mut c_void,
                    (&raw mut DAT_100a0470) as *mut c_void,
                )
            );
            mech_trace::cr!(
                "10054f20",
                "_fflush",
                _fflush((&raw mut DAT_100ab6e8) as *mut c_void)
            );
            FUN_1006fde0(1);
        }
        mech_trace::wr!("10054f20", param_2, write_unaligned, dst as u32);
        mech_trace::wr!(
            "10054f20",
            param_3,
            write_unaligned,
            mech_trace::rd!("10054f20", pl_var1.byte_add(4), read_unaligned)
        );
        let file = mech_trace::rd!(
            "10054f20",
            this.byte_add(0x80).cast::<*mut c_void>(),
            read_unaligned
        );
        mech_trace::cr!(
            "10054f20",
            "_fseek",
            _fseek(
                file,
                mech_trace::rd!("10054f20", pl_var1, read_unaligned),
                0
            )
        );
        let file = mech_trace::rd!(
            "10054f20",
            this.byte_add(0x80).cast::<*mut c_void>(),
            read_unaligned
        );
        mech_trace::cr!(
            "10054f20",
            "1006f790",
            FUN_1006f790(
                dst,
                1,
                mech_trace::rd!("10054f20", pl_var1.byte_add(4), read_unaligned) as u32,
                file
            )
        );
        mech_trace::rt!("10054f20", 0)
    }
}
