// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edf4: u32;
    static mut DAT_1008edf8: Option<unsafe extern "C" fn(u32, *mut u8, *mut u8, u32)>;
    static mut DAT_1008ee00: u32;
    static mut DAT_100a6734: u32;
    static mut DAT_100b0f40: u32;
    static mut DAT_100b0f44: [u8; 0];
    static mut DAT_100b0f50: [u8; 0];
    static mut s_UnregisterAnimFunction__pointer_m_1008f140: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10016bd0() {
    mech_trace::enter!("10016bd0");
    // SAFETY: the globals and the callees are the game's own memory and code;
    // the stored callback is called with the arguments the original passes.
    unsafe {
        let pc_var1 = mech_trace::gr!("10016bd0", "1008edf8", DAT_1008edf8);
        if let Some(f) = pc_var1 {
            mech_trace::gw!("10016bd0", "1008edf4", DAT_1008edf4, f as usize as u32);
            mech_trace::gw!("10016bd0", "1008edf8", DAT_1008edf8, None);
            mech_trace::gw!("10016bd0", "1008ee00", DAT_1008ee00, 0);
            mech_trace::cr!(
                "10016bd0",
                "ind",
                f(
                    mech_trace::gr!("10016bd0", "100a6734", DAT_100a6734),
                    (&raw mut DAT_100b0f50).cast::<u8>(),
                    (&raw mut DAT_100b0f44).cast::<u8>(),
                    mech_trace::gr!("10016bd0", "100b0f40", DAT_100b0f40),
                )
            );
            return;
        }
        mech_trace::cr!(
            "10016bd0",
            "100640a0",
            FUN_100640a0((&raw mut s_UnregisterAnimFunction__pointer_m_1008f140).cast::<i8>())
        );
    }
}
