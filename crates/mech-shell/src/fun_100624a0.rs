// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a724c: *mut u32;
    static mut DAT_100a725c: *mut u32;
    static mut DAT_100a817c: *mut i32;
    static mut DAT_100a8180: u32;
    static mut DAT_100a8184: *mut i32;
    static mut DAT_100a8188: u32;
    static mut DAT_100c605c: u32;
    static mut DAT_100c6060: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100624a0() {
    mech_trace::enter!("100624a0");
    // SAFETY: the globals are the game's own memory; the pointers they hold
    // are valid objects with vtables, used exactly as the original does.
    unsafe {
        mech_trace::wr!(
            "100624a0",
            mech_trace::gr!("100624a0", "100a725c", DAT_100a725c),
            write_unaligned,
            0
        );
        if !mech_trace::gr!("100624a0", "100a817c", DAT_100a817c).is_null() {
            let obj = mech_trace::gr!("100624a0", "100a817c", DAT_100a817c);
            let vtable = mech_trace::rd!("100624a0", obj, read_unaligned);
            let f = mech_trace::rd!(
                "100624a0",
                ((vtable as u32).wrapping_add(8) as *const usize),
                read_unaligned
            );
            let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f);
            mech_trace::cr!("100624a0", "ind", f(obj));
            mech_trace::gw!("100624a0", "100a817c", DAT_100a817c, core::ptr::null_mut());
            mech_trace::gw!("100624a0", "100a8180", DAT_100a8180, 0);
            mech_trace::gw!("100624a0", "100a8188", DAT_100a8188, 0);
            if mech_trace::rd!(
                "100624a0",
                mech_trace::gr!("100624a0", "100a724c", DAT_100a724c),
                read_unaligned
            ) == 0
            {
                mech_trace::gw!("100624a0", "100a8184", DAT_100a8184, core::ptr::null_mut());
            }
        }
        if !mech_trace::gr!("100624a0", "100a8184", DAT_100a8184).is_null()
            && (mech_trace::gr!("100624a0", "100c605c", DAT_100c605c) != 0x140
                || mech_trace::gr!("100624a0", "100c6060", DAT_100c6060) != 200)
        {
            let obj = mech_trace::gr!("100624a0", "100a8184", DAT_100a8184);
            let vtable = mech_trace::rd!("100624a0", obj, read_unaligned);
            let f = mech_trace::rd!(
                "100624a0",
                ((vtable as u32).wrapping_add(8) as *const usize),
                read_unaligned
            );
            let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f);
            mech_trace::cr!("100624a0", "ind", f(obj));
            mech_trace::gw!("100624a0", "100a8184", DAT_100a8184, core::ptr::null_mut());
        }
    }
}
