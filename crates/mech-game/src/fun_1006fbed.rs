// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101e86b0: u8;
    static mut DAT_101e86b2: u8;
    static mut DAT_100b9a30: u8;
    static mut DAT_100b9a31: u8;
    static mut DAT_100b9a32: u8;
    static mut DAT_100c9278: u8;
    static mut DAT_100c9279: u8;
    static mut DAT_100c927a: u8;
    static mut DAT_100b3de0: i32;
    static mut DAT_100eee30: i32;
    static DAT_100b4004: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d94c(param_1: *mut u8, param_2: *mut u8);
    fn FUN_10002168(param_1: *const i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SendMessageA(a: i32, b: u32, c: i32, d: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006fbed(param_1: i32) -> i32 {
    if param_1 == 0 {
        return -1;
    }
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a 256-entry palette of 3-byte entries, read and written as the original does.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < 0x100 {
            let off = local_8.wrapping_mul(3);
            let src = (off.wrapping_add(param_1)) as usize as *mut u8;
            let s_var: u16 = src.cast::<u16>().read_unaligned();
            (&raw mut DAT_101e86b0)
                .byte_add(off as usize)
                .cast::<u16>()
                .write_unaligned(s_var);
            *(&raw mut DAT_101e86b2).byte_add(off as usize) = src.byte_add(2).read();
            FUN_1001d94c(src, (&raw mut DAT_100b9a30).byte_add(off as usize));
            let off4 = local_8.wrapping_mul(4) as usize;
            *(&raw mut DAT_100c927a).byte_add(off4) =
                (*(&raw mut DAT_100b9a30).byte_add(off as usize)).wrapping_mul(4);
            *(&raw mut DAT_100c9279).byte_add(off4) =
                (*(&raw mut DAT_100b9a31).byte_add(off as usize)).wrapping_mul(4);
            *(&raw mut DAT_100c9278).byte_add(off4) =
                (*(&raw mut DAT_100b9a32).byte_add(off as usize)).wrapping_mul(4);
            local_8 += 1;
        }
        let mut local_14: i32 = 0x400;
        DAT_100eee30 = SendMessageA(DAT_100b3de0, 0x4a, 0, (&raw mut local_14) as usize as i32);
        if DAT_100eee30 == 0 {
            0
        } else {
            FUN_10002168((&raw const DAT_100b4004).cast::<i8>());
            -1
        }
    }
}
