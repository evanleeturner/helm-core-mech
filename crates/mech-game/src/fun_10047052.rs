// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn auxSetVolume(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047052(volume: u32) -> u32 {
    mech_trace::enter!("10047052", volume);
    let mut volume = volume;
    if 0xfffe < (volume as i32) {
        volume = 0xffff;
    }
    // SAFETY: the global and the winmm import are the game's own; the device
    // id is read once and passed through unchanged.
    unsafe {
        if mech_trace::gr!("10047052", "100b04b4", DAT_100b04b4) != 0xffff_ffff {
            let r = mech_trace::cr!(
                "10047052",
                "auxSetVolume",
                auxSetVolume(
                    mech_trace::gr!("10047052", "100b04b4", DAT_100b04b4),
                    (volume & 0xffff) | volume.wrapping_shl(16)
                )
            );
            if r == 0 {
                return mech_trace::rt!("10047052", 1);
            }
        }
    }
    mech_trace::rt!("10047052", 0)
}
