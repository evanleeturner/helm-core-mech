// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7fd8: u32;
    static mut DAT_100c7fdc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10070276(param_1: u32, param_2: u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PeekMessageA(p1: *mut u32, p2: u32, p3: u32, p4: u32, p5: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10070219() -> u32 {
    // MSG layout: hwnd, message, wParam, lParam, time, pt.x, pt.y
    let mut local_20: [u32; 7] = [0; 7];
    // SAFETY: the callees and globals are the game's own; local_20 is a
    // valid MSG-sized buffer for PeekMessageA to fill.
    unsafe {
        let b_var1 = PeekMessageA(local_20.as_mut_ptr(), 0, 0x100, 0x108, 1);
        if b_var1 != 0 {
            FUN_10070276(local_20[1], local_20[2], local_20[3]);
        }
        DAT_100c7fdc = 0;
        DAT_100c7fd8 = 0;
    }
    0
}
