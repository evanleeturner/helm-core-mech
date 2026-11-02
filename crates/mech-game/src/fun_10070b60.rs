// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eebb0: [u8; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10070b60() {
    mech_trace::enter!("10070b60");
    let mut local_8: i32 = 0;
    while local_8 < 0x100 {
        // SAFETY: DAT_100eebb0 is the game's own 256-byte table; local_8 is
        // within 0..0x100.
        unsafe {
            mech_trace::wr!(
                "10070b60",
                (&raw mut DAT_100eebb0).cast::<u8>().add(local_8 as usize),
                write,
                local_8 as u8
            );
        }
        local_8 = local_8.wrapping_add(1);
    }
}
