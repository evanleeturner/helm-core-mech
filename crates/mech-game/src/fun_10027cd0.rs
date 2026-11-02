// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027cd0(param_1: i32, param_2: *mut i32) {
    mech_trace::enter!("10027cd0", param_1, param_2);
    // SAFETY: param_1 is the game's object address and param_2 points at the
    // caller's int; fields are read and written where the original does.
    unsafe {
        let p = mech_trace::rd!("10027cd0", param_2, read_unaligned);
        let ptr4c = mech_trace::rd!(
            "10027cd0",
            ((param_1 as u32).wrapping_add(0x4c) as *const i32),
            read_unaligned
        );
        let m = mech_trace::rd!(
            "10027cd0",
            ((ptr4c as u32).wrapping_add(8) as *const i32),
            read_unaligned
        );
        let t = (p >> 1).wrapping_add((p >> 0x1f) & 0x3ff) >> 10;
        let mut local_8 = p.wrapping_sub(m.wrapping_mul(t));
        let p54 = mech_trace::rd!(
            "10027cd0",
            ((param_1 as u32).wrapping_add(0x54) as *const i32),
            read_unaligned
        );
        let u1 = p54.wrapping_sub(local_8) as u32;
        let s = (u1 as i32) >> 0x1f;
        let abs = ((u1 as i32) ^ s).wrapping_sub(s);
        let sign: i32 = if u1 == 0 {
            0
        } else if (u1 as i32) < 0 {
            -1
        } else {
            1
        };
        if 300 < abs {
            local_8 = p54.wrapping_add(sign.wrapping_mul(-300));
        }
        mech_trace::wr!("10027cd0", param_2, write_unaligned, local_8);
        let p5c_ptr = (param_1 as u32).wrapping_add(0x5c) as *mut i32;
        let p5c = mech_trace::rd!("10027cd0", p5c_ptr, read_unaligned);
        let target = sign.wrapping_mul(0xa0000);
        if p5c != target {
            let diff = mech_trace::rd!("10027cd0", p5c_ptr, read_unaligned)
                .wrapping_add(sign.wrapping_mul(-0xa0000));
            let local_1c: i32 = if diff < 0 { -1 } else { 1 };
            let cur = mech_trace::rd!("10027cd0", p5c_ptr, read_unaligned);
            mech_trace::wr!(
                "10027cd0",
                p5c_ptr,
                write_unaligned,
                cur.wrapping_add(local_1c.wrapping_mul(-0x10000))
            );
        }
    }
}
