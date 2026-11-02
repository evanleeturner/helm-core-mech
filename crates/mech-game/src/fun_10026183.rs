// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10026183(param_1: i32) -> i32 {
    mech_trace::enter!("10026183", param_1);
    let mut local_8: i32;
    if param_1 == 0 {
        local_8 = 0;
    } else {
        // SAFETY: param_1 is the address of a node in the game's own memory;
        // fields are read at the offsets the original uses.
        unsafe {
            local_8 = mech_trace::rd!(
                "10026183",
                ((param_1 as usize + 0x18) as *const i32),
                read_unaligned
            )
            .wrapping_mul(4)
            .wrapping_add(0x2c);
            let mut local_c: i32 = 0;
            while local_c < 4 {
                let child = mech_trace::rd!(
                    "10026183",
                    ((param_1 as usize)
                        .wrapping_add(0x1c)
                        .wrapping_add((local_c as usize).wrapping_mul(4))
                        as *const i32),
                    read_unaligned
                );
                let i_var1 = FUN_10026183(child);
                local_8 = local_8.wrapping_add(i_var1);
                local_c += 1;
            }
        }
    }
    mech_trace::rt!("10026183", local_8)
}
