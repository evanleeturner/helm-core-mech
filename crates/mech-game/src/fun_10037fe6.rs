// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut u8;
    static mut DAT_100ae5a8: i8;
    static mut DAT_100ae5b4: i8;
    static mut DAT_100ae5c0: i8;
    static mut DAT_100ae438: i8;
    static mut DAT_100ae4c0: i8;
    static mut DAT_100ae450: i8;
    static mut DAT_100ae4c8: i8;
    static mut DAT_100ae500: i8;
    static mut DAT_100ae518: i8;
    static mut DAT_100ae468: i8;
    static mut DAT_100ae4d8: i8;
    static mut DAT_100ae480: i8;
    static mut DAT_100ae4e8: i8;
    static mut DAT_100ae490: i8;
    static mut DAT_100ae4f0: i8;
    static mut DAT_100ae4a8: i8;
    static mut DAT_100ae4f8: i8;
    static mut DAT_100ae5c8: i8;
    static mut DAT_100ae5d4: i8;
    static mut DAT_100ae528: i8;
    static mut DAT_100ae560: i8;
    static mut DAT_100ae530: i8;
    static mut DAT_100ae568: i8;
    static mut DAT_100ae540: i8;
    static mut DAT_100ae578: i8;
    static mut DAT_100ae550: i8;
    static mut DAT_100ae580: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn joyGetDevCapsA(uJoyID: u32, pjc: *mut u8, cbjc: u32) -> u32;
    fn HeapAlloc(hHeap: *mut u8, dwFlags: u32, dwBytes: u32) -> *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10038a52(joy_id: i32);
    fn FUN_10038f3a(joy_id: u32, param_2: *mut i8, param_3: *mut i8, param_4: i32) -> i32;
    fn sprintf(buffer: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1003909d(joy_id: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037fe6(joy_id: u32, param_2: *mut i8) -> u32 {
    mech_trace::enter!("10037fe6", joy_id, param_2);
    let mut local_198: [u32; 0x65] = [0; 0x65];
    // SAFETY: globals and callees are the game's own; param_2 points at the
    // caller's device structure and the heap blocks are used within their
    // allocated sizes as the original does.
    unsafe {
        let m_var2 = mech_trace::cr!(
            "10037fe6",
            "joyGetDevCapsA",
            joyGetDevCapsA(joy_id, local_198.as_mut_ptr() as *mut u8, 0x194)
        );
        if m_var2 != 0 {
            return mech_trace::rt!("10037fe6", 1);
        }
        let puvar4 = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x18
            )
        ) as *mut *const i8;
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(0x60).cast::<*mut *const i8>(),
            write_unaligned,
            puvar4
        );
        let puvar5 = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x18
            )
        ) as *mut *const i8;
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(100).cast::<*mut *const i8>(),
            write_unaligned,
            puvar5
        );
        let pivar6 = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x90
            )
        ) as *mut i32;
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(0x68).cast::<*mut i32>(),
            write_unaligned,
            pivar6
        );
        let pivar7 = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x90
            )
        ) as *mut i32;
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(0x6c).cast::<*mut i32>(),
            write_unaligned,
            pivar7
        );
        if puvar4.is_null() || puvar5.is_null() || pivar6.is_null() || pivar7.is_null() {
            FUN_10038a52(param_2 as i32);
            return mech_trace::rt!("10037fe6", 1);
        }
        let pu_var8 = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x88
            )
        ) as *mut u32;
        if pu_var8.is_null() {
            return mech_trace::rt!("10037fe6", 1);
        }
        mech_trace::wr!("10037fe6", pu_var8, write_unaligned, joy_id);
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(0x70).cast::<*mut u32>(),
            write_unaligned,
            pu_var8
        );
        if mech_trace::cr!(
            "10037fe6",
            "10038f3a",
            FUN_10038f3a(
                joy_id,
                (&raw mut local_198[0x70 / 4]) as *mut i8,
                param_2.byte_add(0xc),
                0x40,
            )
        ) == 0
        {
            mech_trace::cr!(
                "10037fe6",
                "sprintf",
                sprintf(
                    param_2.byte_add(0xc),
                    &raw const DAT_100ae5a8,
                    joy_id.wrapping_add(1),
                )
            );
        }
        mech_trace::cr!(
            "10037fe6",
            "sprintf",
            sprintf(param_2, &raw const DAT_100ae5b4, joy_id.wrapping_add(1))
        );
        FUN_1003909d(param_2 as i32, local_198.as_mut_ptr() as i32);

        mech_trace::wr!("10037fe6", puvar4, write_unaligned, &raw const DAT_100ae438);
        mech_trace::wr!("10037fe6", puvar5, write_unaligned, &raw const DAT_100ae4c0);
        mech_trace::wr!(
            "10037fe6",
            puvar4.add(1),
            write_unaligned,
            &raw const DAT_100ae450
        );
        mech_trace::wr!(
            "10037fe6",
            puvar5.add(1),
            write_unaligned,
            &raw const DAT_100ae4c8
        );
        let mut local_1a4: u32 = 2;
        let caps = local_198[24] as u8;
        let mut b_var1 = (caps & 1) == 0;
        if !b_var1 {
            let mut i_var10: i32 = 8;
            let mut pc11 = param_2.byte_add(0x4c) as *const i8;
            let mut pc12 = &raw const DAT_100ae5c0 as *const i8;
            loop {
                if i_var10 == 0 {
                    break;
                }
                i_var10 = i_var10.wrapping_sub(1);
                b_var1 = mech_trace::dr!("10037fe6", pc11) == mech_trace::dr!("10037fe6", pc12);
                pc11 = pc11.add(1);
                pc12 = pc12.add(1);
                if !b_var1 {
                    break;
                }
            }
            if b_var1 {
                mech_trace::wr!(
                    "10037fe6",
                    puvar4.add(2),
                    write_unaligned,
                    &raw const DAT_100ae500
                );
                mech_trace::wr!(
                    "10037fe6",
                    puvar5.add(2),
                    write_unaligned,
                    &raw const DAT_100ae518
                );
                local_1a4 = 3;
            } else {
                mech_trace::wr!(
                    "10037fe6",
                    puvar4.add(2),
                    write_unaligned,
                    &raw const DAT_100ae468
                );
                mech_trace::wr!(
                    "10037fe6",
                    puvar5.add(2),
                    write_unaligned,
                    &raw const DAT_100ae4d8
                );
                local_1a4 = 3;
            }
        }
        if caps & 2 != 0 {
            mech_trace::wr!(
                "10037fe6",
                puvar4.add(local_1a4 as usize),
                write_unaligned,
                &raw const DAT_100ae480
            );
            mech_trace::wr!(
                "10037fe6",
                puvar5.add(local_1a4 as usize),
                write_unaligned,
                &raw const DAT_100ae4e8
            );
            local_1a4 = local_1a4.wrapping_add(1);
        }
        if caps & 4 != 0 {
            mech_trace::wr!(
                "10037fe6",
                puvar4.add(local_1a4 as usize),
                write_unaligned,
                &raw const DAT_100ae490
            );
            mech_trace::wr!(
                "10037fe6",
                puvar5.add(local_1a4 as usize),
                write_unaligned,
                &raw const DAT_100ae4f0
            );
            local_1a4 = local_1a4.wrapping_add(1);
        }
        if caps & 8 != 0 {
            mech_trace::wr!(
                "10037fe6",
                puvar4.add(local_1a4 as usize),
                write_unaligned,
                &raw const DAT_100ae4a8
            );
            mech_trace::wr!(
                "10037fe6",
                puvar5.add(local_1a4 as usize),
                write_unaligned,
                &raw const DAT_100ae4f8
            );
            local_1a4 = local_1a4.wrapping_add(1);
        }
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(0x58).cast::<u32>(),
            write_unaligned,
            local_1a4
        );

        let pv = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x180
            )
        );
        mech_trace::wr!("10037fe6", pivar6, write_unaligned, pv as i32);
        let pv = mech_trace::cr!(
            "10037fe6",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10037fe6", "100adf74", DAT_100adf74),
                9,
                0x180
            )
        );
        mech_trace::wr!("10037fe6", pivar7, write_unaligned, pv as i32);
        if mech_trace::rd!("10037fe6", pivar6, read_unaligned) == 0
            || mech_trace::rd!("10037fe6", pivar7, read_unaligned) == 0
        {
            FUN_10038a52(param_2 as i32);
            return mech_trace::rt!("10037fe6", 1);
        }
        local_1a4 = 0;
        while (local_1a4 as i32) < (local_198[15] as i32) {
            let i = local_1a4 as usize;
            let a = mech_trace::rd!("10037fe6", pivar6.add(i), read_unaligned);
            mech_trace::cr!(
                "10037fe6",
                "sprintf",
                sprintf(
                    a as usize as *mut i8,
                    &raw const DAT_100ae5c8,
                    local_1a4.wrapping_add(1),
                )
            );
            let b = mech_trace::rd!("10037fe6", pivar7.add(i), read_unaligned);
            mech_trace::cr!(
                "10037fe6",
                "sprintf",
                sprintf(
                    b as usize as *mut i8,
                    &raw const DAT_100ae5d4,
                    local_1a4.wrapping_add(1),
                )
            );
            let a = mech_trace::rd!("10037fe6", pivar6.add(i), read_unaligned);
            mech_trace::wr!(
                "10037fe6",
                pivar6.add(i.wrapping_add(1)),
                write_unaligned,
                ((a as u32).wrapping_add(0xc)) as i32
            );
            let b = mech_trace::rd!("10037fe6", pivar7.add(i), read_unaligned);
            mech_trace::wr!(
                "10037fe6",
                pivar7.add(i.wrapping_add(1)),
                write_unaligned,
                ((b as u32).wrapping_add(0xc)) as i32
            );
            local_1a4 = local_1a4.wrapping_add(1);
        }
        local_1a4 = local_198[15];
        if caps & 0x10 != 0 {
            let n = local_198[15] as usize;
            mech_trace::wr!(
                "10037fe6",
                pivar6.add(n),
                write_unaligned,
                (&raw const DAT_100ae528) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar7.add(n),
                write_unaligned,
                (&raw const DAT_100ae560) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar6.add(n.wrapping_add(1)),
                write_unaligned,
                (&raw const DAT_100ae530) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar7.add(n.wrapping_add(1)),
                write_unaligned,
                (&raw const DAT_100ae568) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar6.add(n.wrapping_add(2)),
                write_unaligned,
                (&raw const DAT_100ae540) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar7.add(n.wrapping_add(2)),
                write_unaligned,
                (&raw const DAT_100ae578) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar6.add(n.wrapping_add(3)),
                write_unaligned,
                (&raw const DAT_100ae550) as i32
            );
            mech_trace::wr!(
                "10037fe6",
                pivar7.add(n.wrapping_add(3)),
                write_unaligned,
                (&raw const DAT_100ae580) as i32
            );
            local_1a4 = local_198[15].wrapping_add(4);
        }
        mech_trace::wr!(
            "10037fe6",
            param_2.byte_add(0x5c).cast::<u32>(),
            write_unaligned,
            local_1a4
        );
        mech_trace::rt!("10037fe6", 0)
    }
}
