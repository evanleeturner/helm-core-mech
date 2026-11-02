// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069990(param_1: *mut i16, param_2: i32) -> *mut i16 {
    mech_trace::enter!("10069990", param_1, param_2);
    // SAFETY: param_1 and the table at param_2 are the game's own memory;
    // reads happen where the original reads, and the scan has no bound of its own.
    unsafe {
        let mut p = (param_2.wrapping_add(0xc)) as *const i8;
        loop {
            while mech_trace::dr!("10069990", p) == 0 {
                p = p.byte_add(1);
            }
            let mut count: i32 = 2;
            let mut matched;
            let mut p_src = p;
            let mut p_dst = param_1 as *const i8;
            loop {
                if count == 0 {
                    // unreachable with match false; mirrors the original break
                    matched = true;
                    break;
                }
                count -= 1;
                matched = mech_trace::rd!("10069990", p_src.cast::<i16>(), read_unaligned)
                    == mech_trace::rd!("10069990", p_dst.cast::<i16>(), read_unaligned);
                p_src = p_src.byte_add(2);
                p_dst = p_dst.byte_add(2);
                if !matched {
                    break;
                }
            }
            if matched {
                return mech_trace::rt!("10069990", p.byte_add(8) as *mut i16);
            }
            p = p.byte_add(6);
            let offset = mech_trace::rd!("10069990", p.cast::<u16>(), read_unaligned);
            p = p.byte_add(8);
            p = p.byte_add(offset as usize);
        }
    }
}
