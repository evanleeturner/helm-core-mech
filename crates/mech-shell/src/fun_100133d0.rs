// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
    static mut DAT_100a7248: u32;
    static mut DAT_100a724c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067a14(rect_holder: *mut i32, param_2: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_100133d0(rect_holder: i32) {
    mech_trace::enter!("100133d0", rect_holder);
    // SAFETY: the globals and the function tables they point to belong to the
    // game; the table slots hold valid code pointers when read, and
    // rect_holder points at the caller's structure.
    unsafe {
        let mut table_call_result: i32 = -1;
        if mech_trace::gr!("100133d0", "10098204", DAT_10098204) != 0 {
            let f = mech_trace::rd!(
                "100133d0",
                ((mech_trace::gr!("100133d0", "100a7248", DAT_100a7248).wrapping_add(0x20))
                    as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f as usize);
            table_call_result = mech_trace::cr!("100133d0", "ind", f());
        }
        if table_call_result == 0 {
            mech_trace::cr!(
                "100133d0",
                "10067a14",
                FUN_10067a14(rect_holder.wrapping_add(0x56) as *mut i32, 0)
            );
        }
        let g = mech_trace::rd!(
            "100133d0",
            ((mech_trace::gr!("100133d0", "100a724c", DAT_100a724c).wrapping_add(0x18))
                as *const u32),
            read_unaligned
        );
        let g: unsafe extern "C" fn() = core::mem::transmute(g as usize);
        mech_trace::cr!("100133d0", "ind", g());
    }
}
