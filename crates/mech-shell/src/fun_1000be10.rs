// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut u8;
}

/// Hands out the next number from the counter at offset 0x15e of the
/// structure `DAT_10083508` points at: returns the count before the add, as
/// the original returns it in `EAX`, and leaves the counter one higher. Its
/// callers keep the number as an id (`10041b10`, `1000ae80`).
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000be10() -> i32 {
    mech_trace::enter!("1000be10");
    // SAFETY: DAT_10083508 is the game's own pointer to a structure; the
    // counter at offset 0x15e is read and written where the original does.
    let before = unsafe {
        let p = mech_trace::gr!("1000be10", "10083508", DAT_10083508)
            .byte_add(0x15e)
            .cast::<i32>();
        let before = mech_trace::rd!("1000be10", p, read_unaligned);
        mech_trace::wr!("1000be10", p, write_unaligned, before.wrapping_add(1));
        before
    };
    mech_trace::rt!("1000be10", before)
}
