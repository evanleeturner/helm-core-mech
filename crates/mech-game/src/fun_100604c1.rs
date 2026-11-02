// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_100604c1(param_1: i32, param_2: *mut i32, param_3: *mut i32) {
    mech_trace::enter!("100604c1", param_1, param_2, param_3);
    // SAFETY: param_1 is the address of a game object whose fields are read at the
    // original's offsets; param_2 and param_3 point at writable ints.
    unsafe {
        let p = param_1 as *const u8;
        let i_var1 = mech_trace::rd!("100604c1", p.byte_add(0x38).cast::<i32>(), read_unaligned);
        let i_var2 = mech_trace::rd!("100604c1", p.byte_add(0x34).cast::<i32>(), read_unaligned);
        let i_var3 = mech_trace::rd!("100604c1", p.byte_add(0x50).cast::<i32>(), read_unaligned);
        let a = mech_trace::rd!("100604c1", p.byte_add(0x2c).cast::<i32>(), read_unaligned);
        let b = mech_trace::rd!("100604c1", p.byte_add(0x30).cast::<i32>(), read_unaligned);
        let c = mech_trace::rd!("100604c1", p.byte_add(0x4c).cast::<i32>(), read_unaligned);
        let s1 = a.wrapping_add(b) / 2;
        mech_trace::wr!("100604c1", param_2, write_unaligned, c.wrapping_add(s1));
        let s2 = i_var1.wrapping_add(i_var2) / 2;
        mech_trace::wr!(
            "100604c1",
            param_3,
            write_unaligned,
            i_var3.wrapping_add(s2)
        );
    }
}
