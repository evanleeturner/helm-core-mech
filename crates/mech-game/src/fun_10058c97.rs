// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b12c5: i32;
    static mut DAT_100b12c1: i32;
    static mut DAT_100b12d1: *mut i8;
    static mut DAT_100b12d5: i32;
    static mut DAT_100b12cd: i32;
    static mut DAT_100b12c9: i32;
    static mut DAT_100b12e9: i32;
    static mut DAT_100b12f1: i32;
}

unsafe fn prelude() {
    // SAFETY: the globals are the game's own memory; the output pointer is
    // written only where the original writes it.
    unsafe {
        if DAT_100b12c5 != 0 {
            loop {
                let mut v1 = DAT_100b12c5;
                if 0xfe < DAT_100b12c5 {
                    v1 = 0xff;
                }
                DAT_100b12c5 = DAT_100b12c5.wrapping_sub(v1);
                if DAT_100b12c1 != 0 {
                    *DAT_100b12d1 = 1;
                    *DAT_100b12d1.wrapping_offset(1) = v1 as i8;
                }
                DAT_100b12d1 = DAT_100b12d1.wrapping_offset(2);
                DAT_100b12d5 = DAT_100b12d5.wrapping_add(v1);
                if DAT_100b12c5 == 0 {
                    break;
                }
            }
            DAT_100b12c5 = 0;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10058c97(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; the output and source
    // pointers are the game's buffers, accessed where the original does.
    unsafe {
        if param_1 == 2 {
            prelude();
            let mut v3 = DAT_100b12cd
                .wrapping_sub(param_2)
                .wrapping_sub(DAT_100b12d5);
            let mut v1 = DAT_100b12d5.wrapping_add(param_3.wrapping_sub(DAT_100b12c9));
            if v1 < DAT_100b12e9 {
                DAT_100b12e9 = v1;
            }
            let mut v2 = v1.wrapping_add(v3).wrapping_sub(1);
            let mut skip = v2 <= DAT_100b12f1;
            loop {
                if !skip {
                    DAT_100b12f1 = v2;
                    if v3 == 0 {
                        break;
                    }
                }
                skip = false;
                v1 = v3;
                if 0x7e < v3 {
                    v1 = 0x7f;
                }
                if DAT_100b12c1 != 0 {
                    *DAT_100b12d1 = (v1 as i8).wrapping_mul(2);
                    *DAT_100b12d1.wrapping_offset(1) = *(DAT_100b12d5 as *const i8);
                }
                DAT_100b12d1 = DAT_100b12d1.wrapping_offset(2);
                DAT_100b12d5 = DAT_100b12d5.wrapping_add(v1);
                v2 = DAT_100b12f1;
                v3 = v3.wrapping_sub(v1);
            }
        } else if param_1 == 1 {
            prelude();
            let mut v3 = DAT_100b12cd
                .wrapping_sub(param_2)
                .wrapping_sub(DAT_100b12d5);
            let mut v1 = DAT_100b12d5.wrapping_add(param_3.wrapping_sub(DAT_100b12c9));
            if v1 < DAT_100b12e9 {
                DAT_100b12e9 = v1;
            }
            let mut v2 = v1.wrapping_add(v3).wrapping_sub(1);
            let mut skip = v2 <= DAT_100b12f1;
            loop {
                if !skip {
                    DAT_100b12f1 = v2;
                    if v3 == 0 {
                        break;
                    }
                }
                skip = false;
                v1 = v3;
                if 0x7e < v3 {
                    v1 = 0x7f;
                }
                if DAT_100b12c1 == 0 {
                    DAT_100b12d5 = DAT_100b12d5.wrapping_add(v1);
                    DAT_100b12d1 = DAT_100b12d1.wrapping_offset(v1.wrapping_add(1) as isize);
                } else {
                    *DAT_100b12d1 = v1.wrapping_mul(2).wrapping_add(1) as i8;
                    v2 = v1;
                    loop {
                        DAT_100b12d1 = DAT_100b12d1.wrapping_offset(1);
                        if v2 == 0 {
                            break;
                        }
                        *DAT_100b12d1 = *(DAT_100b12d5 as *const i8);
                        DAT_100b12d5 = DAT_100b12d5.wrapping_add(1);
                        v2 = v2.wrapping_sub(1);
                    }
                }
                v2 = DAT_100b12f1;
                v3 = v3.wrapping_sub(v1);
            }
        } else if param_1 == 3 {
            DAT_100b12c5 = DAT_100b12cd
                .wrapping_sub(param_2)
                .wrapping_sub(DAT_100b12d5);
        } else if param_1 == 4 {
            if DAT_100b12c1 != 0 {
                *DAT_100b12d1 = 0;
            }
            DAT_100b12d1 = DAT_100b12d1.wrapping_offset(1);
        } else if param_1 == 0 {
            DAT_100b12c5 = 0;
            DAT_100b12d5 = DAT_100b12cd;
        }
        0
    }
}
