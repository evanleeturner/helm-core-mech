// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8178: *mut i32;
    static mut DAT_100a8190: *mut i32;
    static mut DAT_100a8198: u32;
    static mut DAT_100d08e8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100624a0() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10062430() {
    mech_trace::enter!("10062430");
    // SAFETY: the globals and the callee are the game's own; the objects are
    // COM-style with a vtable pointer at offset 0, and slots are called as the
    // original does.
    unsafe {
        mech_trace::gw!("10062430", "100a8198", DAT_100a8198, 0);
        if !mech_trace::gr!("10062430", "100a8178", DAT_100a8178).is_null() {
            mech_trace::cr!("10062430", "100624a0", FUN_100624a0());

            let p = mech_trace::gr!("10062430", "100a8178", DAT_100a8178);
            let vt = mech_trace::rd!("10062430", p, read_unaligned) as u32 as usize as *const u8;
            let f = mech_trace::rd!(
                "10062430",
                vt.byte_add(0x50)
                    .cast::<extern "C" fn(*mut i32, u32, i32)>(),
                read_unaligned
            );
            mech_trace::cr!(
                "10062430",
                "ind",
                f(p, mech_trace::gr!("10062430", "100d08e8", DAT_100d08e8), 8)
            );

            let p = mech_trace::gr!("10062430", "100a8178", DAT_100a8178);
            let vt = mech_trace::rd!("10062430", p, read_unaligned) as u32 as usize as *const u8;
            let f = mech_trace::rd!(
                "10062430",
                vt.byte_add(0x4c).cast::<extern "C" fn(*mut i32)>(),
                read_unaligned
            );
            mech_trace::cr!("10062430", "ind", f(p));

            if !mech_trace::gr!("10062430", "100a8190", DAT_100a8190).is_null() {
                let q = mech_trace::gr!("10062430", "100a8190", DAT_100a8190);
                let vt =
                    mech_trace::rd!("10062430", q, read_unaligned) as u32 as usize as *const u8;
                let f = mech_trace::rd!(
                    "10062430",
                    vt.byte_add(8).cast::<extern "C" fn(*mut i32)>(),
                    read_unaligned
                );
                mech_trace::cr!("10062430", "ind", f(q));
                mech_trace::gw!("10062430", "100a8190", DAT_100a8190, core::ptr::null_mut());
            }

            let p = mech_trace::gr!("10062430", "100a8178", DAT_100a8178);
            let vt = mech_trace::rd!("10062430", p, read_unaligned) as u32 as usize as *const u8;
            let f = mech_trace::rd!(
                "10062430",
                vt.byte_add(8).cast::<extern "C" fn(*mut i32)>(),
                read_unaligned
            );
            mech_trace::cr!("10062430", "ind", f(p));
            mech_trace::gw!("10062430", "100a8178", DAT_100a8178, core::ptr::null_mut());
        }
    }
}
