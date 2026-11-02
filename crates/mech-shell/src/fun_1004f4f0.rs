// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009bffc: i8;
    static mut DAT_1009c000: i8;
    static mut DAT_1009c004: i8;
    static mut DAT_1009c048: [i8; 1];
    static mut DAT_1009c04c: u16;
    static mut DAT_1009c038: [i8; 1];
    static mut DAT_1009c040: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn atoi(param_1: *const i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004f4c0(param_1: i32, param_2: *mut i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004f4f0(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: i32,
    param_5: *mut i32,
    param_6: *mut i32,
) -> u32 {
    // SAFETY: all pointers are the caller's buffers and the game's own globals;
    // reads and writes happen in the same order as the original.
    unsafe {
        if param_4 <= *param_3 {
            return 0;
        }
        if DAT_1009c000 == 1 {
            DAT_1009c000 = 0;
            sprintf(param_2, (&raw const DAT_1009c038).cast::<i8>(), *param_6);
            return 4;
        }
        if DAT_1009c004 == 1 {
            DAT_1009c004 = 0;
            sprintf(param_2, (&raw const DAT_1009c040).cast::<i8>(), *param_6);
            return 4;
        }
        let src = (&raw const DAT_1009c048).cast::<i8>();
        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut k = 0usize;
        while k < n {
            *param_2.add(k) = *src.add(k);
            k += 1;
        }

        let mut i_var5: i32 = 0;
        let mut i_var7: i32 = *param_3;
        while i_var7 < param_4 {
            loop {
                let b = *(param_1.wrapping_add(i_var7) as *const u8);
                if b == 0x20 || b == 0x5c {
                    break;
                }
                i_var7 = i_var7.wrapping_add(1);
                *param_2.offset(i_var5 as isize) = (b & 0x7f) as i8;
                i_var5 = i_var5.wrapping_add(1);
                if i_var7 >= param_4 {
                    break;
                }
            }
            if param_4 <= i_var7 {
                break;
            }
            let c = *(param_1.wrapping_add(i_var7) as *const i8);
            if (c == 0x5c || c == 0x20) && 0 < i_var5 {
                *param_2.offset(i_var5 as isize) = 0;
                *param_3 = i_var7;
                let mut cnt = FUN_1004f4c0(param_1, param_3, param_4);
                if 0 < cnt {
                    loop {
                        let mut p = param_2;
                        while *p != 0 {
                            p = p.add(1);
                        }
                        cnt = cnt.wrapping_sub(1);
                        p.cast::<u16>().write_unaligned(DAT_1009c04c);
                        if cnt == 0 {
                            break;
                        }
                    }
                }
                return 4;
            }
            if c != 0x5c {
                return 4;
            }
            let sel = *(param_1.wrapping_add(1).wrapping_add(i_var7) as *const u8);
            let a2 = *(param_1.wrapping_add(i_var7).wrapping_add(2) as *const i8);
            let a3 = *(param_1.wrapping_add(i_var7).wrapping_add(3) as *const i8);
            match sel {
                0x41 | 0x61 => {
                    let local_8: [i8; 4] = [a2, a3, 0, 0];
                    DAT_1009bffc = 1;
                    *param_5 = atoi(local_8.as_ptr());
                    *param_3 = i_var7.wrapping_add(4);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 6;
                }
                0x42 | 0x62 => {
                    let a4 = *(param_1.wrapping_add(i_var7).wrapping_add(4) as *const i8);
                    let local_8: [i8; 4] = [a2, a3, a4, 0];
                    DAT_1009c000 = 1;
                    *param_6 = atoi(local_8.as_ptr());
                    *param_3 = i_var7.wrapping_add(5);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 7;
                }
                0x43 | 0x63 => {
                    *param_3 = i_var7.wrapping_add(2);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 2;
                }
                0x47 | 0x67 => {
                    let a4 = *(param_1.wrapping_add(i_var7).wrapping_add(4) as *const i8);
                    let local_8: [i8; 4] = [a2, a3, a4, 0];
                    DAT_1009c004 = 1;
                    *param_6 = atoi(local_8.as_ptr());
                    *param_3 = i_var7.wrapping_add(5);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 8;
                }
                0x4c | 0x6c => {
                    *param_3 = i_var7.wrapping_add(2);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 9;
                }
                0x4e | 0x6e => {
                    *param_3 = i_var7.wrapping_add(2);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 1;
                }
                0x53 | 0x73 => {
                    *param_3 = i_var7.wrapping_add(2);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 3;
                }
                0x54 | 0x74 => {
                    *param_3 = i_var7.wrapping_add(2);
                    FUN_1004f4c0(param_1, param_3, param_4);
                    return 5;
                }
                _ => {
                    *param_2.offset(i_var5 as isize) = 0x5c;
                    i_var7 = i_var7.wrapping_add(2);
                    i_var5 = i_var5.wrapping_add(1);
                }
            }
        }
        *param_2.offset(i_var5 as isize) = 0;
        *param_3 = i_var7;
        0
    }
}
