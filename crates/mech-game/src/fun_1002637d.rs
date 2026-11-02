// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(segment: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(segment: u32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002637d(segment: *mut i32) {
    mech_trace::enter!("1002637d", segment);
    // SAFETY: segment points at the caller's segment record of at least 14
    // ints; the callees are the game's own.
    unsafe {
        if mech_trace::rd!("1002637d", segment.add(0xd), read_unaligned) == 0 {
            let length = mech_trace::cr!(
                "1002637d",
                "1000fc30",
                FUN_1000fc30(
                    mech_trace::rd!("1002637d", segment.add(6), read_unaligned),
                    mech_trace::rd!("1002637d", segment.add(7), read_unaligned),
                    mech_trace::rd!("1002637d", segment.add(8), read_unaligned),
                )
            );
            mech_trace::wr!("1002637d", segment.add(0xc), write_unaligned, length);
            let length = mech_trace::rd!("1002637d", segment.add(0xc), read_unaligned);
            if 0 < length {
                let v = mech_trace::cr!(
                    "1002637d",
                    "10002570",
                    FUN_10002570(
                        mech_trace::rd!("1002637d", segment.add(6), read_unaligned) as u32,
                        length
                    )
                );
                mech_trace::wr!("1002637d", segment.add(9), write_unaligned, v);
                let v = mech_trace::cr!(
                    "1002637d",
                    "10002570",
                    FUN_10002570(
                        mech_trace::rd!("1002637d", segment.add(7), read_unaligned) as u32,
                        length
                    )
                );
                mech_trace::wr!("1002637d", segment.add(10), write_unaligned, v);
                let v = mech_trace::cr!(
                    "1002637d",
                    "10002570",
                    FUN_10002570(
                        mech_trace::rd!("1002637d", segment.add(8), read_unaligned) as u32,
                        length
                    )
                );
                mech_trace::wr!("1002637d", segment.add(0xb), write_unaligned, v);
                mech_trace::wr!("1002637d", segment.add(0xd), write_unaligned, 1);
            }
        }
    }
}
