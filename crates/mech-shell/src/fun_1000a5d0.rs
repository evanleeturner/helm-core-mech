// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
    fn FUN_1000a120(record_base: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a5d0(param_1: i32) {
    let base = param_1 as *mut u8;
    // SAFETY: the callees are the game's own; param_1 is the address of a
    // record whose fields are read and written where the original does.
    unsafe {
        let truncated_float = __ftol();
        let mut computed = FUN_1000a120(truncated_float as i32);
        base.byte_add(0x220).cast::<i32>().write_unaligned(computed);
        base.byte_add(0x224).cast::<i32>().write_unaligned(300);
        let zero_mask: u32 = if base.byte_add(0x240).cast::<i32>().read_unaligned() == 0 {
            0xffff_ffff
        } else {
            0
        };
        let divisor = ((zero_mask & 0xffff_fff6).wrapping_add(0x14)) as i32;
        computed = base.byte_add(0x200).cast::<i32>().read_unaligned() / divisor;
        base.byte_add(0x244).cast::<i32>().write_unaligned(computed);
        computed = base
            .byte_add(0x238)
            .cast::<i32>()
            .read_unaligned()
            .wrapping_add(base.byte_add(0x22c).cast::<i32>().read_unaligned())
            .wrapping_add(base.byte_add(0x270).cast::<i32>().read_unaligned())
            .wrapping_add(base.byte_add(0x260).cast::<i32>().read_unaligned())
            .wrapping_add(base.byte_add(0x250).cast::<i32>().read_unaligned())
            .wrapping_add(base.byte_add(0x268).cast::<i32>().read_unaligned())
            .wrapping_add(base.byte_add(0x214).cast::<i32>().read_unaligned())
            .wrapping_add(base.byte_add(0x220).cast::<i32>().read_unaligned())
            .wrapping_add(300)
            .wrapping_add(computed);
        base.byte_add(0x204).cast::<i32>().write_unaligned(computed);
        let v = base
            .byte_add(0x200)
            .cast::<i32>()
            .read_unaligned()
            .wrapping_sub(computed);
        base.byte_add(0x208).cast::<i32>().write_unaligned(v);
    }
}
