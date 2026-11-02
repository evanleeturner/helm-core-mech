// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0f18: i32;
    static mut DAT_100b0f1c: u32;
    static mut DAT_100b0f20: u32;
    static mut DAT_1011515c: u32;
    static mut DAT_10115170: u32;
    static mut DAT_10167e70: u32;
    static mut DAT_1016828c: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100547e0(param_1: *mut i32, param_2: *mut i32) -> u8;
    fn FUN_10054330(param_1: i32, param_2: i32) -> u32;
    fn FUN_10054e09(param_1: *mut i16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10054a76() {
    mech_trace::enter!("10054a76");
    // SAFETY: the globals, the callees and the tables they point at are the
    // game's own memory; reads, writes and calls happen in the original order.
    unsafe {
        if 1 < mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18) {
            let base = mech_trace::gr!("10054a76", "1016828c", DAT_1016828c);
            mech_trace::cr!(
                "10054a76",
                "100547e0",
                FUN_100547e0(
                    base,
                    base.wrapping_offset(
                        mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18)
                            .wrapping_sub(1)
                            .wrapping_mul(8) as isize
                    ),
                )
            );
        }
        let i_var1 = mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18);
        mech_trace::gw!("10054a76", "100b0f18", DAT_100b0f18, 0);
        mech_trace::gw!(
            "10054a76",
            "10115170",
            DAT_10115170,
            mech_trace::gr!("10054a76", "10167e70", DAT_10167e70)
        );
        let mut local_c: i32 = 0;
        loop {
            let i_var2 = mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18);
            if !(local_c < i_var1) {
                break;
            }
            let entry = mech_trace::gr!("10054a76", "1016828c", DAT_1016828c)
                .wrapping_offset(local_c.wrapping_mul(8) as isize);
            let first = mech_trace::rd!("10054a76", entry, read_unaligned);
            let flags = mech_trace::rd!("10054a76", (first as usize as *const i16), read_unaligned);
            if ((flags as i32) as u32 & 0x8000) == 0 {
                let mut src = entry as *const i32;
                let mut dst = mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18)
                    .wrapping_mul(0x20)
                    .wrapping_add(mech_trace::gr!("10054a76", "10167e70", DAT_10167e70) as i32)
                    as u32 as usize as *mut i32;
                let mut n = 8;
                while n != 0 {
                    mech_trace::wr!(
                        "10054a76",
                        dst,
                        write_unaligned,
                        mech_trace::rd!("10054a76", src, read_unaligned)
                    );
                    src = src.wrapping_add(1);
                    dst = dst.wrapping_add(1);
                    n -= 1;
                }
                mech_trace::gw!(
                    "10054a76",
                    "100b0f18",
                    DAT_100b0f18,
                    mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18).wrapping_add(1)
                );
            } else if mech_trace::gr!("10054a76", "1011515c", DAT_1011515c) != 0 {
                let a = mech_trace::rd!(
                    "10054a76",
                    mech_trace::gr!("10054a76", "1016828c", DAT_1016828c)
                        .wrapping_offset(local_c.wrapping_mul(8) as isize),
                    read_unaligned
                );
                let b = mech_trace::rd!(
                    "10054a76",
                    mech_trace::gr!("10054a76", "1016828c", DAT_1016828c)
                        .wrapping_offset(local_c.wrapping_mul(8).wrapping_add(1) as isize),
                    read_unaligned
                );
                mech_trace::cr!("10054a76", "10054330", FUN_10054330(a, b));
                if mech_trace::gr!("10054a76", "1011515c", DAT_1011515c) == 0 {
                    break;
                }
                if 1 < mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18).wrapping_sub(i_var2) {
                    let p1 = i_var2.wrapping_mul(0x20).wrapping_add(mech_trace::gr!(
                        "10054a76",
                        "10167e70",
                        DAT_10167e70
                    ) as i32) as u32 as usize as *mut i32;
                    let p2 = mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18)
                        .wrapping_sub(1)
                        .wrapping_mul(0x20)
                        .wrapping_add(mech_trace::gr!("10054a76", "10167e70", DAT_10167e70) as i32)
                        as u32 as usize as *mut i32;
                    mech_trace::cr!("10054a76", "100547e0", FUN_100547e0(p1, p2));
                }
            }
            if mech_trace::gr!("10054a76", "100b0f20", DAT_100b0f20)
                <= mech_trace::gr!("10054a76", "100b0f1c", DAT_100b0f1c)
            {
                break;
            }
            local_c = local_c.wrapping_add(1);
        }
        local_c = 0;
        while local_c < mech_trace::gr!("10054a76", "100b0f18", DAT_100b0f18) {
            let addr = local_c.wrapping_mul(0x20).wrapping_add(mech_trace::gr!(
                "10054a76",
                "10167e70",
                DAT_10167e70
            ) as i32) as u32 as usize as *const u32;
            let p = mech_trace::rd!("10054a76", addr, read_unaligned) as usize as *mut i16;
            mech_trace::cr!("10054a76", "10054e09", FUN_10054e09(p));
            local_c = local_c.wrapping_add(1);
        }
    }
}
