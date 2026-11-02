// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ab38(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036317(param_1: i32) -> i32 {
    let mut local_14: i32 = -1;
    if param_1 < 8 {
        // SAFETY: the callee and the global table are the game's own; the
        // index comes from the game's lookup and the stored value is an
        // address of a game record read where the original reads it.
        unsafe {
            let i_var1 = FUN_1002ab38(param_1);
            if i_var1 != -1 {
                let base = (&raw mut DAT_100eef40).cast::<u32>();
                let entry = base.offset(i_var1 as isize).read_unaligned();
                if entry != 0xFFFF_FE90u32 {
                    local_14 = (entry.wrapping_add(0x176) as *const i16).read_unaligned() as i32;
                }
            }
        }
    }
    local_14.wrapping_add(1)
}
