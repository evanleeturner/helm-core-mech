// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015cd4(param_1: i32) {
    mech_trace::enter!("10015cd4", param_1);
    let base = param_1 as usize;
    let p = |off: usize| (base.wrapping_add(off)) as *mut i32;
    // SAFETY: param_1 is the address of the caller's game object; every
    // offset read or written here lies inside it, as in the original.
    unsafe {
        mech_trace::wr!("10015cd4", p(0x34), write_unaligned, 0x1e7a);
        mech_trace::wr!("10015cd4", p(0x38), write_unaligned, 0x8dc9);
        FUN_100660e1(
            mech_trace::rd!("10015cd4", p(0xa4), read_unaligned),
            p(0x34),
            p(0x34),
        );
        let w = mech_trace::rd!(
            "10015cd4",
            ((base.wrapping_add(0xb8)) as *const i16),
            read_unaligned
        ) as i32;
        let h = mech_trace::rd!(
            "10015cd4",
            ((base.wrapping_add(0xba)) as *const i16),
            read_unaligned
        ) as i32;
        mech_trace::wr!(
            "10015cd4",
            p(0x3c),
            write_unaligned,
            w.wrapping_sub(mech_trace::rd!("10015cd4", p(0x34), read_unaligned))
                .wrapping_add(-1)
        );
        let v38 = mech_trace::rd!("10015cd4", p(0x38), read_unaligned);
        mech_trace::wr!(
            "10015cd4",
            p(0x40),
            write_unaligned,
            h.wrapping_sub(
                v38.wrapping_add(mech_trace::rd!("10015cd4", p(0x38), read_unaligned) / 2)
            )
            .wrapping_add(-3)
        );
        mech_trace::wr!("10015cd4", p(0x44), write_unaligned, w.wrapping_add(-1));
        mech_trace::wr!("10015cd4", p(0x48), write_unaligned, h.wrapping_add(-1));
        mech_trace::wr!(
            "10015cd4",
            p(0x4c),
            write_unaligned,
            mech_trace::rd!("10015cd4", p(0x3c), read_unaligned).wrapping_add(1)
        );
        mech_trace::wr!(
            "10015cd4",
            p(0x50),
            write_unaligned,
            h.wrapping_sub(mech_trace::rd!("10015cd4", p(0x38), read_unaligned) / 2)
                .wrapping_add(-2)
        );
    }
}
