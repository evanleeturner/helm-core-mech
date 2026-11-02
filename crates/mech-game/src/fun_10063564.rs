// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063564(param_1: u32) {
    // SAFETY: the globals are the game's own; the pointers read from them
    // are the game's live structures, written where the original does.
    unsafe {
        let idx = DAT_100adf38 as usize;
        let base = (&raw const DAT_100eef40)
            .cast::<u32>()
            .add(idx)
            .read_unaligned();
        let p20 = ((base as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        let a = ((p20 as usize).wrapping_add(0x3e0) as *const i32).read_unaligned();
        let b = ((p20 as usize).wrapping_add(0x54) as *const i32).read_unaligned();
        let addr = a.wrapping_mul(0x40).wrapping_add(b).wrapping_add(0xc);
        (addr as usize as *mut u32).write_unaligned(param_1);
    }
}
