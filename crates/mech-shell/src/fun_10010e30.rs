// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10010930() -> u32;
    fn _AIL_set_sample_volume_8(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10010e30(this: *mut u8, volume: i32) {
    mech_trace::enter!("10010e30", this, volume);
    // SAFETY: `this` points at the caller's sample object; the fields are read
    // and written at the original's offsets, and the callees are the game's own.
    unsafe {
        let sample_handle = mech_trace::rd!(
            "10010e30",
            this.byte_add(0x29).cast::<i32>(),
            read_unaligned
        );
        if sample_handle != 0 {
            if volume < 0x80 {
                if volume < 0 {
                    mech_trace::wr!(
                        "10010e30",
                        this.byte_add(0xc).cast::<i32>(),
                        write_unaligned,
                        0
                    );
                } else {
                    mech_trace::wr!(
                        "10010e30",
                        this.byte_add(0xc).cast::<i32>(),
                        write_unaligned,
                        volume
                    );
                }
            } else {
                mech_trace::wr!(
                    "10010e30",
                    this.byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    0x7f
                );
            }
            let scale = mech_trace::cr!("10010e30", "10010930", FUN_10010930());
            let cur = mech_trace::rd!("10010e30", this.byte_add(0xc).cast::<i32>(), read_unaligned)
                as u32;
            let product = scale.wrapping_mul(cur) as i32;
            mech_trace::cr!(
                "10010e30",
                "_AIL_set_sample_volume_8",
                _AIL_set_sample_volume_8(sample_handle as u32, (product >> 0x10) as u32)
            );
        }
    }
}
