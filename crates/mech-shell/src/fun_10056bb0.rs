// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056bb0(rect: *mut i32, param_2: i16, param_3: i16) -> i32 {
    mech_trace::enter!("10056bb0", rect, param_2, param_3);
    // SAFETY: rect points at the caller's array of at least four i32 values.
    unsafe {
        let r0 = mech_trace::rd!("10056bb0", rect, read_unaligned);
        let r1 = mech_trace::rd!("10056bb0", rect.add(1), read_unaligned);
        let r2 = mech_trace::rd!("10056bb0", rect.add(2), read_unaligned);
        let r3 = mech_trace::rd!("10056bb0", rect.add(3), read_unaligned);
        let x = param_2 as i32;
        let y = param_3 as i32;
        if r1 < x && x < r3 && r2 < y && y < r0 {
            return mech_trace::rt!("10056bb0", 1);
        }
    }
    mech_trace::rt!("10056bb0", 0)
}
