// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_100f2fb0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100635ce(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100134c7(param_1: i32, param_2: i32) {
    mech_trace::enter!("100134c7", param_1, param_2);
    // SAFETY: globals and callee are the game's own; param_1 is an address of
    // the caller's record, read where the original reads.
    unsafe {
        let base = param_1 as *const u8;
        if u32::from(mech_trace::rd!("100134c7", base.byte_add(2), read))
            != mech_trace::gr!("100134c7", "100adf38", DAT_100adf38)
        {
            let entry = mech_trace::rd!(
                "100134c7",
                (&raw mut DAT_100eef40)
                    .cast::<u32>()
                    .offset(param_2 as isize),
                read_unaligned
            );
            let pi_var1 = mech_trace::rd!(
                "100134c7",
                (entry.wrapping_add(0x20) as usize as *const i32),
                read_unaligned
            ) as *mut i32;
            let mut u_var2: u16 = 1;
            let mut local_18: i32 = 0;
            while local_18 < 0xe {
                let flags =
                    mech_trace::rd!("100134c7", base.byte_add(3).cast::<u16>(), read_unaligned);
                if (u_var2 & flags) != 0 {
                    mech_trace::wr!(
                        "100134c7",
                        (&raw mut DAT_100f2fb0)
                            .cast::<u8>()
                            .byte_add(local_18.wrapping_mul(4) as usize)
                            .cast::<u32>(),
                        write_unaligned,
                        1
                    );
                }
                u_var2 = ((u_var2 as i32).wrapping_shl(1)) as u16;
                local_18 = local_18.wrapping_add(1);
            }
            mech_trace::cr!("100134c7", "100635ce", FUN_100635ce(pi_var1));
        }
    }
}
