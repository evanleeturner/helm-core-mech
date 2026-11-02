// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fcce(param_1: *mut i32) {
    mech_trace::enter!("1002fcce", param_1);
    // SAFETY: param_1 points at the game's own node structure, and the linked
    // list nodes reachable from it are read and written where the original does.
    unsafe {
        let i_var1 = mech_trace::rd!("1002fcce", param_1, read_unaligned);
        if i_var1 != 0 {
            let head = (i_var1 as u32).wrapping_add(4) as *mut i32;
            if mech_trace::rd!("1002fcce", head, read_unaligned) == param_1 as i32 {
                mech_trace::wr!(
                    "1002fcce",
                    head,
                    write_unaligned,
                    mech_trace::rd!("1002fcce", param_1.byte_add(8), read_unaligned)
                );
            } else {
                let mut local_c = mech_trace::rd!("1002fcce", head, read_unaligned);
                while local_c != 0 {
                    let next = (local_c as u32).wrapping_add(8) as *mut i32;
                    if mech_trace::rd!("1002fcce", next, read_unaligned) == param_1 as i32 {
                        mech_trace::wr!(
                            "1002fcce",
                            next,
                            write_unaligned,
                            mech_trace::rd!("1002fcce", param_1.byte_add(8), read_unaligned)
                        );
                        break;
                    }
                    local_c = mech_trace::rd!("1002fcce", next, read_unaligned);
                }
            }
        }
        mech_trace::wr!("1002fcce", param_1, write_unaligned, 0);
        mech_trace::wr!("1002fcce", param_1.byte_add(8), write_unaligned, 0);
        let mut src = param_1.wrapping_add(0xf);
        let mut dst = param_1.wrapping_add(3);
        let mut count: i32 = 0xc;
        while count != 0 {
            mech_trace::wr!(
                "1002fcce",
                dst,
                write_unaligned,
                mech_trace::rd!("1002fcce", src, read_unaligned)
            );
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            count = count.wrapping_add(-1);
        }
        let p = param_1.wrapping_add(0x1d);
        mech_trace::wr!(
            "1002fcce",
            p,
            write_unaligned,
            mech_trace::rd!("1002fcce", p, read_unaligned) | 1
        );
        mech_trace::cr!("1002fcce", "1002fbc0", FUN_1002fbc0(param_1));
    }
}
