// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d4bc(
    param_1: i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: i32,
    param_6: u32,
) {
    // SAFETY: all pointers and addresses are the game's own structures,
    // read and written at the offsets the original uses.
    unsafe {
        let i_var2 = ((param_1 as u32).wrapping_add(0x20) as *const i32).read_unaligned();
        if i_var2 != 0 {
            let off = ((i_var2 as u32).wrapping_add(8) as *const i32).read_unaligned();
            let p_var3 = (off as u32)
                .wrapping_add(i_var2 as u32)
                .wrapping_add((param_2 as u32).wrapping_mul(0x24))
                as *const u16;
            if !param_3.is_null() {
                param_3.write_unaligned(u32::from(p_var3.read_unaligned()));
            }
            let mut local_10 = u32::from(p_var3.byte_add(2).read_unaligned());
            if !param_4.is_null() {
                param_4.write_unaligned(local_10);
            }
            if param_5 != 0 {
                if (param_6 as i32) < (local_10 as i32) {
                    local_10 = param_6;
                }
                let base = p_var3.byte_add(4).cast::<i32>().read_unaligned() as u32;
                while local_10 != 0 {
                    let u_var1 = local_10.wrapping_sub(1);
                    let b =
                        (base.wrapping_add(u_var1).wrapping_add(p_var3 as u32) as *const u8).read();
                    ((param_5 as u32).wrapping_add(u_var1.wrapping_mul(4)) as *mut u32)
                        .write_unaligned(u32::from(b));
                    local_10 = u_var1;
                }
            }
        }
    }
}
