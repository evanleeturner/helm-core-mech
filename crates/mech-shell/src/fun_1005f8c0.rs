// SPDX-License-Identifier: MIT

#[unsafe(export_name = "FUN_1005f8c0")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn TestItemFlags(item: *mut u8, mask: u32) -> bool {
    mech_trace::enter!("1005f8c0", item, mask);
    // SAFETY: `item` points at the caller's object, which holds a u32 of flags at offset 0x10c.
    unsafe {
        mech_trace::rt!(
            "1005f8c0",
            (mask
                & mech_trace::rd!(
                    "1005f8c0",
                    item.byte_add(0x10c).cast::<u32>(),
                    read_unaligned
                ))
                != 0
        )
    }
}
