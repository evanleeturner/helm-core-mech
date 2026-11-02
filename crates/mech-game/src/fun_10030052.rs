// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_10030052(param_1: i32, param_2: u32) -> i32 {
    let mut param_1 = param_1;
    // SAFETY: param_1 points at the game's own node structure; the fields
    // are read where the original reads them, and child nodes are walked
    // through their sibling links.
    unsafe {
        let p6c = ((param_1 as usize).wrapping_add(0x6c) as *const i32).read_unaligned();
        if p6c == 0
            || u32::from(((p6c as usize).wrapping_add(0x16) as *const u16).read_unaligned())
                != param_2
        {
            let mut local_8 = ((param_1 as usize).wrapping_add(4) as *const i32).read_unaligned();
            while local_8 != 0 {
                let i_var1 = FUN_10030052(local_8, param_2);
                if i_var1 != 0 {
                    return i_var1;
                }
                local_8 = ((local_8 as usize).wrapping_add(8) as *const i32).read_unaligned();
            }
            param_1 = 0;
        }
        param_1
    }
}
