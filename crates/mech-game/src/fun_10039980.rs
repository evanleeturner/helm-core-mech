// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10039980(param_1: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 is the address of the game's structure; reads and
    // writes happen at the same offsets and in the same order as the original.
    unsafe {
        let idx = ((param_1.wrapping_add(0x754)) as *const i32).read_unaligned();
        if 0x2c < idx {
            return 0;
        }
        let v = ((param_1
            .wrapping_add(0x758)
            .wrapping_add(idx.wrapping_mul(4))) as *const i32)
            .read_unaligned();
        let result = v
            .wrapping_mul(0x24)
            .wrapping_add(param_1)
            .wrapping_add(0xfc);
        let cur = ((param_1.wrapping_add(0x754)) as *const i32).read_unaligned();
        ((param_1.wrapping_add(0x754)) as *mut i32).write_unaligned(cur.wrapping_add(1));
        result
    }
}
