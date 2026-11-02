// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083800: u32;
    static mut DAT_10083870: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10004420(param_1: i32) -> i32;
    fn FUN_1000a180(param_1: i32) -> *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10004800();
    fn FUN_10004960();
    fn FUN_100047f0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10006330() -> i32 {
    mech_trace::enter!("10006330");
    // SAFETY: the globals and callees are the game's own; the divisor is the
    // constant 100, so the division cannot fault.
    unsafe {
        let mut i_var1 = mech_trace::gr!("10006330", "10083870", DAT_10083870) as i32;
        if -1 < (mech_trace::gr!("10006330", "10083870", DAT_10083870) as i32) {
            i_var1 = (mech_trace::gr!("10006330", "10083870", DAT_10083870) as i32) / 100;
            if (mech_trace::gr!("10006330", "10083870", DAT_10083870) as i32) % 100 != 0 {
                mech_trace::cr!(
                    "10006330",
                    "10004420",
                    FUN_10004420(mech_trace::gr!("10006330", "10083870", DAT_10083870) as i32)
                );
                mech_trace::cr!(
                    "10006330",
                    "1000a180",
                    FUN_1000a180(mech_trace::gr!("10006330", "10083870", DAT_10083870) as i32)
                );
                mech_trace::gw!(
                    "10006330",
                    "10083870",
                    DAT_10083870,
                    mech_trace::gr!("10006330", "10083800", DAT_10083800)
                );
                FUN_10004800();
                FUN_10004960();
                FUN_100047f0();
                // The C returns iVar1 from the last call, whose return type is
                // `undefined` (a byte) in an unspecified register; the value is
                // whatever the call leaves in EAX.
                return mech_trace::rt!("10006330", i_var1);
            }
        }
        mech_trace::rt!("10006330", i_var1)
    }
}
