// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f64bc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d671(param_1: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000dd06(param_1: i32) {
    let i_var1 = param_1.wrapping_mul(0x7c);
    // SAFETY: DAT_101f64bc is the game's own table; the entry at the byte
    // offset holds a pointer that is passed to the game's free routine and
    // then cleared, as the original does.
    unsafe {
        let entry = (&raw mut DAT_101f64bc).byte_offset(i_var1 as isize);
        if entry.cast::<i32>().read_unaligned() != 0 {
            FUN_1001d671(entry.cast::<u32>().read_unaligned() as usize as *mut u8);
            entry.cast::<u32>().write_unaligned(0);
        }
    }
}
