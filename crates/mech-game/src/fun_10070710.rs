// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eecb0: [u32; 0x3c];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10070710() {
    mech_trace::enter!("10070710");
    let mut local_8: i32 = 0;
    while local_8 < 0x3c {
        // SAFETY: DAT_100eecb0 is the game's own array of 0x3c words; the index is in range.
        unsafe {
            let p = (&raw mut DAT_100eecb0).cast::<u32>();
            mech_trace::wr!("10070710", p.add(local_8 as usize), write, 0xfffffffe_u32);
        }
        local_8 = local_8.wrapping_add(1);
    }
}
