// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ba9f8: i32;
    static mut DAT_100baa10: u8;
    static mut DAT_100baa14: u8;
    static mut DAT_100baa18: u8;
    static mut DAT_100baa24: u8;
    static mut DAT_100baa8c: u8;
    static mut DAT_100c8b68: u8;
    static mut DAT_100c2474: u8;
    static mut DAT_100c2498: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086cc1(param_1: *mut i8) -> i32;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
    fn FUN_1008716b(param_1: *mut u32) -> *mut u8;
    fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008762f(param_1: *mut u32, param_2: *mut i8, param_3: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the table entries
    // and param_1/param_2 are accessed exactly where the original does.
    unsafe {
        let mut i_var1 = DAT_100ba9f8;
        let mut local_10: i32 = 0;
        loop {
            if DAT_100ba9f8 <= local_10 {
                DAT_100ba9f8 = DAT_100ba9f8.wrapping_add(1);
                if DAT_100ba9f8 < 0x33 {
                    let off = i_var1.wrapping_mul(0x80) as usize;
                    let local_8: i32 = if param_2.is_null() {
                        -1
                    } else {
                        FUN_10086cc1(param_2)
                    };
                    (&raw mut DAT_100baa10)
                        .byte_add(off)
                        .cast::<i32>()
                        .write_unaligned(local_8);
                    (&raw mut DAT_100baa14)
                        .byte_add(off)
                        .cast::<i32>()
                        .write_unaligned(param_3);
                    let p_var2 = FUN_1008716b(param_1);
                    (&raw mut DAT_100baa24)
                        .byte_add(off)
                        .cast::<*mut u8>()
                        .write_unaligned(p_var2);
                    if local_8 < 0 {
                        (&raw mut DAT_100baa18)
                            .byte_add(off)
                            .cast::<u32>()
                            .write_unaligned(0);
                    } else {
                        let v = ((&raw mut DAT_100c8b68) as usize as i32)
                            .wrapping_add(local_8.wrapping_mul(0x50))
                            .wrapping_add(param_3.wrapping_mul(4));
                        (&raw mut DAT_100baa18)
                            .byte_add(off)
                            .cast::<i32>()
                            .write_unaligned(v);
                    }
                    let first = param_1.read_unaligned() as usize as *const i8;
                    let i_var3 = strncmp(first, &raw const DAT_100c2498 as *const i8, 5);
                    if i_var3 == 0 {
                        (&raw mut DAT_100baa8c)
                            .byte_add(off)
                            .cast::<i32>()
                            .write_unaligned(1);
                    } else {
                        (&raw mut DAT_100baa8c)
                            .byte_add(off)
                            .cast::<i32>()
                            .write_unaligned(0);
                    }
                } else {
                    FUN_100108a0(0x11, &raw mut DAT_100c2474 as *mut i8);
                    i_var1 = -1;
                }
                return i_var1;
            }
            let off = local_10.wrapping_mul(0x80) as usize;
            let entry = (&raw mut DAT_100baa24)
                .byte_add(off)
                .cast::<i32>()
                .read_unaligned();
            if entry != 0 {
                let inner = (entry as usize as *const i32).read_unaligned() as u32;
                if inner == param_1.add(1).read_unaligned() {
                    if param_2.is_null() {
                        return local_10;
                    }
                    let cur = (&raw mut DAT_100baa18)
                        .byte_add(off)
                        .cast::<i32>()
                        .read_unaligned();
                    if cur == 0 {
                        let r = FUN_10086cc1(param_2);
                        let v = ((&raw mut DAT_100c8b68) as usize as i32)
                            .wrapping_add(r.wrapping_mul(0x50))
                            .wrapping_add(param_3.wrapping_mul(4));
                        let off2 = local_10.wrapping_mul(0x80) as usize;
                        (&raw mut DAT_100baa18)
                            .byte_add(off2)
                            .cast::<i32>()
                            .write_unaligned(v);
                        (&raw mut DAT_100baa10)
                            .byte_add(off2)
                            .cast::<i32>()
                            .write_unaligned(r);
                        (&raw mut DAT_100baa14)
                            .byte_add(off2)
                            .cast::<i32>()
                            .write_unaligned(param_3);
                        return local_10;
                    }
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
    }
}
