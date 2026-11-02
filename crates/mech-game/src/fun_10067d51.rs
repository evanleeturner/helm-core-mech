// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067d51(param_1: *mut i32) -> i32 {
    // SAFETY: param_1 points at the caller's struct of at least 9 bytes:
    // two i32 values followed by a shift count byte at offset 8.
    unsafe {
        let a = param_1.read_unaligned();
        let b = param_1.byte_add(4).read_unaligned();
        let shift = param_1.byte_add(8).cast::<u8>().read() & 0x1f;
        let delta = (b as u32).wrapping_sub(a as u32) as i32;
        let delta = delta >> shift;
        let a2 = param_1.read_unaligned();
        let result = (a2 as u32).wrapping_add(delta as u32) as i32;
        param_1.write_unaligned(result);
        param_1.read_unaligned()
    }
}
