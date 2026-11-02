// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa558: u32;
    static mut DAT_100aa55c: u32;
    static mut DAT_100aa560: u32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_1007fc1d(param_1: *mut i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100135fe(param_1: i32, param_2: i32) {
    mech_trace::enter!("100135fe", param_1, param_2);
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's packet bytes and the table entries point at live game objects.
    unsafe {
        if param_2 != mech_trace::gr!("100135fe", "100adf38", DAT_100adf38) {
            let tbl = (&raw mut DAT_100eef40).cast::<u32>();
            let i_var1 = (mech_trace::rd!("100135fe", tbl.offset(param_2 as isize), read) as usize)
                .wrapping_add(0x20) as *const u32;
            let i_var1 = mech_trace::rd!("100135fe", i_var1, read_unaligned) as usize;
            let p = param_1 as usize;
            mech_trace::gw!(
                "100135fe",
                "100aa558",
                DAT_100aa558,
                mech_trace::rd!(
                    "100135fe",
                    ((p.wrapping_add(2)) as *const u32),
                    read_unaligned
                )
            );
            mech_trace::gw!(
                "100135fe",
                "100aa55c",
                DAT_100aa55c,
                mech_trace::rd!(
                    "100135fe",
                    ((p.wrapping_add(6)) as *const u32),
                    read_unaligned
                )
            );
            mech_trace::gw!(
                "100135fe",
                "100aa560",
                DAT_100aa560,
                mech_trace::rd!(
                    "100135fe",
                    ((p.wrapping_add(10)) as *const u32),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "100135fe",
                ((i_var1.wrapping_add(0x434)) as *mut u32),
                write_unaligned,
                mech_trace::rd!(
                    "100135fe",
                    ((p.wrapping_add(0xe)) as *const u32),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "100135fe",
                ((i_var1.wrapping_add(0x438)) as *mut u32),
                write_unaligned,
                mech_trace::rd!(
                    "100135fe",
                    ((p.wrapping_add(0x12)) as *const u32),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "100135fe",
                ((i_var1.wrapping_add(0x43c)) as *mut u32),
                write_unaligned,
                mech_trace::rd!(
                    "100135fe",
                    ((p.wrapping_add(0x16)) as *const u32),
                    read_unaligned
                )
            );
            let a = mech_trace::rd!(
                "100135fe",
                ((i_var1.wrapping_add(0x434)) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "100135fe",
                ((i_var1.wrapping_add(0x438)) as *const i32),
                read_unaligned
            );
            let c = mech_trace::rd!(
                "100135fe",
                ((i_var1.wrapping_add(0x43c)) as *const i32),
                read_unaligned
            );
            let mut local_8 = mech_trace::cr!("100135fe", "1000fc30", FUN_1000fc30(a, b, c));
            if 1500000 < local_8 {
                local_8 = 1500000;
            }
            let i_var1 =
                mech_trace::cr!("100135fe", "10011e60", FUN_10011e60(200, local_8, 1500000)) as i32;
            mech_trace::cr!(
                "100135fe",
                "10003742",
                FUN_10003742(0x105, i_var1, 0x40, 5, 0x32)
            );
            let t = mech_trace::rd!(
                "100135fe",
                tbl.offset(mech_trace::gr!("100135fe", "100adf38", DAT_100adf38) as isize),
                read
            ) as usize;
            let s = mech_trace::rd!("100135fe", tbl.offset(param_2 as isize), read) as usize;
            let a = mech_trace::rd!(
                "100135fe",
                ((t.wrapping_add(0x20)) as *const u32),
                read_unaligned
            ) as *mut i32;
            let b = mech_trace::rd!(
                "100135fe",
                ((s.wrapping_add(0x20)) as *const u32),
                read_unaligned
            ) as *mut i32;
            mech_trace::cr!("100135fe", "1007fc1d", FUN_1007fc1d(a, b));
        }
    }
}
