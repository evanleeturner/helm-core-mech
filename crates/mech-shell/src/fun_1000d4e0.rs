// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6798: u32;
    static mut DAT_100a679c: u32;
    static mut DAT_1007f068: f64;
    static mut DAT_10083508: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d440() -> i32;
    fn FUN_10018060(param_1: u32, param_2: i32) -> i32;
    fn FUN_100507a0(param_1: i32) -> i32;
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d4e0() -> u32 {
    // SAFETY: the globals and callees are the game's own; the pointers held
    // in the globals point at the game's live structures, read where the
    // original reads them.
    unsafe {
        let mut total: u32 = DAT_100a6798;
        if ((total.wrapping_add(4)) as *const i32).read_unaligned() == 0 {
            let mut min_scaled: i32 = 100000000;
            let addend = FUN_1000d440();
            let mut index: i32 = 0;
            if 0 < ((DAT_100a679c.wrapping_add(4)) as *const i32).read_unaligned() {
                loop {
                    let item = FUN_10018060(DAT_100a679c, index);
                    let value = FUN_100507a0(item);
                    if (value as f64) * DAT_1007f068 < (min_scaled as f64) {
                        FUN_100507a0(item);
                        let scaled = __ftol();
                        min_scaled = scaled as i32;
                    }
                    index = index.wrapping_add(1);
                    if !(index < ((DAT_100a679c.wrapping_add(4)) as *const i32).read_unaligned()) {
                        break;
                    }
                }
            }
            let base = ((DAT_10083508.wrapping_add(0x12a)) as *const i32).read_unaligned();
            total = base.wrapping_add(addend) as u32;
            if (total as i32) < min_scaled {
                return total & 0xffffff00;
            }
        }
        (total >> 8) | 1
    }
}
