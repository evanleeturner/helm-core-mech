// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1011515c: u32;
    static mut DAT_10167660: u32;
    static mut DAT_10168288: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035f3b() -> i32 {
    mech_trace::enter!("10035f3b");
    // SAFETY: the globals are the game's own memory; the byte written is at
    // the address the original computes from DAT_10167660.
    unsafe {
        let i_var2 = mech_trace::gr!("10035f3b", "10167660", DAT_10167660) as i32;
        let i_var3 = i_var2.wrapping_sub(0x30);
        let p_var1 = i_var2.wrapping_sub(0x13) as *mut u8;
        mech_trace::gw!("10035f3b", "10167660", DAT_10167660, i_var3 as u32);
        mech_trace::wr!("10035f3b", p_var1, write, 0);
        if (i_var2 as u32).wrapping_sub(0xf8)
            <= mech_trace::gr!("10035f3b", "10168288", DAT_10168288)
        {
            mech_trace::gw!("10035f3b", "1011515c", DAT_1011515c, 0);
        }
        mech_trace::rt!("10035f3b", i_var3)
    }
}
