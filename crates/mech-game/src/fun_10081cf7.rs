// SPDX-License-Identifier: MIT

#[repr(C)]
struct LargeInt {
    low_part: u32,
    high_part: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a1c: *mut i32;
    static mut DAT_100b9a28: i32;
    static mut DAT_100b9a24: i32;
    static mut DAT_100b9d60: i32;
    static mut DAT_100c8500: u32;
    static mut DAT_100c8504: i32;
    static mut DAT_100b9a2c: i32;
    static mut DAT_100c967c: i32;
    static mut DAT_100c9680: i32;
    static mut DAT_100b9a04: [*mut i32; 0];
    static mut s_Refresh_mode__d_start___u__d__en_100b9d84: [i8; 0];
    static mut s_Refresh_mode__d_not_available_100b9db8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn QueryPerformanceCounter(param_1: *mut u8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *const i8, ...);
    fn FUN_10081ef2();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081cf7() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the pointer table entries and the record they point to are read and
    // written exactly where the original does.
    unsafe {
        if DAT_100b9a28 < *DAT_100b9a1c {
            DAT_100b9a24 = 0;
        } else if DAT_100b9d60 == 1 {
            let b_var1 = QueryPerformanceCounter((&raw mut DAT_100c8500).cast::<u8>());
            if b_var1 == 0 {
                DAT_100b9a24 = 0;
            } else {
                DAT_100b9d60 = DAT_100b9d60.wrapping_add(1);
            }
        } else if DAT_100b9d60 == 5 {
            let mut local_c = LargeInt {
                low_part: 0,
                high_part: 0,
            };
            let b_var1 = QueryPerformanceCounter((&raw mut local_c).cast::<u8>());
            if b_var1 == 0 {
                DAT_100b9a24 = 0;
            } else {
                if local_c.high_part == DAT_100c8504 {
                    *DAT_100b9a1c.add(3) = local_c.low_part.wrapping_sub(DAT_100c8500) as i32;
                } else {
                    *DAT_100b9a1c.add(3) = (!DAT_100c8500)
                        .wrapping_add(local_c.low_part)
                        .wrapping_add(1) as i32;
                }
                FUN_10002168(
                    (&raw const s_Refresh_mode__d_start___u__d__en_100b9d84).cast::<i8>(),
                    *DAT_100b9a1c,
                    DAT_100c8500,
                    DAT_100c8504,
                    local_c.low_part,
                    local_c.high_part,
                    *DAT_100b9a1c.add(3),
                );
                if *DAT_100b9a1c == DAT_100b9a28 {
                    DAT_100b9a24 = 0;
                } else {
                    while *DAT_100b9a1c < DAT_100b9a28 {
                        let f: extern "C" fn() =
                            core::mem::transmute::<i32, extern "C" fn()>(*DAT_100b9a1c.add(5));
                        f();
                        let idx = *DAT_100b9a1c as isize;
                        DAT_100b9a1c = (&raw mut DAT_100b9a04)
                            .cast::<*mut i32>()
                            .offset(idx)
                            .read();
                        if *DAT_100b9a1c.add(2) != 0 {
                            let g: extern "C" fn(i32, i32, i32) -> i32 =
                                core::mem::transmute::<i32, extern "C" fn(i32, i32, i32) -> i32>(
                                    *DAT_100b9a1c.add(4),
                                );
                            let i_var2 = g(DAT_100b9a2c, DAT_100c967c, DAT_100c9680);
                            if i_var2 == 0 {
                                DAT_100b9d60 = 0;
                                break;
                            }
                        }
                        FUN_10002168(
                            (&raw const s_Refresh_mode__d_not_available_100b9db8).cast::<i8>(),
                            *DAT_100b9a1c,
                        );
                        *DAT_100b9a1c.add(2) = 0;
                        if *DAT_100b9a1c == DAT_100b9a28 {
                            DAT_100b9a24 = 0;
                        }
                    }
                }
                if DAT_100b9a24 == 0 {
                    FUN_10081ef2();
                }
            }
        } else {
            DAT_100b9d60 = DAT_100b9d60.wrapping_add(1);
        }
    }
}
