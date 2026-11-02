// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160be0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004986d() -> u32 {
    // SAFETY: the global is the game's own; each node is a game-owned
    // linked-list record read at the offsets the original uses.
    unsafe {
        let mut local_c: i32 = DAT_10160be0 as i32;
        loop {
            if local_c == 0 {
                return 0;
            }
            if ((local_c as usize + 4) as *const i32).read_unaligned() == 1 {
                break;
            }
            local_c = ((local_c as usize + 0x14) as *const i32).read_unaligned();
        }
        ((local_c as usize + 0xc) as *const u32).read_unaligned()
    }
}
