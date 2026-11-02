// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1f4: u32;
    static mut DAT_100adf80: u32;
    static mut DAT_100adf88: u32;
    static mut DAT_100adf8c: u32;
    static mut DAT_100adf94: u32;
    static mut DAT_100b9d4c: u32;
    static mut DAT_100ba96c: u16;
    static mut s_WinMain_3___pause_timer_false__100ae3d0: [i8; 0];
    static mut s_WinMain_4___pause_timer_TRUE__100ae3f0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100498c5(param_1: i32) -> i32;
    fn FUN_10002168(param_1: *mut i8);
    fn FUN_10036b20(param_1: u16, param_2: i16);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10051093();
    fn FUN_1006aeb1();
    fn FUN_10088e93();
    fn FUN_10051071();
    fn FUN_1006ae8d();
    fn FUN_10088e7e();
    fn ShowCursor(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037d6e() {
    // SAFETY: the globals and callees are the game's own; the string
    // globals are passed by address as the original does.
    unsafe {
        if DAT_100adf8c != 0 {
            if DAT_100adf88 == 0 || DAT_100ba96c == 0 {
                if DAT_100aa1f4 != 0 || FUN_100498c5(4) != 0 {
                    DAT_100adf8c = 0;
                }
            } else {
                FUN_10051093();
                DAT_100ba96c = 0;
                DAT_100adf8c = 0;
            }
        }
        let mut i_var1 = FUN_100498c5(4);
        if i_var1 == 0 && DAT_100adf80 != 0 && DAT_100adf8c == 0 {
            if DAT_100adf88 != 0 {
                loop {
                    i_var1 = ShowCursor(0);
                    if i_var1 < 0 {
                        break;
                    }
                }
                DAT_100adf94 = 0;
                FUN_10002168((&raw mut s_WinMain_3___pause_timer_false__100ae3d0).cast::<i8>());
                FUN_10036b20(0x80, 0);
                FUN_1006aeb1();
                FUN_10088e93();
                DAT_100adf88 = 0;
            }
        } else if DAT_100adf88 == 0 && DAT_100aa1f4 == 0 {
            if DAT_100adf8c != 0 {
                FUN_10051071();
                DAT_100ba96c = 0;
            }
            i_var1 = FUN_100498c5(4);
            if i_var1 == 0 && DAT_100b9d4c != 1 {
                loop {
                    i_var1 = ShowCursor(1);
                    if i_var1 >= 0 {
                        break;
                    }
                }
                DAT_100adf94 = 1;
            }
            FUN_10002168((&raw mut s_WinMain_4___pause_timer_TRUE__100ae3f0).cast::<i8>());
            FUN_10036b20(0x80, 1);
            FUN_1006ae8d();
            FUN_10088e7e();
            DAT_100adf88 = 1;
        }
    }
}
