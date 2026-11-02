// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083720: u32;
    static mut DAT_10083738: u32;
    static mut DAT_1008373c: u32;
    static mut DAT_10083740: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a180(param_1: i32) -> *mut u32;
    fn FUN_1000a3f0(param_1: i32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004660() {
    // SAFETY: the globals and callees are the game's own; they are read,
    // written and called in the original's order.
    unsafe {
        let mut i_var2 = DAT_10083740;
        DAT_10083740 = ((DAT_10083720 as i32) / -0x19)
            .wrapping_add((DAT_1008373c as i32) / 100)
            .wrapping_add(10);
        if DAT_10083740 < 0 {
            DAT_10083740 = 0;
        }
        if -1 < i_var2 && i_var2 != DAT_10083740 {
            if DAT_10083740 < i_var2 {
                loop {
                    let i_var1 = i_var2.wrapping_add(8000);
                    i_var2 = i_var2.wrapping_sub(1);
                    FUN_1000a180(i_var1);
                    if !(DAT_10083740 < i_var2) {
                        break;
                    }
                }
            }
            if i_var2 < DAT_10083740 {
                loop {
                    let i_var1 = i_var2.wrapping_add(0x1f41);
                    i_var2 = i_var2.wrapping_add(1);
                    FUN_1000a3f0(i_var1, DAT_10083738);
                    if !(i_var2 < DAT_10083740) {
                        break;
                    }
                }
            }
        }
    }
}
