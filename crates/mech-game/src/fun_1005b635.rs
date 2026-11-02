// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b635(param_1: i32, param_2: i32, param_3: *mut u32) -> *mut u32 {
    // SAFETY: param_1 is the address of the caller's table, and the offsets
    // read and written below are the ones the original uses.
    unsafe {
        let addr = param_1
            .wrapping_add(param_2.wrapping_mul(8))
            .wrapping_add(0xc);
        let i_var1 = (addr as usize as *const i32).read_unaligned();
        if i_var1 == 0 {
            return core::ptr::null_mut();
        }
        let mut pi_var5 = i_var1.wrapping_add(param_1) as usize as *const i32;
        let pu_var3 = pi_var5.read_unaligned() as u32;
        let mut pu_var4 = pu_var3;
        let mut dst = param_3;
        let mut cond = param_3 as usize as u32;
        while cond != 0 {
            pi_var5 = pi_var5.byte_add(4);
            dst.write_unaligned(pi_var5.read_unaligned() as u32);
            pu_var4 = pu_var4.wrapping_sub(1);
            dst = dst.byte_add(4);
            cond = pu_var4;
        }
        pu_var3 as usize as *mut u32
    }
}
