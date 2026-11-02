// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a1148: f64;
}

fn ftol_low(x: f32) -> i32 {
    // x87 fistp yields the "integer indefinite" 0x8000000000000000 for NaN and
    // out-of-range values, whose low 32 bits are zero.
    if x.is_nan() || x.abs() >= 9223372036854775808.0f32 {
        0
    } else {
        (x as i64) as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100329cf(
    param_1: f32,
    param_2: f32,
    param_3: f32,
    param_4: f32,
    param_5: *mut i32,
    param_6: *mut i32,
    param_7: *mut i32,
    param_8: *mut i32,
) {
    mech_trace::enter!(
        "100329cf", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8
    );
    // SAFETY: the global is the game's own memory; the output pointers are
    // supplied by the caller and written where the original does.
    unsafe {
        let limit = mech_trace::gr!("100329cf", "100a1148", DAT_100a1148) as f32;
        if limit < param_4.abs() + (param_3.abs() + (param_2.abs() + param_1.abs())) {
            mech_trace::wr!("100329cf", param_5, write_unaligned, ftol_low(param_1));
            mech_trace::wr!("100329cf", param_6, write_unaligned, ftol_low(param_2));
            mech_trace::wr!("100329cf", param_7, write_unaligned, ftol_low(param_3));
            mech_trace::wr!("100329cf", param_8, write_unaligned, ftol_low(param_4));
        }
    }
}
