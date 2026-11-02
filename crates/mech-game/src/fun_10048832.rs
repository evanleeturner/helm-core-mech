// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160bd8: u32;
    static mut DAT_101686f0: u32;
    static mut DAT_100af0b4: u32;
    static mut DAT_100af0b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10088e93() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10048832(param_1: i32) {
    // SAFETY: param_1 points at the game's object and its field at +0xc points
    // at a record; the globals and callees are the game's own.
    unsafe {
        let i_var1 = ((param_1 as u32).wrapping_add(0xc) as *const i32).read_unaligned();
        if i_var1 != 0 {
            let p = i_var1 as u32;
            if ((p.wrapping_add(0x14)) as *const i32).read_unaligned() != 0 {
                let v = ((p.wrapping_add(0x10)) as *const u32).read_unaligned();
                FUN_1008664f(v.wrapping_add(DAT_101686f0), DAT_100af0b4 as *mut i32);
                (p.wrapping_add(0x14) as *mut u32).write_unaligned(0);
            }
            if ((p.wrapping_add(0x20)) as *const i32).read_unaligned() != 0 {
                let v = ((p.wrapping_add(0x1c)) as *const u32).read_unaligned();
                FUN_1008664f(v.wrapping_add(DAT_101686f0), DAT_100af0b4 as *mut i32);
                (p.wrapping_add(0x20) as *mut u32).write_unaligned(0);
            }
            if ((p.wrapping_add(0x30)) as *const i32).read_unaligned() != 0 {
                let v = ((p.wrapping_add(0x2c)) as *const u32).read_unaligned();
                FUN_1008664f(v.wrapping_add(DAT_101686f0), DAT_100af0b8 as *mut i32);
                (p.wrapping_add(0x30) as *mut u32).write_unaligned(0);
            }
            if ((p.wrapping_add(4)) as *const u8).read() & 1 != 0 {
                FUN_10088e93();
            }
        }
        ((param_1 as u32).wrapping_add(0xc) as *mut u32).write_unaligned(0);
        DAT_10160bd8 = DAT_10160bd8.wrapping_sub(1);
    }
}
