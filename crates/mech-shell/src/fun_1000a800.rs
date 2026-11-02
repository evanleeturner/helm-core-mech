// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a690(record: i32) -> u8;
    fn FUN_1000a180(record: i32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a800(record: i32) {
    mech_trace::enter!("1000a800", record);
    // SAFETY: record is an address of the game's record; the callees are the
    // game's own, and fields are read and written where the original does.
    unsafe {
        let p = |off: i32| record.wrapping_add(off) as *mut i32;
        mech_trace::cr!("1000a800", "1000a690", FUN_1000a690(record));
        if mech_trace::rd!("1000a800", p(0x280), read_unaligned)
            < mech_trace::rd!("1000a800", p(0x288), read_unaligned)
        {
            loop {
                mech_trace::cr!(
                    "1000a800",
                    "1000a180",
                    FUN_1000a180(
                        mech_trace::rd!("1000a800", p(0x288), read_unaligned).wrapping_add(0x2134)
                    )
                );
                let lowered_count =
                    mech_trace::rd!("1000a800", p(0x288), read_unaligned).wrapping_sub(1);
                mech_trace::wr!("1000a800", p(0x288), write_unaligned, lowered_count);
                if !(mech_trace::rd!("1000a800", p(0x280), read_unaligned) < lowered_count) {
                    break;
                }
            }
        }
        if mech_trace::rd!("1000a800", p(0x200), read_unaligned) < 0x157d {
            mech_trace::wr!("1000a800", p(0x234), write_unaligned, 0x32);
        } else if mech_trace::rd!("1000a800", p(0x200), read_unaligned) < 0x2135 {
            mech_trace::wr!("1000a800", p(0x234), write_unaligned, 100);
        } else {
            mech_trace::wr!("1000a800", p(0x234), write_unaligned, 200);
        }
        mech_trace::wr!(
            "1000a800",
            p(0x238),
            write_unaligned,
            mech_trace::rd!("1000a800", p(0x288), read_unaligned).wrapping_mul(mech_trace::rd!(
                "1000a800",
                p(0x234),
                read_unaligned
            ))
        );
    }
}
