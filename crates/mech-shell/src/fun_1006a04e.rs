// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a04e(param_1: i32, param_2: *mut u8) {
    let mut pb_var2: *const u8 = param_1.wrapping_add(0xd) as *const u8;
    let mut out: *mut u8 = param_2;
    // SAFETY: param_1 is the address of the caller's buffer and param_2 points
    // at its output palette; reads and writes follow the original's offsets.
    unsafe {
        let flags = *(param_1.wrapping_add(10) as *const u8);
        if (flags & 0x80) != 0 {
            let mut i_var1: i32 =
                ((1u32.wrapping_shl(((flags & 7) as u32).wrapping_add(1))) as i32).wrapping_mul(3);
            let mut pb_var3 = pb_var2;
            loop {
                pb_var2 = pb_var3.add(1);
                *out = *pb_var3 >> 2;
                i_var1 = i_var1.wrapping_sub(1);
                pb_var3 = pb_var2;
                out = out.add(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let f2 = *pb_var2.add(9);
        if (f2 & 0x80) != 0 {
            let mut i_var1: i32 =
                ((1u32.wrapping_shl(((f2 & 7) as u32).wrapping_add(1))) as i32).wrapping_mul(3);
            pb_var2 = pb_var2.add(10);
            loop {
                *out = *pb_var2 >> 2;
                i_var1 = i_var1.wrapping_sub(1);
                pb_var2 = pb_var2.add(1);
                out = out.add(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
    }
}
