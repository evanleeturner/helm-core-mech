// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a140(value: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a160(value: i32) {
    mech_trace::enter!("1000a160", value);
    // SAFETY: the callee is the game's own function; it takes the value by copy.
    unsafe {
        FUN_1000a140((value as u32).wrapping_add(0x32) as i32);
    }
}
