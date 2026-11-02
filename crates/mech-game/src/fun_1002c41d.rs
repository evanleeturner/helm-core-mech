// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002c41d(param_1: *mut i32) {
    mech_trace::enter!("1002c41d", param_1);
    // SAFETY: param_1 points at the caller's game object; param_1[0x13] is a
    // pointer to a sub-structure, accessed at the offsets the original uses.
    unsafe {
        let local_8: i32;
        let sub =
            mech_trace::rd!("1002c41d", param_1.add(0x13), read_unaligned) as usize as *mut u8;
        if mech_trace::rd!("1002c41d", sub.byte_add(8).cast::<i32>(), read_unaligned) == 0 {
            local_8 = 0x40;
        } else {
            local_8 = 0x20;
        }
        if mech_trace::rd!("1002c41d", param_1, read_unaligned) == 1 {
            if local_8.wrapping_add(mech_trace::gr!("1002c41d", "100b38fc", DAT_100b38fc))
                < mech_trace::rd!("1002c41d", param_1.add(0x73), read_unaligned)
            {
                mech_trace::wr!(
                    "1002c41d",
                    param_1.add(0x73),
                    write_unaligned,
                    local_8.wrapping_add(mech_trace::gr!("1002c41d", "100b38fc", DAT_100b38fc))
                );
            }
            if mech_trace::rd!("1002c41d", param_1.add(0x73), read_unaligned)
                <= mech_trace::gr!("1002c41d", "100b38fc", DAT_100b38fc)
            {
                let sub = mech_trace::rd!("1002c41d", param_1.add(0x13), read_unaligned) as usize
                    as *mut u8;
                if mech_trace::rd!("1002c41d", param_1.add(0x75), read_unaligned) == 0 {
                    let v =
                        mech_trace::rd!("1002c41d", sub.byte_add(4).cast::<i32>(), read_unaligned);
                    mech_trace::wr!(
                        "1002c41d",
                        sub.byte_add(4).cast::<i32>(),
                        write_unaligned,
                        v.wrapping_add(-0x50000)
                    );
                } else {
                    let v =
                        mech_trace::rd!("1002c41d", sub.byte_add(4).cast::<i32>(), read_unaligned);
                    mech_trace::wr!(
                        "1002c41d",
                        sub.byte_add(4).cast::<i32>(),
                        write_unaligned,
                        v.wrapping_add(0x50000)
                    );
                }
                let sub = mech_trace::rd!("1002c41d", param_1.add(0x13), read_unaligned) as usize
                    as *mut u8;
                let u_var1 =
                    (mech_trace::rd!("1002c41d", sub.byte_add(4).cast::<u32>(), read_unaligned))
                        >> 0x1f;
                let val =
                    mech_trace::rd!("1002c41d", sub.byte_add(4).cast::<u32>(), read_unaligned);
                if 0x2d0000 < ((val ^ u_var1).wrapping_sub(u_var1) as i32) {
                    if mech_trace::rd!("1002c41d", param_1.add(0x75), read_unaligned) == 0 {
                        mech_trace::wr!("1002c41d", param_1.add(0x75), write_unaligned, 1);
                    } else {
                        mech_trace::wr!("1002c41d", param_1.add(0x75), write_unaligned, 0);
                    }
                }
                mech_trace::wr!(
                    "1002c41d",
                    param_1.add(0x73),
                    write_unaligned,
                    local_8.wrapping_add(mech_trace::gr!("1002c41d", "100b38fc", DAT_100b38fc))
                );
            }
        }
    }
}
