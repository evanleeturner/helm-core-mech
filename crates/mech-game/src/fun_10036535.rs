// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_100ef044: [u8; 1];
    static mut DAT_10215698: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ab38(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036535(param_1: u32, param_2: i32) -> *mut u8 {
    mech_trace::enter!("10036535", param_1, param_2);
    let _ = param_1;
    // SAFETY: the globals and callee are the game's own memory; param_2 is an
    // address of a game structure and the table entries point at game records,
    // read where the original reads.
    unsafe {
        let u_var3 = mech_trace::cr!(
            "10036535",
            "1002ab38",
            FUN_1002ab38(mech_trace::rd!(
                "10036535",
                ((param_2 as u32).wrapping_add(0xc) as *const i32),
                read_unaligned
            ))
        ) as u32;
        mech_trace::rt!(
            "10036535",
            if u_var3 < mech_trace::gr!("10036535", "100b38d8", DAT_100b38d8) {
                let table = (&raw mut DAT_100eef40).cast::<u32>();
                let entry = mech_trace::rd!("10036535", table.add(u_var3 as usize), read_unaligned);
                if mech_trace::rd!(
                    "10036535",
                    ((entry.wrapping_add(0x176)) as *const i16),
                    read_unaligned
                ) == 5
                {
                    core::ptr::null_mut()
                } else {
                    let u_var2 = mech_trace::rd!(
                        "10036535",
                        ((entry.wrapping_add(0x17a)) as *const u16),
                        read_unaligned
                    );
                    let u_var1 = u_var2 & 0xff;
                    let u_var2 = u_var2 & 0xf00;
                    let idx = (u_var1 as i16) as i32;
                    if u_var2 == 0x100 {
                        (&raw mut DAT_10215698)
                            .cast::<u8>()
                            .wrapping_offset(idx.wrapping_mul(0x54) as isize)
                    } else if u_var2 == 0x200 {
                        let e =
                            mech_trace::rd!("10036535", table.offset(idx as isize), read_unaligned);
                        e.wrapping_add(0x110) as *mut u8
                    } else if u_var2 == 0x400 {
                        (&raw mut DAT_100ef044)
                            .cast::<u8>()
                            .wrapping_offset(idx.wrapping_mul(0x40) as isize)
                    } else {
                        core::ptr::null_mut()
                    }
                }
            } else {
                core::ptr::null_mut()
            }
        )
    }
}
