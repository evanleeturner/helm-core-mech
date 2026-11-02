// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001ca17(param_1: *mut u16) -> u16;
    fn FUN_1001c9e8(param_1: *mut u16, param_2: u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100300e2(param_1: i32, param_2: i32, param_3: u32) {
    // SAFETY: param_1 points at the game's own object; its fields and the
    // linked list nodes are read where the original reads them, and the
    // callees are the game's own.
    unsafe {
        let u_var1 = FUN_1001ca17(
            ((param_1 as usize).wrapping_add(0x6c) as *const i32).read_unaligned() as *mut u16,
        );
        if param_2 > 0 && param_2 < 0x10 && (((u_var1 & 0xf0) >> 4) as i32) < param_2 {
            let p = ((param_1 as usize).wrapping_add(0x6c) as *const i32).read_unaligned();
            if p != 0 {
                let v = (((p as usize).wrapping_add(0x16)) as *const u16).read_unaligned();
                if u32::from(v) == param_3 {
                    let p2 = ((param_1 as usize).wrapping_add(0x6c) as *const i32).read_unaligned();
                    FUN_1001c9e8(p2 as *mut u16, param_2.wrapping_shl(4) as u16);
                }
            }
            let mut local_8 = ((param_1 as usize).wrapping_add(4) as *const i32).read_unaligned();
            while local_8 != 0 {
                FUN_100300e2(local_8, param_2.wrapping_shl(4) >> 4, param_3);
                local_8 = ((local_8 as usize).wrapping_add(8) as *const i32).read_unaligned();
            }
        }
    }
}
