// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafd4: unsafe extern "C" fn(i32, *mut u32, u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10019c6e(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    param_8: u32,
    param_9: u32,
    param_10: u32,
    param_11: u32,
) {
    mech_trace::enter!(
        "10019c6e", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8,
        param_9, param_10, param_11
    );
    let mut local_7c = MaybeUninit::<[u32; 30]>::uninit();
    let p = local_7c.as_mut_ptr().cast::<u32>();
    // SAFETY: p points at a 30-word local buffer, every index written is in
    // range; the callback global is the game's own.
    unsafe {
        mech_trace::wr!("10019c6e", p.add(3), write, param_1);
        mech_trace::wr!("10019c6e", p.add(0), write, param_1);
        mech_trace::wr!("10019c6e", p.add(4), write, param_2);
        mech_trace::wr!("10019c6e", p.add(1), write, param_2);
        mech_trace::wr!("10019c6e", p.add(9), write, param_3);
        mech_trace::wr!("10019c6e", p.add(6), write, param_3);
        mech_trace::wr!("10019c6e", p.add(10), write, param_4);
        mech_trace::wr!("10019c6e", p.add(7), write, param_4);
        mech_trace::wr!("10019c6e", p.add(15), write, param_5);
        mech_trace::wr!("10019c6e", p.add(12), write, param_5);
        mech_trace::wr!("10019c6e", p.add(16), write, param_6);
        mech_trace::wr!("10019c6e", p.add(13), write, param_6);
        mech_trace::wr!("10019c6e", p.add(21), write, param_7);
        mech_trace::wr!("10019c6e", p.add(18), write, param_7);
        mech_trace::wr!("10019c6e", p.add(22), write, param_8);
        mech_trace::wr!("10019c6e", p.add(19), write, param_8);
        mech_trace::wr!("10019c6e", p.add(27), write, param_9);
        mech_trace::wr!("10019c6e", p.add(24), write, param_9);
        mech_trace::wr!("10019c6e", p.add(28), write, param_10);
        mech_trace::wr!("10019c6e", p.add(25), write, param_10);
        let f = mech_trace::gr!("10019c6e", "100aafd4", DAT_100aafd4);
        mech_trace::cr!("10019c6e", "ind", f(5, p, param_11));
    }
}
