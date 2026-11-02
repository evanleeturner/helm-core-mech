// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bcc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10044cc4(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063818(param_1: *mut i32) {
    mech_trace::enter!("10063818", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's structure; the globals and the callee
    // are the game's own; all offsets mirror the original's reads and writes.
    unsafe {
        let i_var2 = mech_trace::rd!("10063818", param_1.add(0xf7), read_unaligned);
        let mut local_14: i32 = 0;
        while local_14 < i_var2 {
            let i_var3 = mech_trace::rd!("10063818", param_1.add(0x15), read_unaligned)
                .wrapping_add(local_14.wrapping_mul(0x40));
            let i_var1 = mech_trace::rd!(
                "10063818",
                ((i_var3.wrapping_add(4)) as *const i32),
                read_unaligned
            )
            .wrapping_mul(0x68);
            if mech_trace::rd!(
                "10063818",
                ((i_var3.wrapping_add(8)) as *const i32),
                read_unaligned
            ) == 1
            {
                mech_trace::wr!(
                    "10063818",
                    ((i_var3.wrapping_add(8)) as *mut i32),
                    write_unaligned,
                    2
                );
                let src = (&raw mut DAT_100a7bcc).byte_offset(i_var1 as isize) as *const i32;
                mech_trace::wr!(
                    "10063818",
                    ((i_var3.wrapping_add(0x24)) as *mut i32),
                    write_unaligned,
                    mech_trace::rd!("10063818", src, read_unaligned)
                );
                mech_trace::wr!(
                    "10063818",
                    ((i_var3.wrapping_add(0x10)) as *mut i32),
                    write_unaligned,
                    0
                );
                mech_trace::cr!("10063818", "10044cc4", FUN_10044cc4(i_var3, param_1));
                let flag = (&raw mut DAT_100a7bc8).byte_offset(i_var1 as isize) as *const i32;
                if mech_trace::rd!("10063818", flag, read_unaligned) == 0
                    || mech_trace::rd!(
                        "10063818",
                        (param_1.add(0x110) as *const u16),
                        read_unaligned
                    ) & 0x80
                        == 0
                {
                    mech_trace::wr!(
                        "10063818",
                        ((i_var3.wrapping_add(0x1c)) as *mut i32),
                        write_unaligned,
                        0
                    );
                    let v = mech_trace::rd!(
                        "10063818",
                        ((i_var3.wrapping_add(0x1c)) as *const i32),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "10063818",
                        ((i_var3.wrapping_add(0x18)) as *mut i32),
                        write_unaligned,
                        v
                    );
                } else {
                    let base = mech_trace::rd!("10063818", param_1, read_unaligned);
                    let a = mech_trace::rd!(
                        "10063818",
                        ((base.wrapping_add(0xdc)) as *const u32),
                        read_unaligned
                    ) & 0xff;
                    mech_trace::wr!(
                        "10063818",
                        ((i_var3.wrapping_add(0x18)) as *mut u32),
                        write_unaligned,
                        a
                    );
                    let base = mech_trace::rd!("10063818", param_1, read_unaligned);
                    let b = mech_trace::rd!(
                        "10063818",
                        ((base.wrapping_add(0xdc)) as *const u32),
                        read_unaligned
                    ) & 0xf00;
                    mech_trace::wr!(
                        "10063818",
                        ((i_var3.wrapping_add(0x1c)) as *mut u32),
                        write_unaligned,
                        b
                    );
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
    }
}
