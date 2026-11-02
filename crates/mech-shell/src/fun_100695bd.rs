// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100694df(param_4: u32, param_2: *mut i32, param_3: *mut i32);
}

fn hi_term(l: i64, u: u32, lo: u16) -> i32 {
    let h = ((l as u64) >> 32) as i16 as i32;
    let c = (0xffff7fffu32 < u) as u16 as i32;
    h.wrapping_add(c).wrapping_shl(16) | (lo as i32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100695bd(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) {
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;

    // SAFETY: the callee is the game's own; the pointers are the caller's
    // two-element i32 arrays, read and written where the original does.
    unsafe {
        FUN_100694df(param_4, &raw mut local_8, &raw mut local_c);

        let mut l = (((*param_1).wrapping_sub(*param_3)).wrapping_mul(0x10000) as i64)
            .wrapping_mul(param_5 as i64);
        let mut i13 = (((l as u64) >> 32) as i32).wrapping_add((0xffff7fffu32 < (l as u32)) as i32);

        let l1 = (i13 as i64).wrapping_mul(local_8 as i64);
        let u5 = l1 as u32;
        let u9 = (u5.wrapping_add(0x8000) >> 16) as u16;

        let l2 = (i13 as i64).wrapping_mul(local_c as i64);
        let u6 = l2 as u32;
        let u10 = (u6.wrapping_add(0x8000) >> 16) as u16;

        l = (((*param_1.add(1)).wrapping_sub(*param_3.add(1))).wrapping_mul(0x10000) as i64)
            .wrapping_mul(param_6 as i64);
        i13 = (((l as u64) >> 32) as i32).wrapping_add((0xffff7fffu32 < (l as u32)) as i32);

        let l3 = (i13 as i64).wrapping_mul(local_8 as i64);
        let u7 = l3 as u32;
        let u11 = (u7.wrapping_add(0x8000) >> 16) as u16;

        let l4 = (i13 as i64).wrapping_mul(local_c as i64);
        let u8 = l4 as u32;
        let u12 = (u8.wrapping_add(0x8000) >> 16) as u16;

        let term1 = hi_term(l1, u5, u9);
        let term2 = hi_term(l4, u8, u12);
        let term3 = hi_term(l3, u7, u11);
        let term4 = hi_term(l2, u6, u10);

        *param_2 = term1.wrapping_sub(term2).wrapping_add(*param_3);
        *param_2.add(1) = term3.wrapping_add(term4).wrapping_add(*param_3.add(1));
    }
}
