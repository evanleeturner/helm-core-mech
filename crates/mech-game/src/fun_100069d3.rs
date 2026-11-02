// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40cc: u32;
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_100082c4(param_1: i32, param_2: i32) -> u8;
    fn FUN_10004b19(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100069d3(param_1: *mut i32, param_2: i32) {
    mech_trace::enter!("100069d3", param_1, param_2);
    // SAFETY: param_1 points at the game's object, whose first field is a
    // pointer to a structure with a field at offset 4; the callees and
    // globals are the game's own.
    unsafe {
        let state = param_1.add(0xf5);
        if mech_trace::rd!("100069d3", state, read_unaligned) != 4
            && mech_trace::rd!("100069d3", state, read_unaligned) != 5
        {
            let p0 = mech_trace::rd!("100069d3", param_1, read_unaligned);
            let f = (p0 as u32).wrapping_add(4) as usize as *const i32;
            if param_2 == 0
                || (mech_trace::rd!("100069d3", f, read_unaligned) as u32)
                    != mech_trace::gr!("100069d3", "100adf38", DAT_100adf38)
            {
                let p0 = mech_trace::rd!("100069d3", param_1, read_unaligned);
                let f = (p0 as u32).wrapping_add(4) as usize as *const i32;
                if (mech_trace::rd!("100069d3", f, read_unaligned) as u32)
                    == mech_trace::gr!("100069d3", "100adf38", DAT_100adf38)
                {
                    mech_trace::cr!("100069d3", "100082c4", FUN_100082c4(4, -1));
                }
            } else if mech_trace::gr!("100069d3", "100a40cc", DAT_100a40cc) == 0 {
                mech_trace::wr!("100069d3", state, write_unaligned, 5);
                mech_trace::cr!(
                    "100069d3",
                    "10003742",
                    FUN_10003742(0xd6, 100, 0x40, 5, 0x32)
                );
            } else {
                mech_trace::cr!("100069d3", "100082c4", FUN_100082c4(0x20, -1));
            }
            let p0 = mech_trace::rd!("100069d3", param_1, read_unaligned);
            let f = (p0 as u32).wrapping_add(4) as usize as *const i32;
            mech_trace::cr!(
                "100069d3",
                "10004b19",
                FUN_10004b19(mech_trace::rd!("100069d3", f, read_unaligned), param_1)
            );
        }
    }
}
