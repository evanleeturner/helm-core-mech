// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027c4c(param_1: i32) {
    mech_trace::enter!("10027c4c", param_1);
    // SAFETY: param_1 is the game's object address; its fields at +0x4c, +100
    // and the pointed-to struct at +0xc are read and written as the original does.
    unsafe {
        let inner = mech_trace::rd!(
            "10027c4c",
            (param_1.wrapping_add(0x4c) as *const i32),
            read_unaligned
        );
        let i_var1 = mech_trace::rd!(
            "10027c4c",
            (inner.wrapping_add(0xc) as *const i32),
            read_unaligned
        ) >> 0x10;
        let i_var2 = i_var1.wrapping_mul(-0x1680);
        let p = param_1.wrapping_add(100) as *mut i32;
        let u_var3 = (mech_trace::rd!("10027c4c", p, read_unaligned))
            .wrapping_add(i_var1.wrapping_mul(0x1680)) as u32;
        let u_var4 = ((u_var3 as i32) >> 0x1f) as u32;
        if ((u_var3 ^ u_var4).wrapping_sub(u_var4) as i32) < 0xa0001 {
            mech_trace::wr!("10027c4c", p, write_unaligned, i_var2);
        } else {
            let cur = mech_trace::rd!("10027c4c", p, read_unaligned);
            let cond = i_var2.wrapping_sub(cur) < 0;
            let t = (i32::from(cond).wrapping_sub(1) & 2)
                .wrapping_mul(5)
                .wrapping_add(-5)
                .wrapping_mul(0x10000);
            mech_trace::wr!(
                "10027c4c",
                p,
                write_unaligned,
                mech_trace::rd!("10027c4c", p, read_unaligned).wrapping_add(t)
            );
            let v = mech_trace::rd!("10027c4c", p, read_unaligned) % 0x1680000;
            mech_trace::wr!("10027c4c", p, write_unaligned, v);
        }
    }
}
