// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa4e8: [*mut u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100145a1(param_1: i32) -> bool {
    mech_trace::enter!("100145a1", param_1);
    // SAFETY: param_1 is the address of the caller's record; the table is the
    // game's own memory, indexed exactly as the original does.
    unsafe {
        let v = mech_trace::rd!(
            "100145a1",
            ((param_1.wrapping_add(0x24)) as *const u32),
            read_unaligned
        );
        let idx = v.wrapping_mul(3) as i32;
        let p = mech_trace::rd!(
            "100145a1",
            (&raw mut DAT_100aa4e8)
                .cast::<*mut u8>()
                .offset(idx as isize),
            read_unaligned
        );
        mech_trace::rt!("100145a1", !p.is_null())
    }
}
