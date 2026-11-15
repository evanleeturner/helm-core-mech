// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    #[link_name = "DAT_1008eb64"]
    static mut TEXT_DRAW_SUPPRESSED: u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1001e6b0(param_1: *mut i32) -> u8;
}

#[unsafe(export_name = "FUN_10013380")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn WalkSurfaceListTextSuppressed(
    surface: *mut u8,
    use_first_collection: i32,
) {
    mech_trace::enter!("10013380", surface, use_first_collection);
    // SAFETY: the global and callee are the game's own; surface points at the
    // caller's object, read at the offsets the original uses.
    unsafe {
        mech_trace::gw!("10013380", "1008eb64", TEXT_DRAW_SUPPRESSED, 1);
        let collection_holder: *mut i32 = if use_first_collection == 0 {
            mech_trace::rd!(
                "10013380",
                surface.byte_add(0x1a).cast::<*mut i32>(),
                read_unaligned
            )
        } else {
            mech_trace::rd!(
                "10013380",
                surface.byte_add(0x16).cast::<*mut i32>(),
                read_unaligned
            )
        };
        mech_trace::cr!("10013380", "1001e6b0", FUN_1001e6b0(collection_holder));
        mech_trace::gw!("10013380", "1008eb64", TEXT_DRAW_SUPPRESSED, 0);
    }
}
