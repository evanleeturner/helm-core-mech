// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100c7fc8: u16;
    static mut _DAT_100c7fca: u16;
    static mut _DAT_100c7fcc: u32;
    static mut _DAT_100c7fd0: u32;
    static mut _DAT_100c7fd4: u16;
    static mut _DAT_100c7fd6: u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_waveOutOpen(
        h_wave_out: *mut u32,
        param_2: u32,
        param_3: u32,
        lp_format: *mut u16,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006aec6() -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the globals and the callee are the game's own; local_8 is a
    // valid stack slot for the callee to fill in.
    unsafe {
        _DAT_100c7fc8 = 1;
        _DAT_100c7fca = 2;
        _DAT_100c7fcc = 0x2b11;
        _DAT_100c7fd0 = 0x5622;
        _DAT_100c7fd4 = 2;
        _DAT_100c7fd6 = 8;
        let i_var1 = _AIL_waveOutOpen(&mut local_8, 0, 0, &raw mut _DAT_100c7fc8);
        if i_var1 != 0 {
            local_8 = 0;
        }
    }
    local_8
}
