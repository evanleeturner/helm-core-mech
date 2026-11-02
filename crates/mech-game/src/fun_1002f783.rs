// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e051(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f783(param_1: i32) {
    mech_trace::enter!("1002f783", param_1);
    // SAFETY: param_1 points at the game's own node structure; the offsets
    // read are the ones the original reads, and the callee is the game's.
    unsafe {
        let p = mech_trace::rd!(
            "1002f783",
            (param_1 as usize as *const u8).byte_add(0x6c).cast::<i32>(),
            read_unaligned
        );
        if p != 0 {
            let w = (mech_trace::rd!(
                "1002f783",
                (p as usize as *const u8).byte_add(2).cast::<u16>(),
                read_unaligned
            )) as u8;
            if (w & 0xf0) != 0x70 {
                mech_trace::cr!("1002f783", "1003e051", FUN_1003e051(p as usize as *mut u16));
            }
        }
        let mut local_8 = mech_trace::rd!(
            "1002f783",
            (param_1 as usize as *const u8).byte_add(4).cast::<i32>(),
            read_unaligned
        );
        while local_8 != 0 {
            FUN_1002f783(local_8);
            local_8 = mech_trace::rd!(
                "1002f783",
                (local_8 as usize as *const u8).byte_add(8).cast::<i32>(),
                read_unaligned
            );
        }
    }
}
