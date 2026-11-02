// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b30: [i32; 0];
    static mut DAT_10215658: i32;
    static mut DAT_100adf34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009ce8(param_1: i32) {
    mech_trace::enter!("10009ce8", param_1);
    // SAFETY: globals and callees are the game's own; the pointers derived
    // from them point at the game's live structures, accessed as the original does.
    unsafe {
        let i_var1: i32 = mech_trace::rd!(
            "10009ce8",
            (&raw mut DAT_100a7b30)
                .cast::<i32>()
                .offset(mech_trace::gr!("10009ce8", "10215658", DAT_10215658) as isize),
            read_unaligned
        );
        if i_var1 != 0
            && mech_trace::dr!(
                "10009ce8",
                ((mech_trace::gr!("10009ce8", "100adf34", DAT_100adf34) as usize).wrapping_add(9)
                    as *const i8)
            ) != 0
        {
            let p = i_var1 as usize;
            if param_1 == 0 {
                let v = mech_trace::rd!("10009ce8", ((p + 0x20) as *const i32), read_unaligned);
                mech_trace::wr!("10009ce8", ((p + 0x18) as *mut i32), write_unaligned, v);
            } else if param_1 == 1 {
                let a = mech_trace::rd!("10009ce8", ((p + 0x18) as *const i32), read_unaligned);
                let u = mech_trace::cr!("10009ce8", "10002570", FUN_10002570(a as u32, 0x20000));
                mech_trace::wr!("10009ce8", ((p + 0x18) as *mut i32), write_unaligned, u);
                if mech_trace::rd!("10009ce8", ((p + 0x18) as *const i32), read_unaligned)
                    < mech_trace::rd!("10009ce8", ((p + 0x24) as *const i32), read_unaligned)
                {
                    let v = mech_trace::rd!("10009ce8", ((p + 0x28) as *const i32), read_unaligned);
                    mech_trace::wr!("10009ce8", ((p + 0x18) as *mut i32), write_unaligned, v);
                }
            } else if param_1 == 2 {
                let a = mech_trace::rd!("10009ce8", ((p + 0x18) as *const i32), read_unaligned);
                let r = mech_trace::cr!("10009ce8", "10003d60", FUN_10003d60(a, 0x20000));
                mech_trace::wr!("10009ce8", ((p + 0x18) as *mut i32), write_unaligned, r);
                if mech_trace::rd!("10009ce8", ((p + 0x28) as *const i32), read_unaligned)
                    < mech_trace::rd!("10009ce8", ((p + 0x18) as *const i32), read_unaligned)
                {
                    let v = mech_trace::rd!("10009ce8", ((p + 0x24) as *const i32), read_unaligned);
                    mech_trace::wr!("10009ce8", ((p + 0x18) as *mut i32), write_unaligned, v);
                }
            }
            let a = mech_trace::rd!("10009ce8", ((p + 0x20) as *const i32), read_unaligned);
            let b = mech_trace::rd!("10009ce8", ((p + 0x18) as *const i32), read_unaligned);
            let u = mech_trace::cr!("10009ce8", "10002570", FUN_10002570(a as u32, b));
            mech_trace::wr!("10009ce8", ((p + 0x2c) as *mut i32), write_unaligned, u);
        }
    }
}
