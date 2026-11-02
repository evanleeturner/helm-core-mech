// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b0d9(param_1: i32, param_2: i32, param_3: *mut u8) {
    let mut i_var1: i32 = 0x300;
    let mut pb_var2: *const u8 =
        param_1.wrapping_add(param_2).wrapping_sub(0x300) as usize as *const u8;
    let mut dst: *mut u8 = param_3;
    loop {
        // SAFETY: the caller supplies a source of 0x300 bytes at param_1 + param_2 - 0x300
        // and a destination of 0x300 bytes at param_3.
        unsafe {
            *dst = *pb_var2 >> 2;
            pb_var2 = pb_var2.add(1);
            dst = dst.add(1);
        }
        i_var1 = i_var1.wrapping_sub(1);
        if i_var1 == 0 {
            break;
        }
    }
}
