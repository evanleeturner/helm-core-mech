// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fb55(param_1: *mut i32) -> u8;
    fn FUN_1003d8af(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fbc0(param_1: *mut i32) {
    // SAFETY: the callees and the global are the game's own; param_1 points
    // at the caller's structure, read where the original does.
    unsafe {
        FUN_1002fb55(param_1);
        if DAT_100aefc0 != 0 && param_1.read_unaligned() == 0 {
            let p = param_1.add(0x1b).read_unaligned() as usize as *mut u16;
            FUN_1003d8af(p);
        }
    }
}
