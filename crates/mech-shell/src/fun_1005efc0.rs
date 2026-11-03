// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005f8c0(smack_object: *mut u8, unused_edx: i32, param_2: u32) -> u8;
    fn FUN_10012f10(
        smack_object: *mut u8,
        unused_edx: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
}

// `__cdecl`: the original pops the argument itself (`add esp,4` at
// 1005f00b) and smackw32's export ends on a bare `ret`.
#[allow(non_snake_case)]
unsafe extern "C" {
    fn _SmackDoFrame(smack_object: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_1005efc0(smack_object: *mut u8) -> u8 {
    mech_trace::enter!("1005efc0", smack_object);
    // SAFETY: the callees are the game's own; smack_object points at the
    // game's Smacker object, read and written at the original's offsets.
    unsafe {
        let rd = |o: usize| {
            mech_trace::rd!(
                "1005efc0",
                smack_object.byte_add(o).cast::<i32>(),
                read_unaligned
            )
        };
        let wr = |o: usize, v: i32| {
            mech_trace::wr!(
                "1005efc0",
                smack_object.byte_add(o).cast::<i32>(),
                write_unaligned,
                v
            )
        };

        if mech_trace::cr!(
            "1005efc0",
            "1005f8c0",
            FUN_1005f8c0(smack_object, 0, 0x40000000)
        ) != 0
        {
            return mech_trace::rt!("1005efc0", 1);
        }
        if rd(0x100) != 0 {
            if rd(0x12c) == 0
                && mech_trace::cr!("1005efc0", "1005f8c0", FUN_1005f8c0(smack_object, 0, 1)) == 0
            {
                mech_trace::cr!("1005efc0", "_SmackDoFrame", _SmackDoFrame(rd(0x100)));
                if mech_trace::cr!("1005efc0", "1005f8c0", FUN_1005f8c0(smack_object, 0, 2)) != 0 {
                    wr(0x10c, rd(0x10c) | 0x10);
                }
                wr(0x12c, 1);
            }
            if rd(0x100) != 0
                && mech_trace::cr!("1005efc0", "1005f8c0", FUN_1005f8c0(smack_object, 0, 2)) == 0
            {
                wr(0x128, rd(0x12c));
            }
        }
        if rd(0x120) == rd(0x110)
            && rd(0x114) == rd(0x124)
            && rd(0x128) == rd(0x12c)
            && mech_trace::cr!(
                "1005efc0",
                "1005f8c0",
                FUN_1005f8c0(smack_object, 0, 0x40000020)
            ) == 0
        {
            return mech_trace::rt!("1005efc0", 0);
        }
        if mech_trace::cr!("1005efc0", "1005f8c0", FUN_1005f8c0(smack_object, 0, 0x10)) != 0 {
            FUN_10012f10(
                rd(0x140) as usize as *mut u8,
                0,
                rd(0x120),
                rd(0x124),
                rd(0x118),
                rd(0x11c),
            );
            wr(0x10c, rd(0x10c) | 0x100);
        }
        wr(0x120, rd(0x110));
        wr(0x124, rd(0x114));
        wr(0x128, rd(0x12c));
        wr(0x10c, rd(0x10c) & 0xffffffefu32 as i32);
        mech_trace::rt!(
            "1005efc0",
            mech_trace::cr!(
                "1005efc0",
                "1005f8c0",
                FUN_1005f8c0(smack_object, 0, 0x40000000)
            )
        )
    }
}
