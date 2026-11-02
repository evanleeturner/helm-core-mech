// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_10215654: u32;
    static mut DAT_10215658: u32;
    static mut DAT_1021565c: u32;
    static mut DAT_10215660: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100086f7() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_1000a3fd(param_1: i32, param_2: *mut u32, param_3: *mut i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000a86e() {
    // SAFETY: the globals and callees are the game's own memory and code; the
    // table entry and the object it points at are read where the original does.
    unsafe {
        if DAT_10215658 < 3 {
            FUN_100086f7();
            if DAT_10215658 < 3 {
                if DAT_10215658 != 0 {
                    DAT_10215654 = DAT_1021565c;
                    DAT_1021565c = 3;
                }
                let puv_ar1: *mut u32 = (&raw mut DAT_100a7b30)
                    .cast::<u8>()
                    .byte_add(DAT_10215658.wrapping_mul(4) as usize)
                    .cast::<*mut u32>()
                    .read_unaligned();
                if !puv_ar1.is_null() {
                    let pi_var2: *mut i32 =
                        puv_ar1.byte_add(20).cast::<*mut i32>().read_unaligned();
                    if !pi_var2.is_null() {
                        if DAT_10215654 != DAT_1021565c {
                            FUN_10003d80(pi_var2);
                        }
                        FUN_1000a3fd(1, puv_ar1, pi_var2, 0);
                    }
                }
            }
        } else if DAT_10215658 == 4 && DAT_10215660 < 3 {
            FUN_100086f7();
        }
    }
}
