// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1e4: *mut u16;
    static mut DAT_100aa2b0: u16;
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_100f2ff0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: u32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100133f8() {
    mech_trace::enter!("100133f8");
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointer in DAT_100aa1e4 and the table entries are valid in the original.
    unsafe {
        mech_trace::wr!(
            "100133f8",
            mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4),
            write_unaligned,
            mech_trace::gr!("100133f8", "100aa2b0", DAT_100aa2b0)
        );
        let entry = mech_trace::rd!(
            "100133f8",
            (&raw mut DAT_100eef40).cast::<u32>().add(mech_trace::gr!(
                "100133f8",
                "100adf38",
                DAT_100adf38
            ) as usize),
            read_unaligned
        );
        let b = mech_trace::rd!("100133f8", ((entry.wrapping_add(4)) as *const u8), read);
        mech_trace::wr!(
            "100133f8",
            mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4)
                .byte_add(2)
                .cast::<u8>(),
            write,
            b
        );
        mech_trace::wr!(
            "100133f8",
            mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4).byte_add(3),
            write_unaligned,
            0
        );
        let mut local_10: i32 = 0;
        while local_10 < 0xe {
            let slot = (&raw mut DAT_100f2ff0)
                .cast::<u8>()
                .byte_add(local_10.wrapping_mul(4) as usize)
                .cast::<i32>();
            if mech_trace::rd!("100133f8", slot, read_unaligned) != 0 {
                let cur = mech_trace::rd!(
                    "100133f8",
                    mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4).byte_add(3),
                    read_unaligned
                );
                let bit = (1u32.wrapping_shl((local_10 as u8 & 0x1f) as u32)) as u16;
                mech_trace::wr!(
                    "100133f8",
                    mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4).byte_add(3),
                    write_unaligned,
                    cur | bit
                );
                mech_trace::wr!("100133f8", slot, write_unaligned, 0);
            }
            local_10 = local_10.wrapping_add(1);
        }
        if mech_trace::rd!(
            "100133f8",
            mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4).byte_add(3),
            read_unaligned
        ) != 0
        {
            mech_trace::cr!(
                "100133f8",
                "10006e00",
                FUN_10006e00(
                    mech_trace::gr!("100133f8", "100aa1e4", DAT_100aa1e4) as u32,
                    5
                )
            );
        }
    }
}
