// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5534: u32;
    static mut DAT_100a5538: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007dc0(param_1: *mut u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_stop_sample_4(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10007e2a() {
    // SAFETY: the globals and callees are the game's own; the queue head is
    // read from the structure DAT_100a5534 points at, as the original does.
    unsafe {
        if DAT_100a5534 != 0 {
            let mut local_8: u32 =
                ((DAT_100a5534.wrapping_add(0x4f)) as *const u32).read_unaligned();
            while local_8 != 0 {
                local_8 = FUN_10007dc0(local_8 as *mut u32);
            }
        }
        if DAT_100a5538 != 0 {
            _AIL_stop_sample_4(DAT_100a5538);
        }
    }
}
