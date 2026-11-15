// SPDX-License-Identifier: MIT

#[unsafe(export_name = "FUN_10069697")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn GetGlyphWidth(font: i32, charCode: i32) -> i32 {
    mech_trace::enter!("10069697", font, charCode);
    // SAFETY: font is an address of the game's font structure; the
    // offset table entry and the value it points to are read where the
    // original reads them.
    unsafe {
        let glyph_offset_slot = charCode
            .wrapping_mul(4)
            .wrapping_add(font)
            .wrapping_add(0x10);
        let glyph_offset = mech_trace::rd!(
            "10069697",
            (glyph_offset_slot as usize as *const i32),
            read_unaligned
        );
        mech_trace::rt!(
            "10069697",
            mech_trace::rd!(
                "10069697",
                (glyph_offset.wrapping_add(font) as usize as *const i32),
                read_unaligned
            )
        )
    }
}
