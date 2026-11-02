// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fa2b(param_1: i32, param_2: u32) {
    // SAFETY: param_1 is the address of the game's node structure; fields are
    // read and written at the original's offsets, and the recursion walks the
    // node's child list exactly as the original does.
    unsafe {
        let p = ((param_1.wrapping_add(0x6c)) as *const i32).read_unaligned();
        if p != 0 {
            let q = p.wrapping_add(2) as *mut u16;
            let v = q.read_unaligned();
            q.write_unaligned(v & !(param_2 as u16));
        }
        let mut local_8 = ((param_1.wrapping_add(4)) as *const i32).read_unaligned();
        while local_8 != 0 {
            FUN_1002fa2b(local_8, param_2);
            local_8 = ((local_8.wrapping_add(8)) as *const i32).read_unaligned();
        }
    }
}
