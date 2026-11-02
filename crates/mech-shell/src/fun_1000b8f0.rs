// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0424: u8;
    static mut DAT_100b043c: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000b8f0(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: *mut i32,
) -> u32 {
    let _ = param_1;
    let mut position: i32 = 0;
    let off = param_3.wrapping_mul(0x14) as isize;
    // SAFETY: the globals are the game's own tables; addresses are only
    // computed from their base, and reads happen where the original reads.
    unsafe {
        let limit = (&raw mut DAT_100b043c).wrapping_offset(off);
        if 0 < (limit.read_unaligned() as i16) {
            let mut entry_ptr = (&raw mut DAT_100b0424).wrapping_offset(off) as *const u16;
            loop {
                if u32::from(entry_ptr.read_unaligned()) == param_2 {
                    if !param_4.is_null() {
                        param_4.write_unaligned(position);
                    }
                    return 1;
                }
                entry_ptr = entry_ptr.wrapping_add(1);
                position = position.wrapping_add(1);
                if !(position < (limit.read_unaligned() as i16 as i32)) {
                    break;
                }
            }
        }
    }
    0
}
