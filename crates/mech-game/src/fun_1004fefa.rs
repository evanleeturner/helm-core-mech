// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123240: u8;
    static mut DAT_10123369: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004fefa(param_1: i32) -> u32 {
    mech_trace::enter!("1004fefa", param_1);
    let stride = (param_1 as u32).wrapping_mul(0x3c0a) as usize;
    // SAFETY: the globals are the game's own memory; the block and its
    // records are read unaligned at the offsets the original uses.
    unsafe {
        let base = (&raw mut DAT_10123240) as *mut u8;
        let p1 = base.wrapping_add(stride);
        let mut local_8: i32 = 0;
        loop {
            let count = mech_trace::rd!("1004fefa", p1.cast::<i32>(), read_unaligned);
            if count <= local_8 {
                let src = ((&raw mut DAT_10123369) as *mut u8).wrapping_add(stride);
                let val = mech_trace::rd!("1004fefa", src.cast::<u16>(), read_unaligned);
                return mech_trace::rt!(
                    "1004fefa",
                    ((p1 as usize as u32) & 0xffff0000) | u32::from(val)
                );
            }
            let off = (local_8 as u32).wrapping_mul(0x13f);
            let kind = mech_trace::rd!(
                "1004fefa",
                p1.wrapping_add(off.wrapping_add(0x3c) as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            if kind == 0x20 {
                break;
            }
            local_8 = local_8.wrapping_add(1);
        }
        let off = (local_8 as u32).wrapping_mul(0x13f);
        let val = mech_trace::rd!(
            "1004fefa",
            p1.wrapping_add(off.wrapping_add(0x129) as usize)
                .cast::<u16>(),
            read_unaligned
        );
        mech_trace::rt!("1004fefa", (off & 0xffff0000) | u32::from(val))
    }
}
