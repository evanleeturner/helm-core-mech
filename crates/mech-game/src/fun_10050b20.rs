// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 1];
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
    fn FUN_10050389(param_1: i32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001a620() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10050b20() {
    mech_trace::enter!("10050b20");
    let mut arr = [0i32; 180];
    let a = arr.as_mut_ptr();
    let mut local_3c: i32 = 0;
    let mut local_50: i32 = 0;
    let mut local_38: u32 = 0;
    let mut local_20: i32 = 0;
    // SAFETY: globals, callees and the pointed-to game structures belong to
    // the game; the local array holds the per-entry records as the original.
    unsafe {
        let mut local_4c: i32 = 0;
        let mut local_32c: i32 = -1;
        let mut local_10: i32 = -1;
        let mut local_c: i32 = -1;
        let base = mech_trace::gr!("10050b20", "100aaf68", PTR_DAT_100aaf68);
        let local_8 = mech_trace::rd!(
            "10050b20",
            ((base.wrapping_add(0xb8)) as *const i32),
            read_unaligned
        );
        let local_44 = mech_trace::cr!("10050b20", "10003d60", FUN_10003d60(local_8, 0xe10)) as u32;
        let local_14 = mech_trace::cr!("10050b20", "10003d60", FUN_10003d60(local_8, 0x2134));
        let local_28 = mech_trace::cr!("10050b20", "10003d60", FUN_10003d60(local_8, 0x57e4));
        let _local_34 = mech_trace::cr!("10050b20", "10003d60", FUN_10003d60(local_8, 40000));
        let mut local_1c: u32 = local_44;
        let local_30 = mech_trace::rd!("10050b20", (base as *const i32), read_unaligned);
        let local_40 = mech_trace::rd!(
            "10050b20",
            (base.wrapping_add(4) as *const i32),
            read_unaligned
        );
        let local_48 = mech_trace::rd!(
            "10050b20",
            (base.wrapping_add(8) as *const i32),
            read_unaligned
        );
        let tbl = (&raw mut DAT_100eef40).cast::<u32>();

        let mut i: i32 = 0;
        while i < mech_trace::gr!("10050b20", "100b38d8", DAT_100b38d8) {
            let p = a.offset((i as isize) * 3);
            mech_trace::wr!("10050b20", p.add(2), write, i);
            let iv1 = mech_trace::rd!("10050b20", tbl.offset(i as isize), read);
            if i == mech_trace::gr!("10050b20", "100adf38", DAT_100adf38)
                && mech_trace::cr!("10050b20", "1001a620", FUN_1001a620()) == 0
            {
                mech_trace::wr!("10050b20", p.add(1), write, 4);
                mech_trace::wr!("10050b20", p, write, 0);
            } else if (mech_trace::rd!(
                "10050b20",
                (iv1.wrapping_add(0x14) as *const u16),
                read_unaligned
            ) & 2)
                == 0
            {
                mech_trace::wr!("10050b20", p.add(1), write, -2);
                let local_18 = mech_trace::rd!(
                    "10050b20",
                    (iv1.wrapping_add(0x58) as *const i32),
                    read_unaligned
                )
                .wrapping_sub(local_48) as u32;
                mech_trace::cr!(
                    "10050b20",
                    "10083a59",
                    FUN_10083a59(
                        mech_trace::rd!(
                            "10050b20",
                            (iv1.wrapping_add(0x50) as *const i32),
                            read_unaligned
                        )
                        .wrapping_sub(local_30) as u32,
                        mech_trace::rd!(
                            "10050b20",
                            (iv1.wrapping_add(0x54) as *const i32),
                            read_unaligned
                        )
                        .wrapping_sub(local_40) as u32,
                        local_18,
                        &raw mut local_3c,
                        (&raw mut local_50).cast::<u32>(),
                        &raw mut local_38,
                        &raw mut local_20,
                    )
                );
                mech_trace::wr!("10050b20", p, write, local_50);
                if (local_38 as i32) < (local_1c as i32) {
                    local_1c = local_38;
                    local_c = local_10;
                    local_10 = local_32c;
                    local_32c = i;
                }
            } else if i == mech_trace::gr!("10050b20", "100adf38", DAT_100adf38) {
                mech_trace::wr!("10050b20", p.add(1), write, 0);
            } else {
                mech_trace::wr!("10050b20", p.add(1), write, 1);
            }
            i = i.wrapping_add(1);
        }

        i = 0;
        while i < mech_trace::gr!("10050b20", "100b38d8", DAT_100b38d8) {
            let p = a.offset((i as isize) * 3);
            if mech_trace::rd!("10050b20", p.add(1), read) == -2 {
                let idx = mech_trace::rd!("10050b20", p.add(2), read);
                let d = mech_trace::rd!("10050b20", p, read);
                if idx == local_32c && d < (local_44 as i32) {
                    mech_trace::wr!("10050b20", p.add(1), write, 0);
                } else if idx == local_10 || (idx == local_c && d < local_14) {
                    local_4c = local_4c.wrapping_add(1);
                    mech_trace::wr!("10050b20", p.add(1), write, 1);
                } else if (local_10 == -1 || local_c == -1) && d < local_14 && local_4c < 3 {
                    local_4c = local_4c.wrapping_add(1);
                    mech_trace::wr!("10050b20", p.add(1), write, 1);
                } else if d < local_28 {
                    mech_trace::wr!("10050b20", p.add(1), write, 2);
                } else {
                    mech_trace::wr!("10050b20", p.add(1), write, 3);
                }
            }
            let iv1 = mech_trace::rd!(
                "10050b20",
                tbl.offset(mech_trace::rd!("10050b20", p.add(2), read) as isize),
                read
            );
            let local_24 = mech_trace::rd!(
                "10050b20",
                (iv1.wrapping_add(0x1c) as *const i32),
                read_unaligned
            );
            let st = mech_trace::rd!("10050b20", p.add(1), read);
            if -1 < st
                && st != local_24
                && mech_trace::cr!(
                    "10050b20",
                    "10050389",
                    FUN_10050389(
                        mech_trace::rd!(
                            "10050b20",
                            (iv1.wrapping_add(4) as *const i32),
                            read_unaligned
                        ),
                        st
                    )
                ) != 0
            {
                mech_trace::wr!(
                    "10050b20",
                    (iv1.wrapping_add(0x1c) as *mut i32),
                    write_unaligned,
                    st
                );
            }
            i = i.wrapping_add(1);
        }
    }
}
