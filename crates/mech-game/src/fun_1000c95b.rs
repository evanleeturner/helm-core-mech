// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7570: u32;
    static mut DAT_100a7574: u32;
    static mut DAT_100a7584: u32;
    static mut DAT_100a7588: u32;
    static mut DAT_101f5518: u32;
    static mut DAT_10215624: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000d14a();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c95b(param_1: u32) -> i32 {
    mech_trace::enter!("1000c95b", param_1);
    let mut local_8: u32 = 0xFFFF_FFFF;
    // SAFETY: the globals and the callee are the game's own; the tables the
    // two base pointers name are the game's arrays, written where the original does.
    unsafe {
        if mech_trace::gr!("1000c95b", "100a7588", DAT_100a7588) == 0 {
            FUN_1000d14a();
        }
        if mech_trace::gr!("1000c95b", "100a7584", DAT_100a7584) < 0x402
            && mech_trace::gr!("1000c95b", "101f5518", DAT_101f5518)
                < mech_trace::gr!("1000c95b", "10215624", DAT_10215624)
        {
            local_8 = mech_trace::gr!("1000c95b", "100a7584", DAT_100a7584);
            let a = mech_trace::gr!("1000c95b", "100a7570", DAT_100a7570)
                .wrapping_add(mech_trace::gr!("1000c95b", "101f5518", DAT_101f5518).wrapping_mul(4))
                as *mut u32;
            mech_trace::wr!(
                "1000c95b",
                a,
                write_unaligned,
                mech_trace::gr!("1000c95b", "100a7584", DAT_100a7584)
            );
            let b = mech_trace::gr!("1000c95b", "100a7574", DAT_100a7574)
                .wrapping_add(mech_trace::gr!("1000c95b", "101f5518", DAT_101f5518).wrapping_mul(4))
                as *mut u32;
            mech_trace::wr!("1000c95b", b, write_unaligned, param_1);
            mech_trace::gw!(
                "1000c95b",
                "100a7584",
                DAT_100a7584,
                mech_trace::gr!("1000c95b", "100a7584", DAT_100a7584).wrapping_add(1)
            );
            mech_trace::gw!(
                "1000c95b",
                "101f5518",
                DAT_101f5518,
                mech_trace::gr!("1000c95b", "101f5518", DAT_101f5518).wrapping_add(1)
            );
        }
    }
    mech_trace::rt!("1000c95b", local_8 as i32)
}
