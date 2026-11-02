// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut unaff_EDI: i32;
    static mut unaff_ESI: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069d13() -> u8 {
    mech_trace::enter!("10069d13");
    // SAFETY: the globals are the game's own; unaff_ESI points at the input
    // bytes and unaff_EDI at a structure whose field at +0x10 is read and
    // written where the original does.
    unsafe {
        let mut pbvar2: *mut u8 = mech_trace::gr!("10069d13", "unaff_ESI", unaff_ESI);
        let counter =
            (mech_trace::gr!("10069d13", "unaff_EDI", unaff_EDI).wrapping_add(0x10)) as *mut i32;
        if mech_trace::rd!("10069d13", counter, read_unaligned) == 0 {
            pbvar2 = mech_trace::gr!("10069d13", "unaff_ESI", unaff_ESI).wrapping_add(1);
            mech_trace::wr!(
                "10069d13",
                counter,
                write_unaligned,
                u32::from(mech_trace::rd!(
                    "10069d13",
                    mech_trace::gr!("10069d13", "unaff_ESI", unaff_ESI),
                    read
                )) as i32
            );
        }
        let bvar1 = mech_trace::rd!("10069d13", pbvar2, read);
        mech_trace::wr!(
            "10069d13",
            counter,
            write_unaligned,
            mech_trace::rd!("10069d13", counter, read_unaligned).wrapping_sub(1)
        );
        mech_trace::rt!("10069d13", bvar1)
    }
}
