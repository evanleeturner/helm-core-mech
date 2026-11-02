// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100b9a18: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100718bd(param_1: *mut u32) {
    mech_trace::enter!("100718bd", param_1);
    // SAFETY: param_1 points at the caller's struct; the globals and the
    // callback table are the game's own memory, and the callback at +0x18
    // is a cdecl function taking one 4-byte argument.
    unsafe {
        if mech_trace::rd!("100718bd", param_1.add(1), read_unaligned) != 0 {
            mech_trace::cr!(
                "100718bd",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("100718bd", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::rd!("100718bd", param_1.add(1), read_unaligned)
                        as *mut core::ffi::c_void,
                )
            );
            mech_trace::wr!("100718bd", param_1.add(1), write_unaligned, 0);
        }
        let fp_addr =
            mech_trace::gr!("100718bd", "100b9a18", DAT_100b9a18).wrapping_add(0x18) as *const u32;
        let fp = mech_trace::rd!("100718bd", fp_addr, read_unaligned);
        let f: unsafe extern "C" fn(u32) =
            core::mem::transmute::<u32, unsafe extern "C" fn(u32)>(fp);
        mech_trace::cr!(
            "100718bd",
            "ind",
            f(mech_trace::rd!("100718bd", param_1, read_unaligned))
        );
    }
}
