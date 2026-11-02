// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063d30(entry: *mut i32) {
    mech_trace::enter!("10063d30", entry);
    let mut entry = entry;
    let mut prev_field1: i32 = 0;
    let mut prev_field3: i32 = 0;
    if entry.is_null() {
        return;
    }
    // SAFETY: entry points at the game's table of 14-int records ended by a
    // -1 first field; the callbacks and the result pointers are the game's own.
    unsafe {
        let mut first_field = mech_trace::rd!("10063d30", entry, read_unaligned);
        while first_field != -1 {
            let field1_raw = mech_trace::rd!("10063d30", entry.add(1), read_unaligned) as u32;
            if (field1_raw as i32) < 0 {
                let v = (((field1_raw & 0xff0) >> 4) as i32)
                    .wrapping_mul(prev_field3)
                    .wrapping_add((field1_raw & 0xf) as i32)
                    .wrapping_add(prev_field1);
                mech_trace::wr!("10063d30", entry.add(1), write_unaligned, v);
            }
            let result_slot = entry.add(6);
            let cb = mech_trace::rd!("10063d30", entry.add(7), read_unaligned) as u32 as usize;
            if cb == 0 {
                mech_trace::wr!("10063d30", result_slot, write_unaligned, 0);
            } else {
                let f: extern "C" fn(*mut i32) -> i32 = core::mem::transmute(cb);
                prev_field1 = mech_trace::cr!("10063d30", "ind", f(entry));
                mech_trace::wr!("10063d30", result_slot, write_unaligned, prev_field1);
            }
            let res = mech_trace::rd!("10063d30", result_slot, read_unaligned);
            if res != 0 {
                if mech_trace::rd!("10063d30", entry.add(2), read_unaligned) == -1 {
                    let p = (res as u32).wrapping_add(0x21) as usize as *const i32;
                    mech_trace::wr!(
                        "10063d30",
                        entry.add(2),
                        write_unaligned,
                        mech_trace::rd!("10063d30", p, read_unaligned)
                    );
                }
                if mech_trace::rd!("10063d30", entry.add(3), read_unaligned) == -1 {
                    let p = (res as u32).wrapping_add(0x1d) as usize as *const i32;
                    mech_trace::wr!(
                        "10063d30",
                        entry.add(3),
                        write_unaligned,
                        mech_trace::rd!("10063d30", p, read_unaligned)
                    );
                }
            }
            if mech_trace::rd!("10063d30", entry.add(2), read_unaligned) == -1 {
                mech_trace::wr!("10063d30", entry.add(2), write_unaligned, 0);
            }
            if mech_trace::rd!("10063d30", entry.add(3), read_unaligned) == -1 {
                mech_trace::wr!("10063d30", entry.add(3), write_unaligned, prev_field3);
            }
            prev_field1 = mech_trace::rd!("10063d30", entry.add(1), read_unaligned);
            prev_field3 = mech_trace::rd!("10063d30", entry.add(3), read_unaligned);
            entry = entry.add(0xe);
            first_field = mech_trace::rd!("10063d30", entry, read_unaligned);
        }
    }
}
