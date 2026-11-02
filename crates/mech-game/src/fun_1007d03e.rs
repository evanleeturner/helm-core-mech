// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6824: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1007cde0() -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007d03e() -> u32 {
    mech_trace::enter!("1007d03e");
    let mut local_c: u32 = 0xffffffff;
    // SAFETY: the global and the callee are the game's own; the returned
    // pointer, when non-null, points at a record with at least 0x10 words.
    unsafe {
        if mech_trace::gr!("1007d03e", "100b6824", DAT_100b6824) != 0 {
            let p_var1 = mech_trace::cr!("1007d03e", "1007cde0", FUN_1007cde0());
            if !p_var1.is_null() {
                local_c = mech_trace::rd!("1007d03e", p_var1.add(3), read_unaligned);
                mech_trace::wr!("1007d03e", p_var1.add(0xf), write_unaligned, 0);
            }
        }
    }
    mech_trace::rt!("1007d03e", local_c)
}
