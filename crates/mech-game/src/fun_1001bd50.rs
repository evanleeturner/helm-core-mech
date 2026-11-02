// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        param_1: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001bd50() -> *mut u32 {
    mech_trace::enter!("1001bd50");
    // SAFETY: the heap handle and HeapAlloc are the game's own; the returned
    // block is 0x18 bytes, so indices 0..=5 of u32 are in bounds.
    unsafe {
        let p_var1 = mech_trace::cr!(
            "1001bd50",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1001bd50", "100adf74", DAT_100adf74),
                1,
                0x18
            )
        ) as *mut u32;
        if !p_var1.is_null() {
            mech_trace::wr!("1001bd50", p_var1.add(4), write_unaligned, 0);
            mech_trace::wr!(
                "1001bd50",
                p_var1.add(2),
                write_unaligned,
                mech_trace::rd!("1001bd50", p_var1.add(4), read_unaligned)
            );
            mech_trace::wr!(
                "1001bd50",
                p_var1,
                write_unaligned,
                mech_trace::rd!("1001bd50", p_var1.add(2), read_unaligned)
            );
            mech_trace::wr!("1001bd50", p_var1.add(5), write_unaligned, 0);
            mech_trace::wr!(
                "1001bd50",
                p_var1.add(3),
                write_unaligned,
                mech_trace::rd!("1001bd50", p_var1.add(5), read_unaligned)
            );
            mech_trace::wr!(
                "1001bd50",
                p_var1.add(1),
                write_unaligned,
                mech_trace::rd!("1001bd50", p_var1.add(3), read_unaligned)
            );
        }
        mech_trace::rt!("1001bd50", p_var1)
    }
}
