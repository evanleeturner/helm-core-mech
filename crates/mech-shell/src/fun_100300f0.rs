// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
    fn FUN_1000be30(param_1: *mut u32) -> i32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100300f0(param_1: *mut u32) {
    mech_trace::enter!("100300f0", param_1);
    // SAFETY: the callees and the global are the game's own; param_1 points at
    // the caller's record, and the duplicate returned is a heap record of at
    // least 0x818 bytes.
    unsafe {
        let mut i_var1 = mech_trace::cr!(
            "100300f0",
            "1000ad30",
            FUN_1000ad30(
                mech_trace::gr!("100300f0", "100a678c", DAT_100a678c) as i32,
                mech_trace::rd!("100300f0", param_1.add(0x204), read_unaligned) as i32,
            )
        );
        FUN_10017ec0(
            mech_trace::gr!("100300f0", "100a678c", DAT_100a678c) as i32,
            i_var1,
            1,
        );
        i_var1 = mech_trace::cr!("100300f0", "1000be30", FUN_1000be30(param_1));
        mech_trace::cr!(
            "100300f0",
            "10017df0",
            FUN_10017df0(
                mech_trace::gr!("100300f0", "100a678c", DAT_100a678c),
                i_var1 as u32
            )
        );
        mech_trace::wr!("100300f0", param_1.add(0x205), write_unaligned, 1);
        mech_trace::wr!(
            "100300f0",
            ((i_var1 as usize).wrapping_add(0x814) as *mut u32),
            write_unaligned,
            1
        );
    }
}
