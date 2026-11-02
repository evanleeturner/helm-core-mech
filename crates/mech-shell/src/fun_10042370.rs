// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100984cc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042370(param_1: i32, param_2: *mut i32) -> *mut i16 {
    mech_trace::enter!("10042370", param_1, param_2);
    if param_1 < 0 {
        return mech_trace::rt!("10042370", core::ptr::null_mut());
    }
    // SAFETY: the table pointer is the game's own global; param_2 points at the
    // caller's key bytes and chain nodes are read at the original's offsets.
    unsafe {
        let b = param_2 as *const i8;
        let b2 = mech_trace::rd!("10042370", b.byte_add(2), read) as i32;
        let b3 = mech_trace::rd!("10042370", b.byte_add(3), read) as i32;
        let b1 = mech_trace::rd!("10042370", b.byte_add(1), read) as i32;
        let b0 = (mech_trace::rd!("10042370", param_2, read_unaligned) as i8) as i32;
        let sum = b2
            .wrapping_add(b3)
            .wrapping_add(b1)
            .wrapping_add(b0)
            .wrapping_add(param_1);
        let idx = sum % 0x3f1;
        let addr = mech_trace::gr!("10042370", "100984cc", DAT_100984cc)
            .wrapping_add((idx as u32).wrapping_mul(4));
        let mut node: *mut i16 =
            mech_trace::rd!("10042370", (addr as *const u32), read_unaligned) as usize as *mut i16;
        while !node.is_null() {
            if mech_trace::rd!("10042370", node, read_unaligned) as i32 == param_1
                && mech_trace::rd!("10042370", node.byte_add(4).cast::<i32>(), read_unaligned)
                    == mech_trace::rd!("10042370", param_2, read_unaligned)
            {
                break;
            }
            node = mech_trace::rd!("10042370", node.byte_add(8).cast::<u32>(), read_unaligned)
                as usize as *mut i16;
        }
        mech_trace::rt!("10042370", node)
    }
}
