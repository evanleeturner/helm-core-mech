// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008699d(param_1: *mut u32, param_2: u8, param_3: u32) -> *mut u32 {
    mech_trace::enter!("1008699d", param_1, param_2, param_3);
    let mut p = param_1;
    let b = u32::from(param_2);
    let word = b | b.wrapping_shl(8) | b.wrapping_shl(16) | b.wrapping_shl(24);
    let mut n = param_3 >> 2;
    // SAFETY: the caller guarantees param_1 points at param_3 writable bytes.
    unsafe {
        while n != 0 {
            mech_trace::wr!("1008699d", p, write_unaligned, word);
            p = p.add(1);
            n -= 1;
        }
        let mut bp = p.cast::<u8>();
        let mut m = param_3 & 3;
        while m != 0 {
            mech_trace::wr!("1008699d", bp, write, param_2);
            bp = bp.add(1);
            m -= 1;
        }
    }
    mech_trace::rt!("1008699d", param_1)
}
