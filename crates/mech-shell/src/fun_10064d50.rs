// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10064d50(
    this: *mut u8,
    field0_value: u32,
    field4_value: u32,
    field8_value: u32,
    field_c_value: u32,
) -> *mut u8 {
    mech_trace::enter!(
        "10064d50",
        this,
        field0_value,
        field4_value,
        field8_value,
        field_c_value
    );
    // SAFETY: `this` points at a caller-owned object of at least 16 bytes;
    // the four fields are written in the original's order.
    unsafe {
        mech_trace::wr!(
            "10064d50",
            this.cast::<u32>(),
            write_unaligned,
            field0_value
        );
        mech_trace::wr!(
            "10064d50",
            this.byte_add(4).cast::<u32>(),
            write_unaligned,
            field4_value
        );
        mech_trace::wr!(
            "10064d50",
            this.byte_add(8).cast::<u32>(),
            write_unaligned,
            field8_value
        );
        mech_trace::wr!(
            "10064d50",
            this.byte_add(0xc).cast::<u32>(),
            write_unaligned,
            field_c_value
        );
    }
    mech_trace::rt!("10064d50", this)
}
