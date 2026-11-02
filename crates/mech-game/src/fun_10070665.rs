// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eeda0: [u32; 4];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10070665() {
    let mut local_8: i32 = 0;
    while local_8 < 4 {
        // SAFETY: DAT_100eeda0 is the game's own array of at least four
        // words; the index stays within 0..4.
        unsafe {
            (&raw mut DAT_100eeda0)
                .cast::<u32>()
                .add(local_8 as usize)
                .write(0);
        }
        local_8 = local_8.wrapping_add(1);
    }
}
