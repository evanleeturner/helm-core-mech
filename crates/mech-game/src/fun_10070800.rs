// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: i32;
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007078e(param_1: i32);
    fn FUN_100707b3(param_1: i32, param_2: *mut i32) -> *mut u32;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: i32, param_5: i32);
    fn FUN_100709b2(param_1: i32, param_2: i32, param_3: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070800(param_1: *mut i32) {
    mech_trace::enter!("10070800", param_1);
    let mut local_14: [u32; 3] = [0; 3];
    let mut local_8: *mut u32 = core::ptr::null_mut();
    // SAFETY: param_1 points at the game's structure; every pointer derived
    // from its fields is the game's own memory, read and written where the
    // original does. Callees and globals are the game's own.
    unsafe {
        let p8 = mech_trace::rd!("10070800", param_1.add(8), read_unaligned);
        let v = mech_trace::rd!(
            "10070800",
            ((p8 as u32 as usize + 0x4c) as *const i32),
            read_unaligned
        );
        if v < 0x421 {
            FUN_1007078e(param_1 as usize as i32);
        } else {
            let mut local_18 = (v as u32).wrapping_add(0xfffffc00) as i32;
            if local_18 < 0 {
                local_18 = 0;
            }
            mech_trace::wr!("10070800", param_1.add(0x22), write_unaligned, 0);
            if local_18 < 0x100 {
                mech_trace::wr!("10070800", param_1.add(0x23), write_unaligned, 1);
            } else if local_18 < 0x300 {
                mech_trace::wr!("10070800", param_1.add(0x23), write_unaligned, 2);
            } else {
                mech_trace::wr!("10070800", param_1.add(0x23), write_unaligned, 3);
                mech_trace::wr!("10070800", param_1.add(0x22), write_unaligned, 1);
            }
            if mech_trace::rd!("10070800", param_1.add(0x21), read_unaligned) == 2
                && 2 < mech_trace::rd!("10070800", param_1.add(0x23), read_unaligned)
            {
                mech_trace::wr!("10070800", param_1.add(0x23), write_unaligned, 2);
            }
            let p13 = mech_trace::rd!("10070800", param_1.add(0x13), read_unaligned);
            if mech_trace::rd!(
                "10070800",
                ((p13 as u32 as usize + 0x30) as *const i8),
                read
            ) != 0
            {
                mech_trace::wr!("10070800", param_1.add(0x22), write_unaligned, 2);
            }
            if mech_trace::rd!("10070800", param_1, read_unaligned) == 4 {
                let t = mech_trace::rd!("10070800", param_1.add(0x22), read_unaligned);
                mech_trace::wr!("10070800", param_1.add(0x21), write_unaligned, t);
            }
        }
        if mech_trace::rd!("10070800", param_1.add(0x24), read_unaligned) != -1
            && (mech_trace::rd!("10070800", param_1.add(0x20).cast::<u8>(), read) & 8) != 0
        {
            let f = mech_trace::rd!("10070800", param_1.add(0x20), read_unaligned);
            mech_trace::wr!(
                "10070800",
                param_1.add(0x20),
                write_unaligned,
                f & (0xfffffff7u32 as i32)
            );
            local_8 = mech_trace::cr!(
                "10070800",
                "100707b3",
                FUN_100707b3(param_1 as usize as i32, local_14.as_mut_ptr().cast::<i32>())
            );
            FUN_100037f0(
                local_14[0],
                local_14[1],
                local_14[2],
                mech_trace::rd!("10070800", param_1.add(0x24), read_unaligned),
                mech_trace::gr!("10070800", "100ab094", DAT_100ab094),
            );
        }
        if mech_trace::rd!("10070800", param_1.add(1), read_unaligned)
            == mech_trace::gr!("10070800", "100adf38", DAT_100adf38)
        {
            FUN_100709b2(param_1 as usize as i32, 0x100b47e8, local_8);
        }
    }
}
