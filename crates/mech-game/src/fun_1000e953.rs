// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b68: i32;
    static mut DAT_100a7b60: i32;
    static mut DAT_100a7b6c: i32;
    static mut DAT_100a7b70: i32;
    static mut DAT_100a7b74: i32;
    static mut DAT_100a7b78: i32;
    static mut DAT_100a7b7c: i32;
    static mut DAT_100a7b80: i32;
    static mut DAT_100a7b4c: i32;
    static mut DAT_100a7b48: i32;
    static mut DAT_100aaf70: i32;
    static mut DAT_100aafa0: i32;
    static mut DAT_100aafa4: i32;
    static mut DAT_100aaf8c: i32;
    static mut DAT_100aaf90: i32;
    static mut DAT_100ab094: i32;
    static mut DAT_100ad69c: i32;
    static mut DAT_100b0f18: i32;
    static mut DAT_100af098: i32;
    static mut DAT_100c9678: u32;
    static mut DAT_101f5300: *mut u32;
    static mut DAT_101f52e0: [u8; 0];
    static mut DAT_10160c00: [i32; 0];
    static mut DAT_100aaf68: *mut i8;
    static mut DAT_100aafc8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003bed0();
    fn FUN_1003bc5a();
    fn FUN_1003c0e4();
    fn FUN_100603ff(param_1: i32);
    fn FUN_1003b59a(param_1: i32);
    fn FUN_10058e74(param_1: *mut c_void, param_2: i8);
    fn FUN_1003b954(param_1: *mut i32);
    fn FUN_1003b2f0(param_1: *mut i8);
    fn FUN_100186c5(param_1: i32);
    fn FUN_1000dde1(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_1003b8f6(param_1: i32, param_2: i32);
    fn FUN_10054c14(param_1: i32);
    fn FUN_1003cd85(param_1: *mut i32);
    fn FUN_10054925(param_1: i32);
    fn FUN_1003b8cd(param_1: i32, param_2: i32);
    fn FUN_1000f06a(param_1: i32);
    fn FUN_1005f8cf();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e953() {
    // SAFETY: all globals and callees are the game's own; pointers held in
    // globals refer to the game's buffers, accessed as the original does.
    unsafe {
        let mut u_var4: u32 = DAT_100c9678;
        if DAT_100a7b68 != 0 {
            let u_var2: u32 = (DAT_100ad69c as u8) as u32;
            let mut puvar5: *mut u32 = DAT_101f5300;
            let mut u_var3: u32 = DAT_100c9678 >> 2;
            while u_var3 != 0 {
                puvar5.write_unaligned((((u_var2 << 8) | u_var2) << 16) | ((u_var2 << 8) | u_var2));
                puvar5 = puvar5.add(1);
                u_var3 = u_var3.wrapping_sub(1);
            }
            u_var4 &= 3;
            while u_var4 != 0 {
                (puvar5 as *mut u8).write(u_var2 as u8);
                puvar5 = (puvar5 as *mut u8).add(1) as *mut u32;
                u_var4 = u_var4.wrapping_sub(1);
            }
            FUN_100603ff(DAT_100a7b68);
        }
        if DAT_100a7b60 != 0 {
            FUN_1003b59a(DAT_100aaf68 as i32);
            DAT_100a7b60 = 0;
        }
        if DAT_100aaf70 == 0 {
            FUN_1003b954(DAT_100aaf68 as *mut i32);
            FUN_1003b2f0(DAT_100aaf68);
            if DAT_100aafa0 == 0 && DAT_100aafa4 == 0 {
                if DAT_100aaf8c != 0 || DAT_100aaf90 != 0 {
                    FUN_100186c5(DAT_100aaf68 as i32);
                }
            } else {
                FUN_10058e74((&raw mut DAT_101f52e0).cast::<c_void>(), DAT_100ad69c as i8);
            }
            if DAT_100a7b6c != 0 && DAT_100a7b70 != 0 && DAT_100a7b74 != -1 {
                FUN_1000dde1(
                    DAT_100a7b74,
                    DAT_100aaf68.byte_add(0x1c) as *mut u32,
                    DAT_100aaf68.byte_add(0x20) as *mut u32,
                    DAT_100aaf68.byte_add(0x24) as *mut u32,
                );
            }
            DAT_100a7b80 = 0;
            if DAT_100a7b78 != 0 {
                let i_var1 = DAT_100aaf68.byte_add(0x40).cast::<i32>().read_unaligned();
                FUN_1003b8f6(DAT_100aaf68 as i32, 0x7fffffff);
                DAT_100aafc8 = FUN_1003bed0 as usize as *mut c_void;
                FUN_10054c14(DAT_100a7b78);
                DAT_100a7b80 = (DAT_100a7b80 as u32).wrapping_add(DAT_100b0f18 as u32) as i32;
                FUN_1003b8f6(DAT_100aaf68 as i32, i_var1);
                DAT_100aafc8 = FUN_1003bc5a as usize as *mut c_void;
            }
            FUN_1003cd85(DAT_100aaf68 as *mut i32);
            let arr = (&raw mut DAT_10160c00).cast::<i32>();
            let mut local_10: usize = 0;
            let mut local_8: i32 = arr.read();
            while local_8 != 0 {
                FUN_10054925(local_8);
                local_10 = local_10.wrapping_add(1);
                local_8 = arr.add(local_10).read();
            }
            DAT_100a7b80 = (DAT_100a7b80 as u32).wrapping_add(DAT_100b0f18 as u32) as i32;
            if DAT_100ab094 != 0 && DAT_100a7b7c != 0 && DAT_100a7b4c != 0 {
                let i_var1 = DAT_100aaf68.byte_add(0x3c).cast::<i32>().read_unaligned();
                FUN_1003b8cd(DAT_100aaf68 as i32, 8);
                DAT_100aafc8 = FUN_1003c0e4 as usize as *mut c_void;
                FUN_10054c14(DAT_100a7b7c);
                DAT_100a7b80 = (DAT_100a7b80 as u32).wrapping_add(DAT_100b0f18 as u32) as i32;
                FUN_1003b8cd(DAT_100aaf68 as i32, i_var1);
                DAT_100aafc8 = FUN_1003bc5a as usize as *mut c_void;
            }
            if DAT_100a7b48 != 0 {
                FUN_1000f06a(DAT_100af098);
            }
            FUN_1005f8cf();
            FUN_100603ff(0);
        } else {
            FUN_10058e74((&raw mut DAT_101f52e0).cast::<c_void>(), DAT_100ad69c as i8);
        }
    }
}
