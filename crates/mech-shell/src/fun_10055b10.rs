// SPDX-License-Identifier: MIT

#[unsafe(export_name = "FUN_10055b10")]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn IsLeftButtonJustPressed(mouse_state: i32) -> u32 {
    mech_trace::enter!("10055b10", mouse_state);
    // SAFETY: mouse_state is the address of a caller-owned object with a
    // readable u32 at offset 0x10.
    unsafe {
        mech_trace::rt!(
            "10055b10",
            mech_trace::rd!(
                "10055b10",
                ((mouse_state as usize).wrapping_add(0x10) as *const u32),
                read_unaligned
            )
        )
    }
}
