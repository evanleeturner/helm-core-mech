// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10029853(param_1: i32) -> u8;
    fn FUN_1002b4cf(param_1: *mut u32, param_2: i16, param_3: i16, param_4: i16) -> u8;
    fn FUN_1002c354(param_1: i32) -> u8;
    fn FUN_10027253(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10026e24(param_1: *mut u32) {
    mech_trace::enter!("10026e24", param_1);
    // SAFETY: param_1 points at the game's object; all offsets are read and
    // written where the original does, and the callees are the game's own.
    unsafe {
        mech_trace::cr!("10026e24", "10029853", FUN_10029853(param_1 as i32));
        mech_trace::wr!(
            "10026e24",
            param_1.add(0x5e).cast::<u16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10026e24",
            param_1.byte_add(0x17a).cast::<u16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10026e24",
            param_1.byte_add(0x17e).cast::<u32>(),
            write_unaligned,
            0
        );
        if mech_trace::rd!("10026e24", param_1.add(4), read_unaligned) == 2 {
            mech_trace::wr!(
                "10026e24",
                param_1.byte_add(0x176).cast::<u16>(),
                write_unaligned,
                0xc
            );
        } else {
            mech_trace::wr!(
                "10026e24",
                param_1.byte_add(0x176).cast::<u16>(),
                write_unaligned,
                0
            );
        }
        let mut local_8: i32 = 0;
        while local_8 < 3 {
            mech_trace::cr!(
                "10026e24",
                "1002b4cf",
                FUN_1002b4cf(param_1, local_8 as i16, -1, 0)
            );
            local_8 = local_8.wrapping_add(1);
        }
        let p = mech_trace::rd!("10026e24", param_1.add(0x51), read_unaligned) as *mut u32;
        mech_trace::wr!("10026e24", p, write_unaligned, 0);
        mech_trace::wr!(
            "10026e24",
            param_1.add(0x5b).cast::<u16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10026e24",
            param_1.add(0x5c).cast::<u16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10026e24",
            param_1.byte_add(0x18e).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::cr!("10026e24", "1002c354", FUN_1002c354(param_1 as i32));
        mech_trace::cr!("10026e24", "10027253", FUN_10027253(param_1 as i32));
    }
}
