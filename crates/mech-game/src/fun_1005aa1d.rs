// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005a93f(param_1: u32, param_2: *mut i32, param_3: *mut i32);
}

fn scale(diff: i32, factor: i32) -> i32 {
    let l = (diff.wrapping_mul(0x10000) as i64).wrapping_mul(factor as i64);
    let lo = l as u32;
    ((((l as u64) >> 32) as i32) as u32).wrapping_add((0xffff7fffu32 < lo) as u32) as i32
}

fn mul_round(a: i32, b: i32) -> u32 {
    let l = (a as i64).wrapping_mul(b as i64);
    let lo = l as u32;
    let hi = (((l as u64) >> 32) as u16).wrapping_add((0xffff7fffu32 < lo) as u16);
    ((lo.wrapping_add(0x8000) >> 16) as u16 as u32) | ((hi as u32) << 16)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005aa1d(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "1005aa1d", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the callee is the game's own; the pointers are the caller's
    // arrays of two ints, read and written where the original does.
    unsafe {
        FUN_1005a93f(param_4, &raw mut local_8, &raw mut local_c);

        let x = scale(
            mech_trace::rd!("1005aa1d", param_1, read_unaligned).wrapping_sub(mech_trace::rd!(
                "1005aa1d",
                param_3,
                read_unaligned
            )),
            param_5,
        );
        let a = mul_round(x, local_8);
        let b = mul_round(x, local_c);

        let y = scale(
            mech_trace::rd!("1005aa1d", param_1.add(1), read_unaligned)
                .wrapping_sub(mech_trace::rd!("1005aa1d", param_3.add(1), read_unaligned)),
            param_6,
        );
        let c = mul_round(y, local_8);
        let d = mul_round(y, local_c);

        mech_trace::wr!(
            "1005aa1d",
            param_2,
            write_unaligned,
            a.wrapping_sub(d)
                .wrapping_add(mech_trace::rd!("1005aa1d", param_3, read_unaligned) as u32)
                as i32
        );
        mech_trace::wr!(
            "1005aa1d",
            param_2.add(1),
            write_unaligned,
            c.wrapping_add(b).wrapping_add(mech_trace::rd!(
                "1005aa1d",
                param_3.add(1),
                read_unaligned
            ) as u32) as i32
        );
    }
}
