// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003d816(param_1: i32) {
    mech_trace::enter!("1003d816", param_1);
    if param_1 != 0 {
        // SAFETY: the globals and callees are the game's own; param_1 and the
        // nodes reached through it are the caller's live structures, read and
        // written where the original does.
        unsafe {
            if mech_trace::gr!("1003d816", "100aefc0", DAT_100aefc0) != 0 {
                mech_trace::cr!(
                    "1003d816",
                    "1003e369",
                    FUN_1003e369(mech_trace::rd!(
                        "1003d816",
                        (param_1.wrapping_add(0x6c) as *const i32),
                        read_unaligned
                    ))
                );
                mech_trace::cr!(
                    "1003d816",
                    "1003e3e3",
                    FUN_1003e3e3(
                        mech_trace::rd!(
                            "1003d816",
                            (param_1.wrapping_add(0x6c) as *const i32),
                            read_unaligned
                        ),
                        0x100c6d18,
                    )
                );
                let p = mech_trace::rd!(
                    "1003d816",
                    (param_1.wrapping_add(0x6c) as *const i32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1003d816",
                    (p.wrapping_add(0x44) as *mut u32),
                    write_unaligned,
                    9999
                );
                let p = mech_trace::rd!(
                    "1003d816",
                    (param_1.wrapping_add(0x6c) as *const i32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1003d816",
                    (p.wrapping_add(0x48) as *mut u32),
                    write_unaligned,
                    9999
                );
                let mut local_8 = mech_trace::rd!(
                    "1003d816",
                    (param_1.wrapping_add(4) as *const i32),
                    read_unaligned
                );
                while local_8 != 0 {
                    FUN_1003d816(local_8);
                    local_8 = mech_trace::rd!(
                        "1003d816",
                        (local_8.wrapping_add(8) as *const i32),
                        read_unaligned
                    );
                }
            }
        }
    }
}
