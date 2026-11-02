// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c4c88: i32;
    static mut DAT_100c4c78: u32;
    static mut DAT_100c4c7c: u32;
    static mut DAT_100c4c80: u32;
    static mut DAT_100c4c84: u32;
    static mut DAT_100a40b4: i32;
    static mut DAT_100a40a0: i32;
    static mut DAT_100a40c0: u32;
    static mut DAT_100a40c4: u32;
    static mut DAT_100a40a8: u32;
    static mut DAT_100a40ac: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100021b0() {
    mech_trace::enter!("100021b0");
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the indexed read stays relative to the game's table at DAT_100c4c78.
    unsafe {
        mech_trace::gw!("100021b0", "100c4c88", DAT_100c4c88, 1);
        mech_trace::gw!(
            "100021b0",
            "100c4c84",
            DAT_100c4c84,
            (mech_trace::gr!("100021b0", "100a40b4", DAT_100a40b4) / 0x18) as u32
        );
        mech_trace::gw!(
            "100021b0",
            "100c4c78",
            DAT_100c4c78,
            mech_trace::gr!("100021b0", "100c4c84", DAT_100c4c84).wrapping_mul(5)
        );
        mech_trace::gw!(
            "100021b0",
            "100c4c7c",
            DAT_100c4c7c,
            mech_trace::gr!("100021b0", "100c4c84", DAT_100c4c84).wrapping_mul(7)
        );
        mech_trace::gw!(
            "100021b0",
            "100c4c80",
            DAT_100c4c80,
            mech_trace::gr!("100021b0", "100c4c84", DAT_100c4c84).wrapping_mul(0x11)
        );
        mech_trace::gw!(
            "100021b0",
            "100c4c84",
            DAT_100c4c84,
            mech_trace::gr!("100021b0", "100c4c84", DAT_100c4c84).wrapping_mul(0x13)
        );
        if mech_trace::gr!("100021b0", "100a40a0", DAT_100a40a0) != -1 {
            let off = (mech_trace::gr!("100021b0", "100a40a0", DAT_100a40a0) as u32).wrapping_mul(4)
                as usize;
            mech_trace::gw!(
                "100021b0",
                "100a40c0",
                DAT_100a40c0,
                mech_trace::rd!(
                    "100021b0",
                    (&raw mut DAT_100c4c78)
                        .cast::<u8>()
                        .wrapping_add(off)
                        .cast::<u32>(),
                    read_unaligned
                )
            );
        }
        mech_trace::gw!(
            "100021b0",
            "100a40c4",
            DAT_100a40c4,
            mech_trace::gr!("100021b0", "100a40c0", DAT_100a40c0)
        );
        mech_trace::gw!("100021b0", "100a40a0", DAT_100a40a0, -1);
        mech_trace::gw!(
            "100021b0",
            "100a40ac",
            DAT_100a40ac,
            mech_trace::cr!(
                "100021b0",
                "10002570",
                FUN_10002570(mech_trace::gr!("100021b0", "100a40a8", DAT_100a40a8), 0x794)
            )
        );
    }
}
