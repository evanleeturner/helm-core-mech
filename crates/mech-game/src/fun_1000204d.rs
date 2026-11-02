// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123230: i32;
    static mut DAT_10168410: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000204d() {
    let mut local_c: i32 = 0;
    // SAFETY: the globals are the game's own memory; the array is read at
    // 4-byte entries indexed by local_c, as the original does.
    unsafe {
        while local_c < DAT_10123230 {
            let p = (&raw mut DAT_10168410)
                .cast::<u8>()
                .byte_add((local_c as u32).wrapping_mul(4) as usize);
            if p.cast::<i32>().read_unaligned() != 0 {
                let mut local_8: i32 = 0;
                while local_8 < 0x30 {
                    local_8 = local_8.wrapping_add(1);
                }
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
