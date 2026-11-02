// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005adf0(param_1: *mut i16, param_2: i32) -> *mut i16 {
    let mut ps_var2: *mut i16 = param_2.wrapping_add(0xc) as *mut i16;
    loop {
        // SAFETY: the pointers walk the game's own record table, which the
        // original reads in the same order; the scan ends at a nonzero byte.
        unsafe {
            while ps_var2.cast::<u8>().read() == 0 {
                ps_var2 = ps_var2.cast::<u8>().byte_add(1).cast::<i16>();
            }
            let mut i_var1: i32 = 2;
            let mut b_var5 = false;
            let mut ps_var3: *const i16 = ps_var2;
            let mut ps_var4: *const i16 = param_1;
            loop {
                if i_var1 == 0 {
                    break;
                }
                i_var1 -= 1;
                b_var5 = ps_var3.read_unaligned() == ps_var4.read_unaligned();
                ps_var3 = ps_var3.byte_add(2);
                ps_var4 = ps_var4.byte_add(2);
                if !b_var5 {
                    break;
                }
            }
            if b_var5 {
                return ps_var2.byte_add(8);
            }
            let len = ps_var2.byte_add(6).read_unaligned() as u16 as usize;
            ps_var2 = ps_var2.byte_add(14).byte_add(len);
        }
    }
}
