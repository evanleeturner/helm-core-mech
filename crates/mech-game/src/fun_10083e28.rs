// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a1c: *mut u32;
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f78: u32;
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f80: u32;
    static mut DAT_100c967c: u32;
    static mut DAT_100c9680: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083e28() {
    mech_trace::enter!("10083e28");
    // SAFETY: the globals are the game's own memory; the object pointers are
    // live objects whose first word is a vtable address, with the function
    // pointer at offset 8, called as the original does.
    unsafe {
        mech_trace::wr!(
            "10083e28",
            mech_trace::gr!("10083e28", "100b9a2c", DAT_100b9a2c),
            write_unaligned,
            0
        );
        if !mech_trace::gr!("10083e28", "100b9f74", DAT_100b9f74).is_null() {
            let obj = mech_trace::gr!("10083e28", "100b9f74", DAT_100b9f74);
            let vt = mech_trace::rd!("10083e28", obj, read_unaligned) as usize as *const u8;
            let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(mech_trace::rd!(
                "10083e28",
                vt.add(8).cast::<usize>(),
                read_unaligned
            ));
            mech_trace::cr!("10083e28", "ind", f(obj));
            mech_trace::gw!("10083e28", "100b9f74", DAT_100b9f74, core::ptr::null_mut());
            mech_trace::gw!("10083e28", "100b9f78", DAT_100b9f78, 0);
            mech_trace::gw!("10083e28", "100b9f80", DAT_100b9f80, 0);
            if mech_trace::rd!(
                "10083e28",
                mech_trace::gr!("10083e28", "100b9a1c", DAT_100b9a1c),
                read_unaligned
            ) == 0
            {
                mech_trace::gw!("10083e28", "100b9f7c", DAT_100b9f7c, core::ptr::null_mut());
            }
        }
        if !mech_trace::gr!("10083e28", "100b9f7c", DAT_100b9f7c).is_null()
            && (mech_trace::gr!("10083e28", "100c967c", DAT_100c967c) != 0x140
                || mech_trace::gr!("10083e28", "100c9680", DAT_100c9680) != 200)
        {
            let obj = mech_trace::gr!("10083e28", "100b9f7c", DAT_100b9f7c);
            let vt = mech_trace::rd!("10083e28", obj, read_unaligned) as usize as *const u8;
            let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(mech_trace::rd!(
                "10083e28",
                vt.add(8).cast::<usize>(),
                read_unaligned
            ));
            mech_trace::cr!("10083e28", "ind", f(obj));
            mech_trace::gw!("10083e28", "100b9f7c", DAT_100b9f7c, core::ptr::null_mut());
        }
    }
}
