// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10069984: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069990(param_1: *mut i16, param_2: i32) -> *mut i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069bca(param_1: i32) -> i32 {
    // SAFETY: the callee and the global are the game's own; the returned
    // pointer addresses two consecutive shorts, read as the original does.
    unsafe {
        let p = FUN_10069990(&raw mut DAT_10069984, param_1);
        let lo = u32::from(p.read_unaligned() as u16);
        let hi = u32::from(p.add(1).read_unaligned() as u16);
        (lo | (hi << 16)) as i32
    }
}
