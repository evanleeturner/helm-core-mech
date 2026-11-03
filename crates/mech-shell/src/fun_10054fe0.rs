// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100ab6e8: [u8; 1];
    static mut DAT_100c01d8: [u8; 1];
    static mut DAT_100c11d8: [u8; 1];
    static mut s_Out_of_memory_in_TMPackDataBaseO_100a04a4: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10054ef0(this: *mut u8, param_1: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fseek(file: *mut c_void, offset: i32, origin: i32) -> i32;
    fn FUN_1006f790(dst: *mut c_void, size: usize, count: usize, file: *mut c_void) -> usize;
    fn FID_conflict__fwprintf(file: *mut c_void, format: *const u16, ...) -> i32;
    fn _fflush(file: *mut c_void) -> i32;
    fn FUN_1006fde0(param_1: u32);
    fn _fgetc(file: *mut c_void) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(heap: *mut c_void, flags: u32, size: u32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10054fe0(
    this: *mut u8,
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
) -> u32 {
    mech_trace::enter!("10054fe0", this, param_1, param_2, param_3);
    // SAFETY: the callees and globals are the game's own; this points at the
    // object whose FILE* sits at +0x80, and param_2/param_3 are the caller's
    // out slots; the ring buffer is the game's 0x1000-byte window.
    unsafe {
        let mut local_8: u32 = 0;
        let plvar2 =
            mech_trace::cr!("10054fe0", "10054ef0", FUN_10054ef0(this, param_1)) as *const i32;
        if plvar2.is_null() {
            return mech_trace::rt!("10054fe0", 1);
        }
        let file = mech_trace::rd!(
            "10054fe0",
            this.byte_add(0x80).cast::<*mut c_void>(),
            read_unaligned
        );
        let iv = mech_trace::cr!(
            "10054fe0",
            "_fseek",
            _fseek(file, mech_trace::rd!("10054fe0", plvar2, read_unaligned), 0)
        );
        if iv != 0 {
            return mech_trace::rt!("10054fe0", 1);
        }
        let file = mech_trace::rd!(
            "10054fe0",
            this.byte_add(0x80).cast::<*mut c_void>(),
            read_unaligned
        );
        let s = mech_trace::cr!(
            "10054fe0",
            "1006f790",
            FUN_1006f790((&raw mut local_8).cast::<c_void>(), 4, 1, file)
        );
        if s != 1 {
            return mech_trace::rt!("10054fe0", 1);
        }
        let mut p5 = mech_trace::cr!(
            "10054fe0",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10054fe0", "10098224", DAT_10098224),
                1,
                local_8
            )
        );
        if p5.is_null() {
            mech_trace::cr!(
                "10054fe0",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(
                    (&raw mut DAT_100ab6e8).cast::<c_void>(),
                    (&raw mut s_Out_of_memory_in_TMPackDataBaseO_100a04a4)
                        .cast::<u16>()
                        .cast_const(),
                )
            );
            mech_trace::cr!(
                "10054fe0",
                "_fflush",
                _fflush((&raw mut DAT_100ab6e8).cast::<c_void>())
            );
            FUN_1006fde0(1);
        }
        mech_trace::wr!("10054fe0", param_2, write_unaligned, p5 as u32);
        let mut u8v: u32 = 2;
        mech_trace::wr!("10054fe0", param_3, write_unaligned, local_8);
        let base = (&raw mut DAT_100c01d8).cast::<u8>();
        let end = (&raw mut DAT_100c11d8).cast::<u8>();
        let mut p11 = base.cast::<u32>();
        let mut n: i32 = 0x400;
        while n != 0 {
            mech_trace::wr!("10054fe0", p11, write_unaligned, 0);
            p11 = p11.byte_add(4);
            n -= 1;
        }
        let mut p11 = base;
        while 0 < (local_8 as i32) {
            u8v >>= 1;
            if u8v == 1 {
                let f = mech_trace::rd!(
                    "10054fe0",
                    this.byte_add(0x80).cast::<*mut c_void>(),
                    read_unaligned
                );
                u8v = (mech_trace::cr!("10054fe0", "_fgetc", _fgetc(f)) as u32) | 0x100;
            }
            if (u8v & 1) == 0 {
                let f = mech_trace::rd!(
                    "10054fe0",
                    this.byte_add(0x80).cast::<*mut c_void>(),
                    read_unaligned
                );
                let i3 = mech_trace::cr!("10054fe0", "_fgetc", _fgetc(f));
                let f = mech_trace::rd!(
                    "10054fe0",
                    this.byte_add(0x80).cast::<*mut c_void>(),
                    read_unaligned
                );
                let i6 = mech_trace::cr!("10054fe0", "_fgetc", _fgetc(f));
                let u9 = i3.wrapping_add(i6.wrapping_mul(0x100)) as u32;
                let mut cnt = (((u9 & 0xf000) >> 0xc) + 3) as i32;
                let mut p10 = base.byte_add((u9 & 0xfff) as usize);
                local_8 = local_8.wrapping_sub(cnt as u32);
                while cnt != 0 {
                    cnt -= 1;
                    let b = mech_trace::rd!("10054fe0", p10, read);
                    p10 = p10.byte_add(1);
                    mech_trace::wr!("10054fe0", p11, write, b);
                    p11 = p11.byte_add(1);
                    mech_trace::wr!("10054fe0", p5, write, b);
                    p5 = p5.byte_add(1);
                    if p11 == end {
                        p11 = base;
                    }
                    if p10 == end {
                        p10 = base;
                    }
                }
            } else {
                local_8 = local_8.wrapping_sub(1);
                let p10 = p11.byte_add(1);
                let p7 = p5.byte_add(1);
                let f = mech_trace::rd!(
                    "10054fe0",
                    this.byte_add(0x80).cast::<*mut c_void>(),
                    read_unaligned
                );
                let c = mech_trace::cr!("10054fe0", "_fgetc", _fgetc(f));
                mech_trace::wr!("10054fe0", p11, write, c as u8);
                mech_trace::wr!("10054fe0", p5, write, c as u8);
                p11 = p10;
                p5 = p7;
                if p10 == end {
                    p11 = base;
                }
            }
        }
        mech_trace::rt!("10054fe0", 0)
    }
}
