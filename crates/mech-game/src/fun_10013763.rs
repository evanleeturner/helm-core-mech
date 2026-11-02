// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1d0: u16;
    static mut DAT_100aa1f4: u32;
    static mut DAT_100aa210: u32;
    static mut DAT_100adf38: i32;
    static mut DAT_100adf98: u32;
    static mut DAT_101eb510: [u8; 0];
    static mut DAT_101eb518: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10013716();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e80(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10013763(param_1: u32, param_2: i32) {
    let _ = param_1;
    // SAFETY: the globals and callees are the game's own memory and code;
    // the flag array is indexed by param_2 exactly as the original does.
    unsafe {
        if DAT_100adf38 != param_2 {
            if DAT_100aa1f4 == 1 {
                let base = (&raw mut DAT_101eb510).cast::<u8>();
                base.offset(param_2 as isize).write(1);
                let mut local_8: i32 = 0;
                while local_8 < DAT_101eb518 && base.offset(local_8 as isize).read() == 1 {
                    local_8 = local_8.wrapping_add(1);
                }
                if DAT_101eb518 == local_8 {
                    FUN_10013716();
                }
            } else {
                let i_var1 = FUN_10011e80(DAT_100aa210 as i32);
                if i_var1 == param_2 {
                    DAT_100adf98 |= 1;
                    DAT_100aa1d0 = 1;
                }
            }
        }
    }
}
