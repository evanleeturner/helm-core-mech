// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004010f0() {
    mech_trace::enter!("004010f0");
    let mut p_var1: *mut u32 = 0xb0f00usize as *mut u32;
    loop {
        // SAFETY: the address range 0xb0f00..0xb0fa0 is the game's own memory,
        // read and written word by word as the original does.
        unsafe {
            mech_trace::wr!(
                "004010f0",
                p_var1,
                write_unaligned,
                mech_trace::rd!("004010f0", p_var1, read_unaligned) & 0xff00ff00
            );
        }
        p_var1 = p_var1.wrapping_add(1);
        if (p_var1 as usize as i32) >= 0xb0fa0 {
            break;
        }
    }
}
