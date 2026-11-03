// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005f8c0(this: *mut u8, param_2: u32) -> bool {
    mech_trace::enter!("1005f8c0", this, param_2);
    // SAFETY: `this` points at the caller's object, which holds a u32 of flags at offset 0x10c.
    unsafe {
        mech_trace::rt!(
            "1005f8c0",
            (param_2
                & mech_trace::rd!(
                    "1005f8c0",
                    this.byte_add(0x10c).cast::<u32>(),
                    read_unaligned
                ))
                != 0
        )
    }
}
