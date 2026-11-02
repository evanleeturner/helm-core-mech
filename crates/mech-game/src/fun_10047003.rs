// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn auxGetVolume(param_1: u32, param_2: *mut u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10047003() -> u32 {
    mech_trace::enter!("10047003");
    let mut local_8: u32 = 0;
    // SAFETY: the global is the game's own memory; auxGetVolume is the system
    // import and writes through a pointer to a valid local.
    unsafe {
        if mech_trace::gr!("10047003", "100b04b4", DAT_100b04b4) != 0xffff_ffff {
            let m_var1 = mech_trace::cr!(
                "10047003",
                "auxGetVolume",
                auxGetVolume(
                    mech_trace::gr!("10047003", "100b04b4", DAT_100b04b4),
                    &raw mut local_8
                )
            );
            if m_var1 == 0 {
                return mech_trace::rt!("10047003", local_8 & 0xffff);
            }
        }
    }
    mech_trace::rt!("10047003", 0xffff_ffff)
}
