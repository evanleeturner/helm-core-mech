// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_stop_sample_4(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10010de0(sample_object: *mut i32) {
    mech_trace::enter!("10010de0", sample_object);
    // SAFETY: sample_object points at the game's sample object, whose fields
    // are read and written at the original's offsets; the callee is the game's own.
    unsafe {
        let handle = mech_trace::rd!(
            "10010de0",
            sample_object.byte_add(0x29).cast::<i32>(),
            read_unaligned
        );
        if handle != 0 {
            let vtbl =
                mech_trace::rd!("10010de0", sample_object, read_unaligned) as usize as *const u8;
            let v = mech_trace::rd!("10010de0", vtbl.byte_add(5).cast::<i32>(), read_unaligned);
            if v != 0 {
                mech_trace::wr!("10010de0", sample_object.add(10).cast::<u8>(), write, 0);
                let h = mech_trace::rd!(
                    "10010de0",
                    sample_object.byte_add(0x29).cast::<i32>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "10010de0",
                    "_AIL_stop_sample_4",
                    _AIL_stop_sample_4(h as u32)
                );
            }
        }
    }
}
