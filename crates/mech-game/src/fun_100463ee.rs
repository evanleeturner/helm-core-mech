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
pub unsafe extern "C" fn FUN_100463ee(from_position: u32, to_position: i32) {
    mech_trace::enter!("100463ee", from_position, to_position);
    let mut play_params: MaybeUninit<[u32; 3]> = MaybeUninit::uninit();
    let p = play_params.as_mut_ptr().cast::<u32>();
    let mut play_flags: u32 = 4;
    // SAFETY: p points at a local three-word buffer; the globals and the
    // MCI call are the game's own.
    unsafe {
        mech_trace::wr!("100463ee", p.add(1), write, from_position);
        if to_position != 0 {
            play_flags = 0xc;
            mech_trace::wr!("100463ee", p.add(2), write, to_position as u32);
        }
        mech_trace::cr!(
            "100463ee",
            "mciSendCommandA",
            mciSendCommandA(
                mech_trace::gr!("100463ee", "100b04b0", DAT_100b04b0),
                0x806,
                play_flags,
                p as u32
            )
        );
    }
}
