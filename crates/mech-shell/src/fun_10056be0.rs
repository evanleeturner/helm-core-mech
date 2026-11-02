// SPDX-License-Identifier: MIT

/// 1/32767 as a double, the original's constant at `0x1007f1d0`.
const RAND_SCALE: f64 = f64::from_bits(0x3f00_0020_0040_0080);

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "_rand"]
    fn _rand() -> i32;
}

/// A random whole number from `lo` to `hi`: `lo + _rand() * RAND_SCALE *
/// (hi + 1 - lo)`, cut toward zero as the original's `__ftol` cuts. cdecl,
/// the caller pops; Ghidra stored it as `(void)`, its assembly reads both
/// arguments. Each step rounds to a double, which equals the x87's result
/// only under its 53-bit precision, the Windows default.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056be0(lo: i32, hi: i32) -> i64 {
    mech_trace::enter!("10056be0", lo, hi);
    // SAFETY: `_rand` is the game's own runtime routine; it takes no arguments.
    let roll = unsafe { mech_trace::cr!("10056be0", "_rand", _rand()) };
    let span = hi.wrapping_add(1).wrapping_sub(lo);
    mech_trace::rt!(
        "10056be0",
        (f64::from(lo) + f64::from(roll) * RAND_SCALE * f64::from(span)) as i64
    )
}
