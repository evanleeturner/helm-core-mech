// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut c_void;
    static mut DAT_1009b368: i32;
    static mut DAT_100d08e8: *mut c_void;
    static mut DAT_1009bdf0: i32;
    static mut DAT_1009bdd8: i32;
    static mut DAT_1009bde0: u8;
    static mut DAT_1009bde8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000e970(msg: *mut c_void, param_2: u32, param_3: u32) -> u32;
    fn FUN_1000fe40();
    fn FUN_1000d440() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PostMessageA(hWnd: *mut c_void, Msg: u32, wParam: u32, lParam: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004eb30(msg: u32) -> u32 {
    // SAFETY: the globals and callees are the game's own; DAT_10083508 points
    // at the game state block whose offsets are read exactly as the original does.
    unsafe {
        let mut u_var1: u32 = DAT_10083508 as u32;
        let base = DAT_10083508 as *const u8;
        let local_23bb = base.byte_add(0x23bb).cast::<i32>().read_unaligned();
        if local_23bb == 0 {
            let local_1fe = base.byte_add(0x1fe).cast::<i32>().read_unaligned();
            if local_1fe == 4 {
                if (DAT_10083508 as *const u8).byte_add(0x239c).read() as i8 == 1 {
                    DAT_1009b368 = 0;
                    let u_var3 =
                        FUN_1000e970((&raw mut DAT_1009bdf0) as *mut c_void, 0xc02b, 0xc00a);
                    return (u_var3 >> 8) | 1;
                }
                u_var1 = PostMessageA(DAT_100d08e8, msg, 0xc00a, 0) as u32;
                return u_var1 & 0xFFFFFF00;
            }
        } else if local_23bb == 1 {
            let local_1fe = (DAT_10083508 as *const u8)
                .byte_add(0x1fe)
                .cast::<i32>()
                .read_unaligned();
            if local_1fe == 4 {
                if (DAT_10083508 as *const u8).byte_add(0x23b7).read() as i8 == 0 {
                    DAT_1009b368 = 0;
                    FUN_1000e970((&raw mut DAT_1009bdd8) as *mut c_void, 0xc004, 0xc00a);
                }
                FUN_1000fe40();
                let i_var2 = FUN_1000d440();
                let local_12a = (DAT_10083508 as *const u8)
                    .byte_add(0x12a)
                    .cast::<i32>()
                    .read_unaligned();
                if 14999999 < local_12a.wrapping_add(i_var2) {
                    let u_var3 =
                        FUN_1000e970((&raw mut DAT_1009bde0) as *mut c_void, 0xc02b, 0xc00a);
                    return (u_var3 >> 8) | 1;
                }
                let u_var3 = FUN_1000e970((&raw mut DAT_1009bde8) as *mut c_void, 0xc02b, 0xc00a);
                return (u_var3 >> 8) | 1;
            }
        } else {
            u_var1 = PostMessageA(DAT_100d08e8, msg, 0xc00a, 0) as u32;
        }
        u_var1 & 0xFFFFFF00
    }
}
