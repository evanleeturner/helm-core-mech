// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a18a(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the pointers are derived from the caller's addresses and
    // are read and written exactly where the original does.
    unsafe {
        let off = param_1
            .wrapping_add(param_2.wrapping_mul(8))
            .wrapping_add(0xc);
        let mut i_var2 = (off as u32 as usize as *const i32).read_unaligned();
        if i_var2 != 0 {
            let mut pb = i_var2_ptr(i_var2, param_1);
            i_var2 = (pb as *const i32).read_unaligned();
            loop {
                let i_var1 = (pb.byte_add(4).read() as i32).wrapping_mul(3);
                let dst = i_var1.wrapping_add(param_3);
                (dst as u32 as usize as *mut u8).write(pb.byte_add(5).read());
                (dst.wrapping_add(1) as u32 as usize as *mut u16)
                    .write_unaligned(pb.byte_add(6).cast::<u16>().read_unaligned());
                i_var2 = i_var2.wrapping_sub(1);
                pb = pb.byte_add(4);
                if i_var2 == 0 {
                    break;
                }
            }
        }
    }
}

#[inline(always)]
fn i_var2_ptr(offset: i32, base: i32) -> *const u8 {
    offset.wrapping_add(base) as u32 as usize as *const u8
}
