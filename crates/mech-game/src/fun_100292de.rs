// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100292de(param_1: i32) -> i32 {
    // SAFETY: param_1 is the address of the caller's structure; all reads and
    // writes are at the offsets the original uses.
    unsafe {
        let hi = ((param_1.wrapping_add(0x17a)) as *const u16).read_unaligned();
        let lo = ((param_1.wrapping_add(0x176)) as *const u16).read_unaligned();
        let local_8: i32 = ((hi as i32).wrapping_shl(16)) | (lo as i32);
        let p_idx = param_1.wrapping_add(0x16c) as *mut i16;
        let idx = p_idx.read_unaligned();
        if idx == 0 {
            (param_1.wrapping_add(0x154) as *mut i32).write_unaligned(local_8);
        } else {
            let off = (idx as u16 as i32).wrapping_mul(4);
            let addr = param_1.wrapping_add(0x154).wrapping_add(off);
            (addr as *mut i32).write_unaligned(local_8);
            let cur = p_idx.read_unaligned();
            p_idx.write_unaligned(cur.wrapping_add(1));
        }
        1
    }
}
