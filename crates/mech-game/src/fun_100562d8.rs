// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100562d8(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: u8,
) -> u32 {
    // SAFETY: param_1 points at the caller's descriptor, whose first field
    // points at a second descriptor; the pixel pointer is read and written
    // where the original does.
    unsafe {
        let pi_var2 = param_1.read_unaligned() as *const i32;
        let i_var3 = pi_var2.byte_add(4).read_unaligned();
        if i_var3.wrapping_add(1) == 0 || i_var3 < -1 {
            return 0xffff_ffff;
        }
        let i_var4 = pi_var2.byte_add(8).read_unaligned();
        if i_var4 < 0 {
            return 0xffff_ffff;
        }
        let mut i_var10 = param_1.byte_add(4).read_unaligned();
        let mut i_var5 = i_var10;
        if i_var10 < 1 {
            i_var5 = 0;
        }
        let mut i_var11 = param_1.byte_add(8).read_unaligned();
        let mut i_var6 = i_var11;
        if i_var11 < 1 {
            i_var6 = 0;
        }
        let mut i_var7 = param_1.byte_add(12).read_unaligned();
        if i_var3 <= i_var7 {
            i_var7 = i_var3;
        }
        let mut i_var8 = param_1.byte_add(16).read_unaligned();
        if i_var4 <= i_var8 {
            i_var8 = i_var4;
        }
        if i_var5 <= i_var7 && i_var6 <= i_var8 {
            i_var10 = param_2.wrapping_add(i_var10);
            i_var11 = param_3.wrapping_add(i_var11);
            if i_var5 <= i_var10 && i_var10 <= i_var7 && i_var6 <= i_var11 && i_var11 <= i_var8 {
                let base = pi_var2.read_unaligned();
                let addr = i_var11
                    .wrapping_mul(i_var3.wrapping_add(1))
                    .wrapping_add(base)
                    .wrapping_add(i_var10);
                let pbvar9 = addr as *mut u8;
                let b_var1 = pbvar9.read();
                pbvar9.write(param_4);
                return u32::from(b_var1);
            }
            return 0xffff_fffd;
        }
        0xffff_fffe
    }
}
