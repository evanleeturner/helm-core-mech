// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(vec_x: u32, vec_y: u32, vec_z: u32) -> i32;
    fn FUN_10002570(vec_x: u32, vec_y: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fef4(vec_x: *mut u32, vec_y: *mut u32, vec_z: *mut u32) {
    mech_trace::enter!("1000fef4", vec_x, vec_y, vec_z);
    // SAFETY: the callees are the game's own; the pointers are the caller's
    // and are read and written where the original does.
    unsafe {
        let vec_length = mech_trace::cr!(
            "1000fef4",
            "1000fc30",
            FUN_1000fc30(
                mech_trace::rd!("1000fef4", vec_x, read_unaligned),
                mech_trace::rd!("1000fef4", vec_y, read_unaligned),
                mech_trace::rd!("1000fef4", vec_z, read_unaligned),
            )
        );
        if 0 < vec_length {
            let c = mech_trace::cr!(
                "1000fef4",
                "10002570",
                FUN_10002570(
                    mech_trace::rd!("1000fef4", vec_x, read_unaligned),
                    vec_length
                )
            );
            mech_trace::wr!("1000fef4", vec_x, write_unaligned, c);
            let c = mech_trace::cr!(
                "1000fef4",
                "10002570",
                FUN_10002570(
                    mech_trace::rd!("1000fef4", vec_y, read_unaligned),
                    vec_length
                )
            );
            mech_trace::wr!("1000fef4", vec_y, write_unaligned, c);
            let c = mech_trace::cr!(
                "1000fef4",
                "10002570",
                FUN_10002570(
                    mech_trace::rd!("1000fef4", vec_z, read_unaligned),
                    vec_length
                )
            );
            mech_trace::wr!("1000fef4", vec_z, write_unaligned, c);
        }
    }
}
