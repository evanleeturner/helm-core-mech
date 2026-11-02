// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083710: u32;
    static mut DAT_10083744: u32;
    static mut DAT_10083748: u32;
    static mut DAT_10083790: u32;
    static mut DAT_10083798: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10004620() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a180(param_1: i32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004760() {
    mech_trace::enter!("10004760");
    // SAFETY: the callees and the globals are the game's own memory; they are
    // read and written in the same order as the original.
    unsafe {
        mech_trace::cr!("10004760", "10004620", FUN_10004620());
        if mech_trace::gr!("10004760", "10083790", DAT_10083790)
            < mech_trace::gr!("10004760", "10083798", DAT_10083798)
        {
            loop {
                mech_trace::cr!(
                    "10004760",
                    "1000a180",
                    FUN_1000a180(
                        mech_trace::gr!("10004760", "10083798", DAT_10083798).wrapping_add(0x2134)
                            as i32
                    )
                );
                mech_trace::gw!(
                    "10004760",
                    "10083798",
                    DAT_10083798,
                    mech_trace::gr!("10004760", "10083798", DAT_10083798).wrapping_sub(1)
                );
                if !(mech_trace::gr!("10004760", "10083790", DAT_10083790)
                    < mech_trace::gr!("10004760", "10083798", DAT_10083798))
                {
                    break;
                }
            }
        }
        if mech_trace::gr!("10004760", "10083710", DAT_10083710) < 0x157d {
            mech_trace::gw!("10004760", "10083744", DAT_10083744, 0x32);
        } else {
            mech_trace::gw!("10004760", "10083744", DAT_10083744, 100);
            if 0x2134 < mech_trace::gr!("10004760", "10083710", DAT_10083710) {
                mech_trace::gw!("10004760", "10083744", DAT_10083744, 200);
            }
        }
        mech_trace::gw!(
            "10004760",
            "10083748",
            DAT_10083748,
            mech_trace::gr!("10004760", "10083798", DAT_10083798).wrapping_mul(mech_trace::gr!(
                "10004760",
                "10083744",
                DAT_10083744
            ))
        );
    }
}
