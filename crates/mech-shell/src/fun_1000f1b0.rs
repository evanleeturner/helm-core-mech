// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008da60: i32;
    static mut DAT_1008da9c: i32;
    static mut DAT_10098204: i32;
    static mut DAT_100a672c: i32;
    static mut DAT_100a7248: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _SmackGoto(param_1: i32, param_2: i32);
    fn FUN_10012fc0(param_1: i32);
    fn _SmackDoFrame(param_1: i32);
    fn _SmackNextFrame(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000f1b0() {
    // SAFETY: the callees and globals are the game's own; the function pointer
    // is read from the game's table at DAT_100a7248 + 0x20 as the original does.
    unsafe {
        _SmackGoto(DAT_1008da60, DAT_1008da9c);
        FUN_10012fc0(DAT_100a672c);
        let mut i_var1: i32 = -1;
        if DAT_10098204 != 0 {
            let f = ((DAT_100a7248 as u32).wrapping_add(0x20) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f as usize);
            i_var1 = f();
        }
        if i_var1 != 0 {
            _SmackDoFrame(DAT_1008da60);
        }
        _SmackNextFrame(DAT_1008da60);
        DAT_1008da9c = DAT_1008da9c.wrapping_add(1);
    }
}
