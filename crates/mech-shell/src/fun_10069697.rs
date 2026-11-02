// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069697(font_Likely: i32, charCode: i32) -> i32 {
    // SAFETY: font_Likely is an address of the game's font structure; the
    // offset table entry and the value it points to are read where the
    // original reads them.
    unsafe {
        let inner = charCode
            .wrapping_mul(4)
            .wrapping_add(font_Likely)
            .wrapping_add(0x10);
        let off = (inner as usize as *const i32).read_unaligned();
        (off.wrapping_add(font_Likely) as usize as *const i32).read_unaligned()
    }
}
