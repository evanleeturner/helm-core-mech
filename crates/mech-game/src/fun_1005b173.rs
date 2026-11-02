// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut unaff_ESI: *mut u8;
    static mut unaff_EDI: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005b173() -> u8 {
    mech_trace::enter!("1005b173");
    // SAFETY: the globals stand in for the caller's ESI and EDI registers;
    // they point at the caller's buffer and state block, read and written
    // where the original does.
    unsafe {
        let mut pb_var2: *mut u8 = mech_trace::gr!("1005b173", "unaff_ESI", unaff_ESI);
        let counter = mech_trace::gr!("1005b173", "unaff_EDI", unaff_EDI)
            .byte_add(0x10)
            .cast::<i32>();
        if mech_trace::rd!("1005b173", counter, read_unaligned) == 0 {
            pb_var2 = mech_trace::gr!("1005b173", "unaff_ESI", unaff_ESI).add(1);
            mech_trace::wr!(
                "1005b173",
                counter.cast::<u32>(),
                write_unaligned,
                u32::from(mech_trace::rd!(
                    "1005b173",
                    mech_trace::gr!("1005b173", "unaff_ESI", unaff_ESI),
                    read
                ))
            );
        }
        let b_var1 = mech_trace::rd!("1005b173", pb_var2, read);
        mech_trace::wr!(
            "1005b173",
            counter,
            write_unaligned,
            mech_trace::rd!("1005b173", counter, read_unaligned).wrapping_add(-1)
        );
        mech_trace::rt!("1005b173", b_var1)
    }
}
