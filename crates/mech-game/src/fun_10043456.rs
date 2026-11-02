// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a11f0: f64;
    static mut DAT_100a11f8: f64;
    static mut DAT_100a1200: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: *mut i32, param_2: i16);
    fn FUN_10044054(param_1: *mut i32) -> i32;
    fn FUN_100297b4(param_1: *mut i32) -> i32;
    fn FUN_100295f4(param_1: *mut i32, param_2: i32) -> i64;
    fn FUN_1002a132(param_1: *mut i32) -> i32;
    fn FUN_10022b70(param_1: *mut i32, param_2: i32);
    fn FUN_10044a2b(param_1: *mut i32) -> i32;
    fn FUN_10002570(param_1: i32, param_2: i32) -> i32;
    fn FUN_10044a83(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043456(param_1: *mut i32, param_2: i16) -> i32 {
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the game object, with fields read and written at the original's offsets.
    unsafe {
        FUN_1002950f(param_1, param_2);
        let local_10: i32;
        if FUN_10044054(param_1) == 0 {
            local_10 = FUN_100297b4(param_1);
            let u_var3 = FUN_100295f4(param_1, 0x157c);
            let p = param_1.byte_add(0x4c).cast::<i32>().read_unaligned() as u32 as *mut i32;
            p.byte_add(8).write_unaligned(u_var3 as i32);
        } else {
            local_10 = FUN_1002a132(param_1);
        }
        FUN_10022b70(param_1, local_10);
        let i_var2 = FUN_10044a2b(param_1);
        let i_var2 = FUN_10002570(((i_var2 as u32).wrapping_shl(0x10)) as i32, 500000);
        let mut local_c: f64 = (i_var2 as f64) / DAT_100a11f0;
        if local_c < DAT_100a11f8 {
            local_c = 1.0;
        }
        local_c *= DAT_100a1200;
        let i_var2 = param_1.byte_add(0xc0).cast::<i32>().read_unaligned();
        FUN_10044a83(param_1);
        ((i_var2 as f64) <= local_c) as i32
    }
}
