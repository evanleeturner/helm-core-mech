// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
    fn FUN_1004652f() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10046659() {
    let mut local_10 = [0u8; 12];
    // SAFETY: the global and the callees are the game's own; local_10 is a
    // live buffer whose address is passed as the MCI parameter block.
    unsafe {
        let m_var1 = mciSendCommandA(DAT_100b04b0, 0x80d, 0x100, local_10.as_mut_ptr() as u32);
        if m_var1 != 0 {
            FUN_1004652f();
        }
    }
}
