// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c8a3(param_1: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c8cf(param_1: i32) {
    mech_trace::enter!("1001c8cf", param_1);
    // SAFETY: param_1 points at the game's own structure; its list nodes are
    // read and written at the same offsets and order as the original.
    unsafe {
        let pv_var1 = mech_trace::rd!(
            "1001c8cf",
            ((param_1 as usize + 0x20) as *const *mut u8),
            read_unaligned
        );
        if !pv_var1.is_null()
            && mech_trace::rd!(
                "1001c8cf",
                ((param_1 as usize + 0x1c) as *const i32),
                read_unaligned
            ) != 0
        {
            let head = mech_trace::rd!(
                "1001c8cf",
                ((param_1 as usize + 0x1c) as *const *mut u8),
                read_unaligned
            );
            if head == pv_var1 {
                let next = (pv_var1 as usize + 0xc) as *const u32;
                mech_trace::wr!(
                    "1001c8cf",
                    ((param_1 as usize + 0x1c) as *mut u32),
                    write_unaligned,
                    mech_trace::rd!("1001c8cf", next, read_unaligned)
                );
            } else {
                let mut local_8 = mech_trace::rd!(
                    "1001c8cf",
                    ((param_1 as usize + 0x1c) as *const i32),
                    read_unaligned
                );
                loop {
                    let n = mech_trace::rd!(
                        "1001c8cf",
                        ((local_8 as usize + 0xc) as *const i32),
                        read_unaligned
                    );
                    if n != 0 && n as usize as *mut u8 != pv_var1 {
                        local_8 = n;
                    } else {
                        break;
                    }
                }
                if mech_trace::rd!(
                    "1001c8cf",
                    ((local_8 as usize + 0xc) as *const i32),
                    read_unaligned
                ) == 0
                {
                    return;
                }
                let next = (pv_var1 as usize + 0xc) as *const u32;
                mech_trace::wr!(
                    "1001c8cf",
                    ((local_8 as usize + 0xc) as *mut u32),
                    write_unaligned,
                    mech_trace::rd!("1001c8cf", next, read_unaligned)
                );
            }
            FUN_1001c8a3(pv_var1);
        }
    }
}
