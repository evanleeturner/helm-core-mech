// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a180(param_1: i32) -> *mut u32;
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100319b0(param_1: i32) {
    // SAFETY: param_1 is the address of the game's own structure; reads and
    // writes happen at the same offsets and order as the original, and the
    // callees are the game's own.
    unsafe {
        if ((param_1.wrapping_add(0x84f)) as *const i32).read_unaligned() == 1 {
            let p = (param_1.wrapping_add(0x278)) as *mut i32;
            p.write_unaligned(p.read_unaligned().wrapping_add(-0x32));
            FUN_1000a180(0x254f);
            let idx = ((param_1.wrapping_add(0x364)) as *const i32).read_unaligned();
            let pb = idx
                .wrapping_mul(0x12)
                .wrapping_add(0x771)
                .wrapping_add(param_1) as *mut u8;
            pb.write(pb.read() & 0xfe);
        }
        FUN_1000a890(param_1);
    }
}
