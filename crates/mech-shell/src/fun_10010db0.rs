// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10010d80(sample_object: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_sample_status_4(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10010db0(param_1: *mut core::ffi::c_void) {
    mech_trace::enter!("10010db0", param_1);
    // SAFETY: param_1 points at the caller's sample object; the callees are
    // the game's own functions.
    unsafe {
        if mech_trace::rd!(
            "10010db0",
            param_1.byte_add(0x29).cast::<i32>(),
            read_unaligned
        ) != 0
        {
            FUN_10010d80(param_1);
            loop {
                let status = mech_trace::cr!(
                    "10010db0",
                    "_AIL_sample_status_4",
                    _AIL_sample_status_4(mech_trace::rd!(
                        "10010db0",
                        param_1.byte_add(0x29).cast::<u32>(),
                        read_unaligned
                    ))
                );
                if status as i32 != 4 {
                    break;
                }
            }
            mech_trace::wr!("10010db0", param_1.byte_add(0x28).cast::<u8>(), write, 0);
        }
    }
}
