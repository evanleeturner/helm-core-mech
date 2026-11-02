// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003eb02(param_1: u32, param_2: *mut i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005c2d0() {
    mech_trace::enter!("1005c2d0");
    let local_28: [u32; 8] = [0xca, 0x10c, 0xed, 0x105, 0x10b, 0xe0, 0xdf, 0x113];
    let mut i: u32 = 0;
    while i < 8 {
        // SAFETY: the callee and the global are the game's own; the global
        // holds a pointer value passed through unchanged.
        unsafe {
            mech_trace::cr!(
                "1005c2d0",
                "1003eb02",
                FUN_1003eb02(
                    local_28[i as usize],
                    mech_trace::gr!("1005c2d0", "100af0a8", DAT_100af0a8) as *mut i32
                )
            );
        }
        i = i.wrapping_add(1);
    }
}
