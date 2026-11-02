// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4d68: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10071674(param_1: *mut u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007153a(param_1: *mut i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 is non-null and points at the caller's int; the inner
    // pointer is the game's own structure, read where the original reads it.
    unsafe {
        let base = param_1.read_unaligned();
        if base == 0 {
            return;
        }
        let i_var1 = (base.wrapping_add(4) as usize as *const i32).read_unaligned();
        if i_var1 < 0x3d {
            let arg = (base.wrapping_add(0x40) as usize as *const u32).read_unaligned();
            let pu_var2 = FUN_10071674(arg as usize as *mut u32);
            let dst = (&raw mut DAT_100b4d68)
                .cast::<u8>()
                .byte_offset(i_var1.wrapping_mul(4) as isize);
            dst.cast::<u32>().write_unaligned(pu_var2 as usize as u32);
        }
    }
}
