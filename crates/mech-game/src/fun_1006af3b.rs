// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_midiOutOpen_12(param_1: u32, param_2: u32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006af3b() -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the callee is the game's own; it receives the address of a
    // live local that it may write.
    unsafe {
        let i_var1 = _AIL_midiOutOpen_12((&raw mut local_8) as usize as u32, 0, 0xffff_ffff);
        if i_var1 != 0 {
            let i_var1 = _AIL_midiOutOpen_12((&raw mut local_8) as usize as u32, 0, 0);
            if i_var1 != 0 {
                local_8 = 0;
            }
        }
    }
    local_8
}
