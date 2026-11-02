// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007074a(param_1: i32) {
    // SAFETY: param_1 is an address of the caller's record; the fields are
    // read and written at the same offsets as the original, and the global
    // is the game's own.
    unsafe {
        let base = param_1 as usize as *mut u8;
        if base.byte_add(4).cast::<i32>().read_unaligned() != DAT_100adf38 {
            base.byte_add(0x90).cast::<u32>().write_unaligned(0x11a);
        }
        base.byte_add(0x8c).cast::<u32>().write_unaligned(0);
        base.byte_add(0x88)
            .cast::<u32>()
            .write_unaligned(0xffff_ffff);
    }
}
