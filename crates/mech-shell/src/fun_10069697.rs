// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069697(font_Likely: i32, charCode: i32) -> i32 {
    mech_trace::enter!("10069697", font_Likely, charCode);
    // SAFETY: font_Likely is an address of the game's font structure; the
    // offset table entry and the value it points to are read where the
    // original reads them.
    unsafe {
        let inner = charCode
            .wrapping_mul(4)
            .wrapping_add(font_Likely)
            .wrapping_add(0x10);
        let off = mech_trace::rd!("10069697", (inner as usize as *const i32), read_unaligned);
        mech_trace::rt!(
            "10069697",
            mech_trace::rd!(
                "10069697",
                (off.wrapping_add(font_Likely) as usize as *const i32),
                read_unaligned
            )
        )
    }
}
