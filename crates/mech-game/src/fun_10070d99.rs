// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c81e0: u8;
    static mut DAT_100c81fc: u32;
    static mut PTR_DAT_100b4828: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn fread(dst: *mut c_void, size: usize, count: usize, file: *mut c_void) -> usize;
    fn ftell(file: *mut c_void) -> i32;
    fn fseek(file: *mut c_void, offset: i32, origin: i32) -> i32;
    fn FUN_10070cd5(param_1: i32, param_2: *mut u32, param_3: *mut u32) -> u8;
    fn FUN_100a0320(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070d99(param_1: *mut u32) {
    mech_trace::enter!("10070d99", param_1);
    let mut local_c: u32 = 0;
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's structure, accessed at the same offsets as the original.
    unsafe {
        loop {
            let s_var1 = mech_trace::cr!(
                "10070d99",
                "fread",
                fread(
                    mech_trace::gr!("10070d99", "PTR_DAT_100b4828", PTR_DAT_100b4828),
                    1,
                    0x200,
                    mech_trace::rd!("10070d99", param_1, read) as *mut c_void
                )
            );
            if s_var1 != 0x200 {
                return;
            }
            mech_trace::cr!(
                "10070d99",
                "10070cd5",
                FUN_10070cd5(
                    mech_trace::gr!("10070d99", "PTR_DAT_100b4828", PTR_DAT_100b4828) as i32,
                    &raw mut DAT_100c81e0 as *mut u32,
                    &raw mut local_c,
                )
            );
            if mech_trace::gr!("10070d99", "100c81fc", DAT_100c81fc) == 0 {
                return;
            }
            if (mech_trace::rd!("10070d99", param_1.add(0x12d), read) as i32) >= 100 {
                return;
            }
            let i_var3 = mech_trace::rd!("10070d99", param_1.add(0x12d), read) as i32;
            let base = i_var3.wrapping_mul(3);
            let local_8 = param_1.offset(base.wrapping_add(1) as isize) as *mut i32;
            mech_trace::wr!(
                "10070d99",
                param_1.add(0x12d),
                write,
                (mech_trace::rd!("10070d99", param_1.add(0x12d), read) as i32).wrapping_add(1)
                    as u32
            );
            mech_trace::wr!(
                "10070d99",
                param_1.offset(base.wrapping_add(3) as isize),
                write,
                mech_trace::gr!("10070d99", "100c81fc", DAT_100c81fc)
            );
            let l_var2 = mech_trace::cr!(
                "10070d99",
                "ftell",
                ftell(mech_trace::rd!("10070d99", param_1, read) as *mut c_void)
            );
            mech_trace::wr!("10070d99", local_8.add(1), write, l_var2);
            let i_var3 = mech_trace::cr!(
                "10070d99",
                "100a0320",
                FUN_100a0320(
                    mech_trace::gr!("10070d99", "PTR_DAT_100b4828", PTR_DAT_100b4828) as *mut u32
                )
            ) as i32;
            mech_trace::wr!("10070d99", local_8, write, i_var3);
            if mech_trace::rd!("10070d99", local_8, read) == 0 {
                return;
            }
            mech_trace::cr!(
                "10070d99",
                "fseek",
                fseek(
                    mech_trace::rd!("10070d99", param_1, read) as *mut c_void,
                    mech_trace::gr!("10070d99", "100c81fc", DAT_100c81fc) as i32,
                    1
                )
            );
            let u_var4 =
                ((mech_trace::gr!("10070d99", "100c81fc", DAT_100c81fc) as i32) >> 0x1f) as u32;
            let a = (mech_trace::gr!("10070d99", "100c81fc", DAT_100c81fc) ^ u_var4)
                .wrapping_sub(u_var4)
                & 0x1ff;
            let b = (a ^ u_var4).wrapping_sub(u_var4);
            let mut local_10 = 0x200i32.wrapping_sub(b as i32);
            if local_10 == 0x200 {
                local_10 = 0;
            }
            mech_trace::cr!(
                "10070d99",
                "fseek",
                fseek(
                    mech_trace::rd!("10070d99", param_1, read) as *mut c_void,
                    local_10,
                    1
                )
            );
        }
    }
}
