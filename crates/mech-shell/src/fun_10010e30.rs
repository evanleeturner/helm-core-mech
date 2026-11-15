// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10010930() -> u32;
    #[link_name = "\u{1}_HC6_AIL_set_sample_volume@8"]
    fn AIL_set_sample_volume(handle: u32, volume: u32);
}

#[unsafe(export_name = "FUN_10010e30")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn SetSampleVolume(sample: *mut u8, volume: i32) {
    mech_trace::enter!("10010e30", sample, volume);
    // SAFETY: `sample` points at the caller's sample object; the fields are read
    // and written at the original's offsets, and the callees are the game's own.
    unsafe {
        let sample_handle = mech_trace::rd!(
            "10010e30",
            sample.byte_add(0x29).cast::<i32>(),
            read_unaligned
        );
        if sample_handle != 0 {
            if volume < 0x80 {
                if volume < 0 {
                    mech_trace::wr!(
                        "10010e30",
                        sample.byte_add(0xc).cast::<i32>(),
                        write_unaligned,
                        0
                    );
                } else {
                    mech_trace::wr!(
                        "10010e30",
                        sample.byte_add(0xc).cast::<i32>(),
                        write_unaligned,
                        volume
                    );
                }
            } else {
                mech_trace::wr!(
                    "10010e30",
                    sample.byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    0x7f
                );
            }
            let scale = mech_trace::cr!("10010e30", "10010930", FUN_10010930());
            let stored_volume = mech_trace::rd!(
                "10010e30",
                sample.byte_add(0xc).cast::<i32>(),
                read_unaligned
            ) as u32;
            let scaled_volume = scale.wrapping_mul(stored_volume) as i32;
            mech_trace::cr!(
                "10010e30",
                "AIL_set_sample_volume",
                AIL_set_sample_volume(sample_handle as u32, (scaled_volume >> 0x10) as u32)
            );
        }
    }
}
