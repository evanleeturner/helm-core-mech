// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a1148: f64;
}

fn fabsf(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffff)
}

fn ftol_low(x: f32) -> i32 {
    // x87 fistp yields the integer indefinite (0x8000000000000000) on overflow or NaN.
    if x >= -9223372036854775808.0_f32 && x < 9223372036854775808.0_f32 {
        (x as i64) as u32 as i32
    } else {
        0
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032959(
    param_1: f32,
    param_2: f32,
    param_3: f32,
    param_4: *mut i32,
    param_5: *mut i32,
    param_6: *mut i32,
) {
    mech_trace::enter!(
        "10032959", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the global is the game's own; the output pointers are the
    // caller's and written only where the original writes them.
    unsafe {
        let threshold = mech_trace::gr!("10032959", "100a1148", DAT_100a1148) as f32;
        let sum = fabsf(param_3) + (fabsf(param_2) + fabsf(param_1));
        if threshold < sum {
            mech_trace::wr!("10032959", param_4, write_unaligned, ftol_low(param_1));
            mech_trace::wr!("10032959", param_5, write_unaligned, ftol_low(param_2));
            mech_trace::wr!("10032959", param_6, write_unaligned, ftol_low(param_3));
        }
    }
}
