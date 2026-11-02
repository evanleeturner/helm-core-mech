// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: u32;
    static mut DAT_100a6798: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8) -> u8;
    fn FUN_1000be30(param_1: *mut u32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c930() {
    mech_trace::enter!("1004c930");
    // SAFETY: globals and callees are the game's own; the collection and item
    // pointers are read and written at the offsets the original uses.
    unsafe {
        let mut i_var2: i32 = 0;
        if 0 < mech_trace::rd!(
            "1004c930",
            ((mech_trace::gr!("1004c930", "100a678c", DAT_100a678c).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let i_var4 = i_var2.wrapping_add(1);
                let pu_var1 = mech_trace::cr!(
                    "1004c930",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1004c930", "100a678c", DAT_100a678c) as i32,
                        i_var2
                    )
                ) as *mut u32;
                i_var2 = mech_trace::cr!(
                    "1004c930",
                    "1000ad30",
                    FUN_1000ad30(
                        mech_trace::gr!("1004c930", "100a6798", DAT_100a6798) as i32,
                        mech_trace::rd!("1004c930", pu_var1.add(0x204), read_unaligned) as i32,
                    )
                );
                let v = mech_trace::rd!(
                    "1004c930",
                    (i_var2.wrapping_add(0x80c) as u32 as *const u32),
                    read_unaligned
                );
                mech_trace::wr!("1004c930", pu_var1.add(0x203), write_unaligned, v);
                mech_trace::cr!(
                    "1004c930",
                    "10017ec0",
                    FUN_10017ec0(
                        mech_trace::gr!("1004c930", "100a6798", DAT_100a6798) as i32,
                        i_var2,
                        1
                    )
                );
                let u_var3 = mech_trace::cr!("1004c930", "1000be30", FUN_1000be30(pu_var1));
                mech_trace::cr!(
                    "1004c930",
                    "10017df0",
                    FUN_10017df0(
                        mech_trace::gr!("1004c930", "100a6798", DAT_100a6798),
                        u_var3
                    )
                );
                i_var2 = i_var4;
                if !(i_var4
                    < mech_trace::rd!(
                        "1004c930",
                        ((mech_trace::gr!("1004c930", "100a678c", DAT_100a678c).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
    }
}
