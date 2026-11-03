// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017c90(param_1: *mut u8, param_2: i32, param_3: u32, param_4: u8, param_5: u32)
    -> u32;
    fn FUN_1006f200(param_1: u32) -> u32;
    fn FUN_10017df0(param_1: u32, param_2: *mut u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050c30(
        this: u32,
        param_1: i32,
        param_2: i32,
        param_3: *const i8,
        param_4: *mut u8,
    ) -> u32;
    fn FUN_10064b60(
        this: u32,
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u8,
        param_7: u32,
        param_8: u32,
        param_9: *const i8,
        param_10: u32,
        param_11: u32,
        param_12: *mut u8,
        param_13: u8,
        param_14: u32,
    ) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10064610(
    this: *mut u8,
    param_1: u32,
    param_2: u32,
    param_3: u8,
    param_4: i32,
    param_5: i32,
    param_6: u32,
) -> *mut u32 {
    mech_trace::enter!(
        "10064610", this, param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: `this` is the caller's object and param_4 points at its record
    // array; callees are the game's own. Accesses mirror the original.
    unsafe {
        mech_trace::wr!("10064610", this.byte_add(0x110), write, param_3);
        mech_trace::wr!(
            "10064610",
            this.byte_add(4).cast::<u32>(),
            write_unaligned,
            param_1
        );
        mech_trace::wr!(
            "10064610",
            this.byte_add(8).cast::<u32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!("10064610", this.byte_add(0xc), write, 0xff);
        mech_trace::wr!(
            "10064610",
            this.byte_add(0x10c).cast::<u32>(),
            write_unaligned,
            param_6
        );
        let mut i: i32 = 1;
        loop {
            mech_trace::wr!(
                "10064610",
                this.byte_add(i as usize).byte_add(0xc),
                write,
                i as u8
            );
            i = i.wrapping_add(1);
            if i >= 0x100 {
                break;
            }
        }
        mech_trace::wr!("10064610", this.byte_add(0xd), write, 0x16);
        mech_trace::cr!("10064610", "10017c90", FUN_10017c90(this, 10, 0, 4, 0));
        let mut local_14: i32 = 0;
        if 0 < param_5 {
            let mut p = (param_4 as u32).wrapping_add(0x10) as *const i32;
            loop {
                let mut pc =
                    mech_trace::rd!("10064610", p.byte_add(8), read_unaligned) as u32 as *const i8;
                let local_1c: u32;
                if pc.is_null() || mech_trace::rd!("10064610", pc, read) != 0x3c {
                    local_1c = 0;
                } else {
                    pc = pc.add(1);
                    local_1c = mech_trace::cr!(
                        "10064610",
                        "10050c30",
                        FUN_10050c30(
                            mech_trace::rd!(
                                "10064610",
                                this.byte_add(8).cast::<u32>(),
                                read_unaligned
                            ),
                            mech_trace::rd!("10064610", p, read_unaligned),
                            mech_trace::rd!("10064610", p.add(1), read_unaligned),
                            pc,
                            this.byte_add(0xc),
                        )
                    );
                }
                let this_00 = mech_trace::cr!("10064610", "1006f200", FUN_1006f200(0x2e));
                let mut pu: *mut u32 = core::ptr::null_mut();
                if this_00 != 0 {
                    pu = mech_trace::cr!(
                        "10064610",
                        "10064b60",
                        FUN_10064b60(
                            this_00,
                            mech_trace::rd!("10064610", p.sub(4), read_unaligned) as u32,
                            mech_trace::rd!("10064610", p.sub(3), read_unaligned) as u32,
                            mech_trace::rd!("10064610", p.sub(2), read_unaligned) as u32,
                            mech_trace::rd!("10064610", p.sub(1), read_unaligned) as u32,
                            local_14 as u32,
                            mech_trace::rd!("10064610", this.byte_add(0x110), read),
                            mech_trace::rd!(
                                "10064610",
                                this.byte_add(4).cast::<u32>(),
                                read_unaligned
                            ),
                            mech_trace::rd!(
                                "10064610",
                                this.byte_add(8).cast::<u32>(),
                                read_unaligned
                            ),
                            pc,
                            mech_trace::rd!("10064610", p, read_unaligned) as u32,
                            mech_trace::rd!("10064610", p.add(1), read_unaligned) as u32,
                            this.byte_add(0xc),
                            1,
                            mech_trace::rd!(
                                "10064610",
                                this.byte_add(0x10c).cast::<u32>(),
                                read_unaligned
                            ),
                        )
                    );
                }
                p = p.add(7);
                mech_trace::wr!("10064610", pu.add(8), write_unaligned, local_1c);
                mech_trace::cr!(
                    "10064610",
                    "10017df0",
                    FUN_10017df0(
                        mech_trace::rd!("10064610", this.cast::<u32>(), read_unaligned),
                        pu
                    )
                );
                local_14 = local_14.wrapping_add(1);
                if param_5 <= local_14 {
                    break;
                }
            }
        }
    }
    mech_trace::rt!("10064610", this.cast::<u32>())
}
