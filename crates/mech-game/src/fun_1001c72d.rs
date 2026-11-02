// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c72d(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
) {
    mech_trace::enter!(
        "1001c72d", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: param_1 points at the game's object; its field at 0x20 is either
    // null or a pointer to a record whose entry array is written where the
    // original does.
    unsafe {
        let i_var2 = (param_1 as u32).wrapping_add(0x20) as *const i32;
        let i_var2 = mech_trace::rd!("1001c72d", i_var2, read_unaligned);
        if i_var2 != 0 {
            let count = mech_trace::rd!(
                "1001c72d",
                ((i_var2 as u32).wrapping_add(4) as *const i16),
                read_unaligned
            );
            let p = (i_var2 as u32)
                .wrapping_add(0x18)
                .wrapping_add((count as i32).wrapping_mul(0x44) as u32)
                as *mut u32;
            mech_trace::wr!("1001c72d", p, write_unaligned, param_2);
            mech_trace::wr!(
                "1001c72d",
                p.add(3),
                write_unaligned,
                mech_trace::rd!("1001c72d", p, read_unaligned)
            );
            mech_trace::wr!("1001c72d", p.add(1), write_unaligned, param_3);
            mech_trace::wr!(
                "1001c72d",
                p.add(4),
                write_unaligned,
                mech_trace::rd!("1001c72d", p.add(1), read_unaligned)
            );
            mech_trace::wr!("1001c72d", p.add(2), write_unaligned, param_4);
            mech_trace::wr!(
                "1001c72d",
                p.add(5),
                write_unaligned,
                mech_trace::rd!("1001c72d", p.add(2), read_unaligned)
            );
            mech_trace::wr!("1001c72d", p.add(6), write_unaligned, param_5);
            mech_trace::wr!("1001c72d", p.add(7), write_unaligned, param_6);
            mech_trace::wr!("1001c72d", p.add(0xb), write_unaligned, 0);
            mech_trace::wr!("1001c72d", p.add(0xc), write_unaligned, 0);
            mech_trace::wr!("1001c72d", p.add(0xd), write_unaligned, 0);
            let cnt_ptr = (i_var2 as u32).wrapping_add(4) as *mut i16;
            let cur = mech_trace::rd!("1001c72d", cnt_ptr, read_unaligned);
            mech_trace::wr!(
                "1001c72d",
                cnt_ptr,
                write_unaligned,
                (cur as i32 as u32).wrapping_add(1) as i16
            );
        }
    }
}
