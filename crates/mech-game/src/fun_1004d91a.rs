// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef030: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d91a(param_1: *const u8) -> u16 {
    mech_trace::enter!("1004d91a", param_1);
    // SAFETY: param_1 points at the caller's bytes; the globals are the
    // game's own tables, indexed as the original does.
    unsafe {
        let local_8 = mech_trace::rd!("1004d91a", param_1, read) as u32;
        let b_var1 = mech_trace::rd!("1004d91a", param_1.add(1), read);
        mech_trace::rt!(
            "1004d91a",
            if b_var1 == 1 {
                0
            } else if b_var1 == 2 {
                let base = mech_trace::rd!(
                    "1004d91a",
                    (&raw mut DAT_100eef40).cast::<u32>().add(local_8 as usize),
                    read
                );
                mech_trace::rd!(
                    "1004d91a",
                    (base.wrapping_add(0x14) as *const u16),
                    read_unaligned
                ) & 0xe
            } else if b_var1 == 4 {
                let p = (&raw mut DAT_100ef030)
                    .cast::<u8>()
                    .add(local_8.wrapping_mul(0x40) as usize);
                mech_trace::rd!("1004d91a", (p as *const u16), read_unaligned) & 0xe
            } else {
                0
            }
        )
    }
}
