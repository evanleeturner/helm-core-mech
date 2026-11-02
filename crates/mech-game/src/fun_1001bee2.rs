// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001be70(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001bee2(param_1: *mut u16) {
    mech_trace::enter!("1001bee2", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's object; the callee is the game's own.
    unsafe {
        if (mech_trace::rd!("1001bee2", param_1, read_unaligned) & 0x200) != 0 {
            return;
        }
        let field = param_1.byte_add(0x4c).cast::<i32>();
        if mech_trace::rd!("1001bee2", field, read_unaligned) == 0 {
            let p = mech_trace::cr!(
                "1001bee2",
                "1001be70",
                FUN_1001be70(param_1 as usize as i32)
            );
            mech_trace::wr!("1001bee2", field, write_unaligned, p as usize as i32);
        }
        if mech_trace::rd!("1001bee2", field, read_unaligned) != 0 {
            mech_trace::wr!(
                "1001bee2",
                param_1,
                write_unaligned,
                mech_trace::rd!("1001bee2", param_1, read_unaligned) | 0x200
            );
        }
    }
}
