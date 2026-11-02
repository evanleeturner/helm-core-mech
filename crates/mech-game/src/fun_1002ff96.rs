// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ff96(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1002ff96", param_1, param_2);
    let mut local_10: i32 = 0;
    let mut local_c: i32;
    let mut local_8: i32;

    if param_1 == 0 {
        local_8 = 0;
    } else {
        // SAFETY: param_1 points at a node whose field at +4 is the list head;
        // each node's next pointer is at +8, read as the original does.
        unsafe {
            local_c = mech_trace::rd!(
                "1002ff96",
                ((param_1 as u32).wrapping_add(4) as *const i32),
                read_unaligned
            );
            if local_c == 0 {
                local_8 = 0;
            } else {
                while local_c != 0 {
                    local_10 = local_10.wrapping_add(1);
                    local_c = mech_trace::rd!(
                        "1002ff96",
                        ((local_c as u32).wrapping_add(8) as *const i32),
                        read_unaligned
                    );
                }
                local_10 = local_10.wrapping_add((-1i32).wrapping_sub(param_2));
                if local_10 < 0 {
                    local_8 = 0;
                } else {
                    local_8 = mech_trace::rd!(
                        "1002ff96",
                        ((param_1 as u32).wrapping_add(4) as *const i32),
                        read_unaligned
                    );
                    while local_10 != 0 && local_8 != 0 {
                        local_10 = local_10.wrapping_sub(1);
                        local_8 = mech_trace::rd!(
                            "1002ff96",
                            ((local_8 as u32).wrapping_add(8) as *const i32),
                            read_unaligned
                        );
                    }
                }
            }
        }
    }
    mech_trace::rt!("1002ff96", local_8)
}
