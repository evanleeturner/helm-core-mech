// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10010e80(this: *mut u8, sample: u32);
    fn FUN_10010e30(this: *mut u8, sample: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_start_sample_4(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10010d80(sample: *mut u8) {
    mech_trace::enter!("10010d80", sample);
    // SAFETY: sample points at the game's sample object; fields are read and
    // written at the original's offsets, and the callees are the game's own.
    unsafe {
        if mech_trace::rd!(
            "10010d80",
            sample.byte_add(0x29).cast::<i32>(),
            read_unaligned
        ) != 0
        {
            FUN_10010e80(
                sample,
                mech_trace::rd!(
                    "10010d80",
                    sample.byte_add(0x10).cast::<u32>(),
                    read_unaligned
                ),
            );
            FUN_10010e30(
                sample,
                mech_trace::rd!(
                    "10010d80",
                    sample.byte_add(0xc).cast::<i32>(),
                    read_unaligned
                ),
            );
            mech_trace::cr!(
                "10010d80",
                "_AIL_start_sample_4",
                _AIL_start_sample_4(mech_trace::rd!(
                    "10010d80",
                    sample.byte_add(0x29).cast::<u32>(),
                    read_unaligned
                ))
            );
            mech_trace::wr!("10010d80", sample.byte_add(0x28), write, 1);
        }
    }
}
