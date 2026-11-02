// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160be0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004986d() -> u32 {
    mech_trace::enter!("1004986d");
    // SAFETY: the global is the game's own; each node is a game-owned
    // linked-list record read at the offsets the original uses.
    unsafe {
        let mut local_c: i32 = mech_trace::gr!("1004986d", "10160be0", DAT_10160be0) as i32;
        loop {
            if local_c == 0 {
                return mech_trace::rt!("1004986d", 0);
            }
            if mech_trace::rd!(
                "1004986d",
                ((local_c as usize + 4) as *const i32),
                read_unaligned
            ) == 1
            {
                break;
            }
            local_c = mech_trace::rd!(
                "1004986d",
                ((local_c as usize + 0x14) as *const i32),
                read_unaligned
            );
        }
        mech_trace::rt!(
            "1004986d",
            mech_trace::rd!(
                "1004986d",
                ((local_c as usize + 0xc) as *const u32),
                read_unaligned
            )
        )
    }
}
