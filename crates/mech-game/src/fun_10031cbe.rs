// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031cbe(param_1: *mut i32) {
    let mut local_8: i32 = 7;
    let mut local_10: i32 = 0;
    while local_10 < 3 {
        // SAFETY: param_1 points at the caller's two ints; the table at
        // 0x100ada78 and the store at 0x101686f0 are the game's own memory.
        unsafe {
            let p1 = param_1.byte_add(4).read_unaligned();
            let p0 = param_1.read_unaligned();
            let off = (local_10 as usize).wrapping_mul(8);
            let t2 = ((0x100ada7cusize.wrapping_add(off)) as *const i32).read_unaligned();
            let t1 = ((0x100ada78usize.wrapping_add(off)) as *const i32).read_unaligned();
            let u_var2 = (t2 as u32).wrapping_sub(p1.wrapping_sub(1) as u32);
            let u_var4 = ((u_var2 as i32) >> 0x1f) as u32;
            let u_var1 = (t1 as u32).wrapping_sub(p0.wrapping_sub(1) as u32);
            let u_var5 = ((u_var1 as i32) >> 0x1f) as u32;
            let i_var3 = ((u_var2 ^ u_var4)
                .wrapping_sub(u_var4)
                .wrapping_sub(u_var5)
                .wrapping_add(u_var1 ^ u_var5)) as i32;
            if i_var3 < local_8 {
                (0x101686f0usize as *mut i32).write_unaligned(local_10);
                local_8 = i_var3;
            }
        }
        local_10 = local_10.wrapping_add(1);
    }
}
