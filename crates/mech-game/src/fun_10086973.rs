// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086973(
    param_1: *mut u32,
    param_2: *mut u32,
    param_3: u32,
) -> *mut u32 {
    let mut p_var2 = param_1;
    let mut src = param_2;
    let mut u_var1 = param_3 >> 2;
    // SAFETY: the caller supplies buffers valid for param_3 bytes; copies
    // proceed in the original's order, dwords first then trailing bytes.
    unsafe {
        while u_var1 != 0 {
            p_var2.write_unaligned(src.read_unaligned());
            src = src.byte_add(4);
            p_var2 = p_var2.byte_add(4);
            u_var1 = u_var1.wrapping_sub(1);
        }
        u_var1 = param_3 & 3;
        while u_var1 != 0 {
            p_var2.cast::<u8>().write(src.cast::<u8>().read());
            src = src.byte_add(1);
            p_var2 = p_var2.byte_add(1);
            u_var1 = u_var1.wrapping_sub(1);
        }
    }
    param_1
}
