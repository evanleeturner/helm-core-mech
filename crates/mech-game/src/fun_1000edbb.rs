// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b50: *mut i8;
    static mut DAT_100a7b84: u32;
    static mut DAT_100a7b88: u16;
    static mut DAT_100a7b8a: u8;
    static mut DAT_100a7b8c: u16;
    static mut DAT_100a7b90: u32;
    static mut DAT_100a7b94: u16;
    static mut DAT_100a7b98: u32;
    static mut DAT_100ada70: [i8; 0];
    static mut DAT_101686f0: i32;
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100adf80: i32;
    static mut DAT_100b9a18: u32;
    static mut DAT_100b9a1c: u32;
    static mut DAT_101f52e0: [u32; 5];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006657e(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_1005b295(param_1: *mut u32, param_2: i32, param_3: *mut u32);
    fn FUN_1005b4ae(param_1: i32, param_2: *mut u8);
    fn FUN_1006af90(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _FILE_read_8(param_1: *mut i8, param_2: i32) -> i32;
    fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: u32) -> *mut c_void;
    fn HeapFree(hHeap: *mut c_void, dwFlags: u32, lpMem: *mut c_void) -> i32;
}

/// Returns pointer one past the terminating nul.
unsafe fn past_nul(mut p: *const i8) -> *const i8 {
    // SAFETY: caller passes a nul-terminated string.
    unsafe {
        loop {
            let c = *p;
            p = p.add(1);
            if c == 0 {
                return p;
            }
        }
    }
}

/// Copies the nul-terminated `src` (with its nul) to `dst`.
unsafe fn copy_str(dst: *mut i8, src: *const i8) {
    // SAFETY: caller guarantees src is a string and dst has room.
    unsafe {
        let end = past_nul(src);
        let n = end as usize - src as usize;
        let mut i = 0;
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

/// Appends `src` to the string in `dst`.
unsafe fn append_str(dst: *mut i8, src: *const i8) {
    // SAFETY: caller guarantees both are strings and dst has room.
    unsafe {
        let src_end = past_nul(src);
        let n = src_end as usize - src as usize;
        let d = (past_nul(dst) as *mut i8).sub(1);
        let mut i = 0;
        while i < n {
            *d.add(i) = *src.add(i);
            i += 1;
        }
    }
}

unsafe fn put16(buf: *mut i8, v: u16) {
    // SAFETY: buf is a string with room after its nul.
    unsafe {
        let p = past_nul(buf) as *mut i8;
        *p.sub(1) = v as i8;
        *p = (v >> 8) as i8;
    }
}

unsafe fn put32(buf: *mut i8, v: u32) {
    // SAFETY: buf is a string with room after its nul.
    unsafe {
        let p = past_nul(buf) as *mut i8;
        *p.sub(1) = v as i8;
        *p = (v >> 8) as i8;
        *p.add(1) = (v >> 16) as i8;
        *p.add(2) = (v >> 24) as i8;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000edbb() {
    let mut local_120_buf: [u32; 0x40] = [0; 0x40];
    let local_120: *mut i8 = local_120_buf.as_mut_ptr().cast::<i8>();
    let mut local_1c: [u32; 5] = [0; 5];

    // SAFETY: globals and callees are the game's own; the buffers are local
    // and large enough for the strings built, as in the original.
    unsafe {
        let name_ptr: *const i8 = (&raw mut DAT_100ada70)
            .cast::<i8>()
            .cast_const()
            .wrapping_add((DAT_101686f0 as u32).wrapping_mul(2) as usize);

        if DAT_100a7b50.is_null() || *DAT_100a7b50 == 0 {
            local_120_buf[0] = DAT_100a7b84;
            local_120
                .byte_add(4)
                .cast::<u16>()
                .write_unaligned(DAT_100a7b88);
            *local_120.byte_add(6).cast::<u8>() = DAT_100a7b8a;
            append_str(local_120, name_ptr);
            put16(local_120, DAT_100a7b8c);
            put32(local_120, DAT_100a7b90);
        } else {
            copy_str(local_120, DAT_100a7b50);
            append_str(local_120, name_ptr);
            put16(local_120, DAT_100a7b94);
            put32(local_120, DAT_100a7b98);
        }

        let local_8 = _FILE_read_8(local_120, 0);
        if local_8 != 0 {
            let local_20 = HeapAlloc(DAT_100adf74, 1, 0x502e) as *mut u32;
            if !local_20.is_null() {
                let lp_mem = HeapAlloc(DAT_100adf74, 9, 0x300) as *mut u8;
                if !lp_mem.is_null() {
                    let f14: unsafe extern "C" fn(i32, i32, *mut u8, i32) = core::mem::transmute(
                        (DAT_100b9a18.wrapping_add(0x14) as *const u32).read_unaligned(),
                    );
                    f14(0, 0x100, lp_mem, 1);

                    let src = &raw mut DAT_101f52e0;
                    let mut i = 0;
                    while i < 5 {
                        local_1c[i] = (*src)[i];
                        i += 1;
                    }
                    FUN_1006657e(
                        local_1c.as_mut_ptr() as i32,
                        local_1c.as_mut_ptr() as i32,
                        local_8,
                    );

                    let local_128: i32 = if DAT_100adf80 == 0 {
                        -1
                    } else {
                        let f20: unsafe extern "C" fn() -> i32 = core::mem::transmute(
                            (DAT_100b9a18.wrapping_add(0x20) as *const u32).read_unaligned(),
                        );
                        f20()
                    };
                    if local_128 == 0 {
                        FUN_1005b295(local_1c.as_mut_ptr(), local_8, local_20);
                        if DAT_100adf80 != 0 {
                            let f18: unsafe extern "C" fn() = core::mem::transmute(
                                (DAT_100b9a1c.wrapping_add(0x18) as *const u32).read_unaligned(),
                            );
                            f18();
                        }
                    }
                    FUN_1005b4ae(local_8, lp_mem);
                    let f1c: unsafe extern "C" fn(*mut u8, i32) = core::mem::transmute(
                        (DAT_100b9a18.wrapping_add(0x1c) as *const u32).read_unaligned(),
                    );
                    f1c(lp_mem, 0x3c);
                    HeapFree(DAT_100adf74, 1, lp_mem as *mut c_void);
                }
                HeapFree(DAT_100adf74, 1, local_20 as *mut c_void);
            }
            FUN_1006af90(local_8);
        }
    }
}
