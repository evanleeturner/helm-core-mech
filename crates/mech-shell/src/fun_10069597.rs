// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069597(param_1: i32, param_2: i32, param_3: *mut u32) {
    mech_trace::enter!("10069597", param_1, param_2, param_3);
    let prod: u64 = (param_1 as i64 as u64).wrapping_mul(param_2 as i64 as u64);
    let lo: u32 = prod as u32;
    let rounded_hi: u16 = (lo.wrapping_add(0x8000) >> 16) as u16;
    let hi16: u16 = ((prod >> 32) as u16).wrapping_add((0xffff7fff_u32 < lo) as u16);
    let value: u32 = (rounded_hi as u32) | ((hi16 as u32) << 16);
    // SAFETY: param_3 points at a writable u32 supplied by the caller.
    unsafe {
        mech_trace::wr!("10069597", param_3, write_unaligned, value);
    }
}
