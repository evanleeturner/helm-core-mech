// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100399ec(param_1: i32, param_2: i32) {
    if param_1 == 0 {
        return;
    }
    // SAFETY: param_1 is the caller's structure address; fields are read and
    // written at the same offsets as the original.
    unsafe {
        let base = param_1 as usize as *mut u8;
        let count = base.byte_add(0x754).cast::<i32>().read_unaligned();
        if 0 < count && -1 < param_2 {
            let limit = base.byte_add(0x750).cast::<i32>().read_unaligned();
            if param_2 < limit {
                let off = (param_2.wrapping_mul(0x24))
                    .wrapping_add(param_1)
                    .wrapping_add(0x100);
                (off as usize as *mut u32).write_unaligned(0xffff_ffff);
                let c = base.byte_add(0x754).cast::<i32>().read_unaligned();
                base.byte_add(0x754)
                    .cast::<i32>()
                    .write_unaligned(c.wrapping_sub(1));
                let idx = base.byte_add(0x754).cast::<i32>().read_unaligned();
                let addr = param_1
                    .wrapping_add(0x758)
                    .wrapping_add(idx.wrapping_mul(4));
                (addr as usize as *mut i32).write_unaligned(param_2);
            }
        }
    }
}
