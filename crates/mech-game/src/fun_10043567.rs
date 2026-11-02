// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_10044054(param_1: *mut i32) -> u32;
    fn FUN_10044c17(param_1: u16) -> i32;
    fn FUN_100297b4(param_1: i32);
    fn FUN_100295f4(param_3: *mut i32, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043567(param_1: *mut i32, param_2: u16) -> u8 {
    // SAFETY: the callees are the game's own; param_1 points at the game's
    // object, read and written at the same offsets as the original.
    unsafe {
        FUN_1002950f(param_1 as i32, param_2 as i16);
        let u_var1 = FUN_10044054(param_1);
        let mut skip = false;
        if u_var1 != 0 {
            let i_var2 = FUN_10044c17(param_2);
            if i_var2 == param_1.add(0x77).read_unaligned() {
                skip = true;
            }
        }
        if !skip {
            FUN_100297b4(param_1 as i32);
            let u_var3 = FUN_100295f4(param_1, 0);
            let p = param_1.add(0x13).read_unaligned();
            (p as usize).wrapping_add(8) as *mut i32;
            ((p as u32).wrapping_add(8) as *mut i32).write_unaligned(u_var3);
        }
        let i_var2 = param_1.add(0x31).read_unaligned();
        if i_var2 < 0x7d1 {
            let p = param_1.add(0x13).read_unaligned();
            ((p as u32).wrapping_add(0x44) as *mut u8).write(1);
        }
        (i_var2 < 0x7d1) as u8
    }
}
