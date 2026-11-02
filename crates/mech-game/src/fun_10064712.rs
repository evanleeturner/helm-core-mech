// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6b8: u32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064712(param_1: i32) -> bool {
    // SAFETY: the globals and the callee are the game's own; the array is
    // indexed where the original indexes it.
    unsafe {
        let mut local_c: i32 = 0x206;
        if param_1 != DAT_100adf38 {
            local_c = 0x250;
        }
        let i_var1 = FUN_100814bb(local_c, DAT_100ad6b8 as i32);
        if i_var1 != 0 {
            (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(param_1 as isize)
                .write_unaligned(i_var1 as u32);
        }
        i_var1 != 0
    }
}
