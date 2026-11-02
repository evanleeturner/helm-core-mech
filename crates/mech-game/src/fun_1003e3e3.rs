// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003e3e3(param_1: i32, param_2: i32) {
    mech_trace::enter!("1003e3e3", param_1, param_2);
    // SAFETY: both parameters are addresses of the game's list nodes; fields
    // are read and written at the same offsets and in the same order as the
    // original.
    unsafe {
        let p1 = param_1 as *mut u8;
        let p2 = param_2 as *mut u8;
        let next = mech_trace::rd!("1003e3e3", p2.byte_add(8).cast::<i32>(), read_unaligned);
        if next != 0 {
            mech_trace::wr!(
                "1003e3e3",
                (next as *mut u8).byte_add(4).cast::<i32>(),
                write_unaligned,
                param_1
            );
        }
        let next2 = mech_trace::rd!("1003e3e3", p2.byte_add(8).cast::<u32>(), read_unaligned);
        mech_trace::wr!(
            "1003e3e3",
            p1.byte_add(8).cast::<u32>(),
            write_unaligned,
            next2
        );
        mech_trace::wr!(
            "1003e3e3",
            p2.byte_add(8).cast::<i32>(),
            write_unaligned,
            param_1
        );
        mech_trace::wr!(
            "1003e3e3",
            p1.byte_add(4).cast::<i32>(),
            write_unaligned,
            param_2
        );
    }
}
