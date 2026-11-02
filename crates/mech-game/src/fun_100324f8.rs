// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada90: i32;
    static mut DAT_100ada98: i32;
    static mut DAT_101686e8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dst: *mut i8, src: *mut i8, n: i32);
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
    fn FUN_1001d9b0(param_1: *mut i16, param_2: *mut i32) -> *mut i32;
    fn FUN_1001dd9f(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100324f8(param_1: i32, param_2: *mut core::ffi::c_void) -> u32 {
    let mut local_44: [i32; 8] = [0; 8];
    // Contiguous: short at bytes 0..2, chars at bytes 2..16.
    let mut buf: [u8; 16] = [0; 16];
    let mut local_14: i32 = 1;
    let mut local_8: u32 = 0;
    let p = buf.as_mut_ptr();

    // SAFETY: param_1 points at the caller's record; buf is a local buffer
    // large enough for every access; the callees and globals are the game's own.
    unsafe {
        let v = ((param_1.wrapping_add(8)) as *const i16).read_unaligned();
        (p as *mut i16).write_unaligned(v);
        _strncpy(
            p.add(2) as *mut i8,
            param_1.wrapping_add(10) as *mut i8,
            0xc,
        );
        *p.add(14) = 0;
        let s = (p as *const i16).read_unaligned();
        if (s == -2 || s == -1) && *p.add(2) == b'^' {
            if DAT_100ada90 == 0 {
                FUN_100108a0(0x39, core::ptr::null_mut());
                local_14 = 0;
            } else if DAT_100ada98 < DAT_101686e8 {
                _strncpy(
                    p.add(2) as *mut i8,
                    DAT_100ada98
                        .wrapping_mul(0xc)
                        .wrapping_add(DAT_100ada90)
                        .wrapping_add(8) as *mut i8,
                    0xc,
                );
                DAT_100ada98 = DAT_100ada98.wrapping_add(1);
                *p.add(14) = 0;
            } else {
                FUN_100108a0(0x3b, core::ptr::null_mut());
                local_14 = 0;
            }
        }
        if local_14 != 0 {
            let local_c = FUN_1001d9b0(p as *mut i16, local_44.as_mut_ptr());
            if local_c.is_null() {
                FUN_100108a0(0x34, core::ptr::null_mut());
            } else {
                local_8 = 1;
                let f: unsafe extern "C" fn(*mut i32) -> u32 = core::mem::transmute(param_2);
                let r = f(local_c);
                local_8 &= r;
                FUN_1001dd9f(local_c);
            }
        }
    }
    local_8
}
