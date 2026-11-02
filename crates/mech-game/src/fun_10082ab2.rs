// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: u32;
    static mut DAT_100adf38: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10082ab2() {
    mech_trace::enter!("10082ab2");
    // SAFETY: the globals are the game's own; the table entry at the index is
    // a pointer to a game object whose field at offset 0xdc is read and written.
    unsafe {
        let base = (&raw mut DAT_100eef40).wrapping_add(mech_trace::gr!(
            "10082ab2",
            "100adf38",
            DAT_100adf38
        ) as usize);
        let addr =
            (mech_trace::rd!("10082ab2", base, read_unaligned)).wrapping_add(0xdc) as *mut u32;
        let base2 = (&raw mut DAT_100eef40).wrapping_add(mech_trace::gr!(
            "10082ab2",
            "100adf38",
            DAT_100adf38
        ) as usize);
        let addr2 =
            (mech_trace::rd!("10082ab2", base2, read_unaligned)).wrapping_add(0xdc) as *mut u32;
        let value = mech_trace::rd!("10082ab2", addr2, read_unaligned) | 0x1000;
        mech_trace::wr!("10082ab2", addr, write_unaligned, value);
    }
}
