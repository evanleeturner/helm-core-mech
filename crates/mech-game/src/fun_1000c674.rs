// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7578: i32;
    static mut DAT_100a757c: i32;
    static mut DAT_100a7580: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c674(param_1: u32, param_2: u32) -> bool {
    mech_trace::enter!("1000c674", param_1, param_2);
    // SAFETY: the globals are the game's own memory; the buffer address held
    // in DAT_100a7580 is written at the offsets the original uses.
    unsafe {
        let b_var1 = mech_trace::gr!("1000c674", "100a7578", DAT_100a7578)
            < mech_trace::gr!("1000c674", "100a757c", DAT_100a757c);
        if b_var1 {
            let base = mech_trace::gr!("1000c674", "100a7580", DAT_100a7580) as u32;
            let idx = mech_trace::gr!("1000c674", "100a7578", DAT_100a7578) as u32;
            let p0 = base.wrapping_add(idx.wrapping_mul(8)) as *mut u32;
            mech_trace::wr!("1000c674", p0, write_unaligned, param_1);
            let p1 = base.wrapping_add(4).wrapping_add(
                (mech_trace::gr!("1000c674", "100a7578", DAT_100a7578) as u32).wrapping_mul(8),
            ) as *mut u32;
            mech_trace::wr!("1000c674", p1, write_unaligned, param_2);
            mech_trace::gw!(
                "1000c674",
                "100a7578",
                DAT_100a7578,
                (mech_trace::gr!("1000c674", "100a7578", DAT_100a7578) as u32).wrapping_add(1)
                    as i32
            );
        }
        mech_trace::rt!("1000c674", b_var1)
    }
}
