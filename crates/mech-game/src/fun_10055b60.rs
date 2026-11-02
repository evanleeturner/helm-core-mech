// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055b60(param_1: i32, param_2: i32, param_3: i32, shift: u8) -> u32 {
    mech_trace::enter!("10055b60", param_1, param_2, param_3, shift);
    let diff = (param_1 as u32)
        .wrapping_sub(param_2 as u32)
        .wrapping_mul(4) as i32;
    let prod = (diff as i64 as u64).wrapping_mul(param_3 as i64 as u64);
    let lo = prod as u32;
    let hi = (prod >> 32) as u32;
    let n = (shift & 0x1f) as u32;

    if n == 0 {
        return mech_trace::rt!("10055b60", lo);
    }
    mech_trace::rt!("10055b60", (lo >> n) | hi.wrapping_shl(0x20 - n))
}
