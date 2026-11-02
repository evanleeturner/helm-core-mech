// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a446(param_1: i32, param_2: i16) -> u32 {
    let mut local_8: i32 = 0;
    loop {
        if 2 < local_8 {
            return 0;
        }
        // SAFETY: param_1 is the address of a game structure whose pointer
        // table at +0x148 holds null or pointers to u16 count-prefixed records.
        unsafe {
            let p = (param_1
                .wrapping_add(0x148)
                .wrapping_add(local_8.wrapping_mul(4)) as *const *const u16)
                .read_unaligned();
            if !p.is_null() {
                let mut local_10: i32 = 0;
                while local_10 < p.read_unaligned() as i32 {
                    let v = p
                        .add((local_10.wrapping_mul(2).wrapping_add(1)) as usize)
                        .read_unaligned();
                    if v as u32 == param_2 as i32 as u32 {
                        return 1;
                    }
                    local_10 = local_10.wrapping_add(1);
                }
            }
        }
        local_8 = local_8.wrapping_add(1);
    }
}
