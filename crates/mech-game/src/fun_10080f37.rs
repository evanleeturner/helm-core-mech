// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10081106(param_1: u32, param_2: i32) -> u8;
    fn FUN_1008117d(param_1: u32, param_2: i32) -> u8;
    fn FUN_100811fd(param_1: u32, param_2: i32) -> u8;
    fn FUN_1008124c(param_1: u32, param_2: i32) -> u8;
    fn FUN_1000e511(param_1: u32, param_2: u32) -> u8;
    fn FUN_10081023(param_1: u32, param_2: i32) -> u8;
    fn FUN_10081060(param_1: u32, param_2: i32) -> u8;
    fn FUN_1008109d(param_1: u32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10080f37() -> u32 {
    mech_trace::enter!("10080f37");
    // SAFETY: the global and callees are the game's own; DAT_100b3c9c, when
    // nonzero, points at a structure whose fields are read at the original's offsets.
    unsafe {
        if mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c) != 0 {
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "10081106",
                FUN_10081106(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x14)) as *const i32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "1008117d",
                FUN_1008117d(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x18)) as *const i32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "100811fd",
                FUN_100811fd(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x1c)) as *const i32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "1008124c",
                FUN_1008124c(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x20)) as *const i32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "1000e511",
                FUN_1000e511(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x24)) as *const u32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "10081023",
                FUN_10081023(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x28)) as *const i32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "10081060",
                FUN_10081060(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x2c)) as *const i32),
                        read_unaligned
                    )
                )
            );
            let p = mech_trace::gr!("10080f37", "100b3c9c", DAT_100b3c9c);
            mech_trace::cr!(
                "10080f37",
                "1008109d",
                FUN_1008109d(
                    1,
                    mech_trace::rd!(
                        "10080f37",
                        ((p.wrapping_add(0x30)) as *const i32),
                        read_unaligned
                    )
                )
            );
        }
        mech_trace::rt!("10080f37", 1)
    }
}
