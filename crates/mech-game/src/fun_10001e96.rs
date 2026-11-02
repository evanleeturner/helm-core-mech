// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123230: i32;
    static mut DAT_1012327a: u8;
    static mut DAT_1012327c: u8;
    static mut DAT_10123368: u8;
    static mut DAT_10123369: u8;
    static mut DAT_10168410: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(str1: *const i8, str2: *const i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004fee5() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10001e96(str1: *const i8, param_2: u32, param_3: u16) {
    let mut local_14: i32 = 0;
    // SAFETY: all globals and callees are the game's own memory and code; the
    // offsets mirror the original's table layouts.
    unsafe {
        while local_14 < DAT_10123230 {
            let entry = (&raw mut DAT_10168410)
                .byte_offset((local_14 as isize).wrapping_mul(4))
                .cast::<i32>()
                .read_unaligned();
            if entry != 0 {
                let mut local_c: i32 = 0;
                while local_c < 0x30 {
                    let off = local_14
                        .wrapping_mul(0x3c0a)
                        .wrapping_add(local_c.wrapping_mul(0x13f))
                        as isize;
                    if (&raw mut DAT_1012327a).byte_offset(off).read() == 7 {
                        let name = (entry as u32)
                            .wrapping_add((local_c as u32).wrapping_mul(0x97))
                            .wrapping_add(0x70) as *const i8;
                        let ivar2 = __strcmpi(str1, name);
                        if ivar2 == 0
                            && (param_2
                                & (&raw mut DAT_1012327c)
                                    .byte_offset(off)
                                    .cast::<u32>()
                                    .read_unaligned())
                                != 0
                            && (&raw mut DAT_10123368).byte_offset(off).read() < 0x28
                        {
                            let p = (&raw mut DAT_10123368).byte_offset(off);
                            let bvar1 = p.read();
                            p.write(p.read().wrapping_add(1));
                            (&raw mut DAT_10123369)
                                .byte_offset(off)
                                .byte_add((bvar1 as usize).wrapping_mul(2))
                                .cast::<u16>()
                                .write_unaligned(param_3);
                            FUN_1004fee5();
                        }
                    }
                    local_c = local_c.wrapping_add(1);
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
    }
}
