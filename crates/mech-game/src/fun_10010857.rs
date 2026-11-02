// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101ec430: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10010857(param_1: i32) {
    if (-1 < param_1) || (param_1 < 7) {
        let off = (param_1 as u32).wrapping_mul(4) as usize;
        // SAFETY: the table is the game's own memory; each entry is a pointer
        // to a block that the original writes zero into at offsets 0 and 0xc.
        unsafe {
            let base = (&raw mut DAT_101ec430).cast::<u8>();
            let p = base.byte_add(off).cast::<*mut u32>().read_unaligned();
            p.write_unaligned(0);
            let q = base.byte_add(off).cast::<u32>().read_unaligned();
            (q.wrapping_add(0xc) as *mut u32).write_unaligned(0);
        }
    }
}
