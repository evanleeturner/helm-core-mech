// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100497e6(param_1: i32, param_2: u32) -> bool {
    // SAFETY: param_1 points at the caller's structure: a count at offset 0xc
    // and a pointer to a buffer of 4-byte slots at offset 8, accessed as the original does.
    unsafe {
        let count = ((param_1 as usize).wrapping_add(0xc) as *const i32).read_unaligned();
        let b_var1 = count < 8;
        if b_var1 {
            let buf = ((param_1 as usize).wrapping_add(8) as *const i32).read_unaligned();
            let c = ((param_1 as usize).wrapping_add(0xc) as *const i32).read_unaligned();
            let addr = (buf.wrapping_add(c.wrapping_mul(4))) as usize as *mut u32;
            addr.write_unaligned(param_2);
            let c2 = ((param_1 as usize).wrapping_add(0xc) as *const i32).read_unaligned();
            ((param_1 as usize).wrapping_add(0xc) as *mut i32).write_unaligned(c2.wrapping_add(1));
        }
        b_var1
    }
}
