// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: *mut i32;
    static mut DAT_100a6798: *mut i32;
    static mut DAT_100a67ac: *mut i32;
    static mut DAT_100a67b0: *mut i32;
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67b8: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10050400(param_1: *mut i32) -> u8;
    fn FUN_100503d0(param_1: *mut i32) -> u8;
    fn FUN_10050390(param_1: *mut i32) -> u8;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8) -> u8;
    fn FUN_1000be60(param_1: i32, param_2: i32) -> u8;
    fn FUN_1000ae80(param_1: *mut i8) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c520() {
    mech_trace::enter!("1004c520");
    let mut local_20: [u8; 9] = [0; 9];
    // SAFETY: all globals, collections and records are the game's own memory;
    // pointers are read and written at the offsets the original uses.
    unsafe {
        let mut iVar10: i32 = 0;
        if 0 < mech_trace::rd!(
            "1004c520",
            mech_trace::gr!("1004c520", "100a67ac", DAT_100a67ac).add(1),
            read_unaligned
        ) {
            loop {
                let piVar3 = mech_trace::cr!(
                    "1004c520",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1004c520", "100a67ac", DAT_100a67ac) as i32,
                        iVar10
                    )
                ) as usize as *mut i32;
                if mech_trace::rd!("1004c520", piVar3.byte_add(9).cast::<u8>(), read) == 1 {
                    let c = mech_trace::rd!("1004c520", piVar3.add(2), read_unaligned) as i8;
                    let piVar4: *mut i32;
                    let mut ok = true;
                    if c == 0 {
                        mech_trace::cr!("1004c520", "10050400", FUN_10050400(piVar3));
                        piVar4 = mech_trace::gr!("1004c520", "100a67b4", DAT_100a67b4);
                    } else if c == 1 {
                        mech_trace::cr!("1004c520", "100503d0", FUN_100503d0(piVar3));
                        piVar4 = mech_trace::gr!("1004c520", "100a67b8", DAT_100a67b8);
                    } else if c == 2 {
                        mech_trace::cr!("1004c520", "10050390", FUN_10050390(piVar3));
                        piVar4 = mech_trace::gr!("1004c520", "100a67b4", DAT_100a67b4);
                    } else {
                        ok = false;
                        piVar4 = core::ptr::null_mut();
                    }
                    if ok {
                        mech_trace::cr!(
                            "1004c520",
                            "10017df0",
                            FUN_10017df0(piVar4, piVar3 as usize as u32)
                        );
                    }
                }
                iVar10 = iVar10.wrapping_add(1);
                if !(iVar10
                    < mech_trace::rd!(
                        "1004c520",
                        mech_trace::gr!("1004c520", "100a67ac", DAT_100a67ac).add(1),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::cr!(
            "1004c520",
            "100180f0",
            FUN_100180f0(mech_trace::gr!("1004c520", "100a67ac", DAT_100a67ac), 0)
        );
        loop {
            iVar10 = 0;
            if mech_trace::rd!(
                "1004c520",
                mech_trace::gr!("1004c520", "100a67b0", DAT_100a67b0).add(1),
                read_unaligned
            ) < 1
            {
                mech_trace::cr!(
                    "1004c520",
                    "100180f0",
                    FUN_100180f0(mech_trace::gr!("1004c520", "100a67b0", DAT_100a67b0), 0)
                );
                return;
            }
            let iVar5: i32;
            loop {
                let item = mech_trace::cr!(
                    "1004c520",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1004c520", "100a67b0", DAT_100a67b0) as i32,
                        iVar10
                    )
                ) as i32;
                if mech_trace::rd!(
                    "1004c520",
                    (item as usize as *const u8).byte_add(0x84e),
                    read
                ) == 1
                {
                    iVar5 = item;
                    break;
                }
                iVar10 = iVar10.wrapping_add(1);
                if mech_trace::rd!(
                    "1004c520",
                    mech_trace::gr!("1004c520", "100a67b0", DAT_100a67b0).add(1),
                    read_unaligned
                ) <= iVar10
                {
                    mech_trace::cr!(
                        "1004c520",
                        "100180f0",
                        FUN_100180f0(mech_trace::gr!("1004c520", "100a67b0", DAT_100a67b0), 0)
                    );
                    return;
                }
            }
            mech_trace::cr!(
                "1004c520",
                "10017ec0",
                FUN_10017ec0(
                    mech_trace::gr!("1004c520", "100a67b0", DAT_100a67b0) as i32,
                    iVar5,
                    0
                )
            );
            let src = (iVar5 as usize as *const u8).byte_add(0x823);
            let mut len: usize = 0;
            while mech_trace::rd!("1004c520", src.add(len), read) != 0 {
                len += 1;
            }
            len += 1;
            let dst = local_20.as_mut_ptr();
            let mut i = 0;
            while i < len {
                mech_trace::wr!(
                    "1004c520",
                    dst.add(i),
                    write,
                    mech_trace::rd!("1004c520", src.add(i), read)
                );
                i += 1;
            }
            local_20[3] = 0x30;
            local_20[4] = 0x30;
            local_20[5] = 0x53;
            local_20[6] = 0x54;
            local_20[7] = 0x44;
            local_20[8] = 0;
            mech_trace::cr!(
                "1004c520",
                "1000be60",
                FUN_1000be60(
                    iVar5,
                    mech_trace::gr!("1004c520", "100a6798", DAT_100a6798) as i32
                )
            );
            let pcVar6 = mech_trace::cr!(
                "1004c520",
                "1000ae80",
                FUN_1000ae80(local_20.as_mut_ptr().cast::<i8>())
            );
            mech_trace::wr!(
                "1004c520",
                pcVar6.byte_add(0x810).cast::<u32>(),
                write_unaligned,
                mech_trace::rd!(
                    "1004c520",
                    (iVar5 as usize as *const u8).byte_add(0x810).cast::<u32>(),
                    read_unaligned
                )
            );
            mech_trace::cr!(
                "1004c520",
                "10017df0",
                FUN_10017df0(
                    mech_trace::gr!("1004c520", "100a6798", DAT_100a6798),
                    iVar5 as u32
                )
            );
            let mut puVar9 = (iVar5 as usize as *mut u8).byte_add(0x770).cast::<u16>();
            let mut pcVar11 = pcVar6.byte_add(0x770).cast::<u16>();
            mech_trace::cr!(
                "1004c520",
                "10017df0",
                FUN_10017df0(
                    mech_trace::gr!("1004c520", "100a678c", DAT_100a678c),
                    pcVar6 as usize as u32
                )
            );
            let mut n = 8;
            loop {
                let uVar2 = mech_trace::rd!("1004c520", pcVar11, read_unaligned);
                pcVar11 = pcVar11.byte_add(0x12);
                mech_trace::wr!("1004c520", puVar9, write_unaligned, uVar2);
                puVar9 = puVar9.add(9);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
        }
    }
}
