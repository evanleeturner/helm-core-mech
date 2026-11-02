// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b5ea(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the pointers are derived from the caller's buffers exactly as
    // the original computes them; reads and writes happen in the same order.
    unsafe {
        let mut i_var2 = ((param_1
            .wrapping_add(param_2.wrapping_mul(8))
            .wrapping_add(0xc)) as *const i32)
            .read_unaligned();
        if i_var2 != 0 {
            let mut pb_var3 = i_var2.wrapping_add(param_1) as *const u8;
            i_var2 = (pb_var3 as *const i32).read_unaligned();
            loop {
                let i_var1 = (pb_var3.byte_add(4).read() as i32).wrapping_mul(3);
                let dst = i_var1.wrapping_add(param_3) as *mut u8;
                dst.write(pb_var3.byte_add(5).read());
                let w = pb_var3.byte_add(6).cast::<u16>().read_unaligned();
                (i_var1.wrapping_add(1).wrapping_add(param_3) as *mut u16).write_unaligned(w);
                i_var2 = i_var2.wrapping_sub(1);
                pb_var3 = pb_var3.wrapping_byte_add(4);
                if i_var2 == 0 {
                    break;
                }
            }
        }
    }
}
