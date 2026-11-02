// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100463c4() {
    mech_trace::enter!("100463c4");
    let mut stop_params: MaybeUninit<u32> = MaybeUninit::uninit();
    // SAFETY: the global and the MCI import are the game's own; the
    // parameter block lives on this stack frame for the whole call.
    unsafe {
        mech_trace::cr!(
            "100463c4",
            "mciSendCommandA",
            mciSendCommandA(
                mech_trace::gr!("100463c4", "100b04b0", DAT_100b04b0),
                0x808,
                0,
                stop_params.as_mut_ptr() as usize as u32,
            )
        );
    }
}
