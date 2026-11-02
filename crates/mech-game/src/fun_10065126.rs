// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006528b();
    fn FUN_10065555() -> i32;
    fn FUN_10065786(param_1: u8);
    fn FUN_10065350(param_1: u32, param_2: i32);
    fn FUN_100656d3(param_1: u32, param_2: u8) -> u32;
    fn FUN_100655f6(param_1: u32, param_2: u8) -> u32;
    fn FUN_100652b6();
    fn FUN_100654db();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065126(param_1: i32, param_2: *mut c_void) -> i32 {
    // SAFETY: the callees are the game's own; param_2 is a caller-supplied
    // __cdecl function pointer returning an unsigned int.
    unsafe {
        let mut local_24: u32 = 0xffff;
        FUN_1006528b();
        let u_var1: u32 = 1u32.wrapping_shl(((param_1 as u8) & 0x1f) as u32);
        let mut local_8: i32 = param_1.wrapping_add(1);
        let mut local_18: u32 = 1u32
            .wrapping_shl(((local_8 as u8) & 0x1f) as u32)
            .wrapping_sub(1);
        let mut i_var2 = FUN_10065555();
        if i_var2 == 0 {
            FUN_10065786(param_1 as u8);
            FUN_10065350(u_var1, local_8);
            let callback: unsafe extern "C" fn() -> u32 = core::mem::transmute(param_2);
            let mut u_var4 = local_24;
            loop {
                local_24 = u_var4;
                let u_var3 = callback();
                if u_var3 == 0xffff_ffff {
                    break;
                }
                u_var4 = FUN_100656d3(local_24, u_var3 as u8);
                if u_var4 == 0xffff {
                    FUN_10065350(local_24, local_8);
                    let u_var5 = FUN_100655f6(local_24, u_var3 as u8);
                    u_var4 = u_var3;
                    if local_18 < u_var5 {
                        i_var2 = local_8.wrapping_add(1);
                        if 0xc < i_var2 {
                            FUN_10065350(u_var1, local_8);
                            FUN_10065786(param_1 as u8);
                            i_var2 = param_1.wrapping_add(1);
                        }
                        local_8 = i_var2;
                        local_18 = 1u32
                            .wrapping_shl(((local_8 as u8) & 0x1f) as u32)
                            .wrapping_sub(1);
                    }
                }
            }
            if local_24 != 0xffff {
                FUN_10065350(local_24, local_8);
            }
            FUN_10065350(u_var1.wrapping_add(1), local_8);
            FUN_100652b6();
            FUN_100654db();
            i_var2 = 0;
        }
        i_var2
    }
}
