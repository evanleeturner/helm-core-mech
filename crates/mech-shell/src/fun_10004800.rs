// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083770: u32;
    static mut DAT_10083800: u8;
    static mut DAT_10083870: u8;
    static mut DAT_10085170: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004800() {
    mech_trace::enter!("10004800");
    // SAFETY: the globals are the game's own memory; the table pointers
    // are walked and read exactly as the original does.
    unsafe {
        mech_trace::gw!("10004800", "10083770", DAT_10083770, 0);
        let mut p_var1: *const i32 = (&raw mut DAT_10083800) as *const i32;
        loop {
            let v = mech_trace::rd!("10004800", p_var1, read_unaligned);
            if v != -1 {
                let off = (v / 100).wrapping_mul(0x40) as isize;
                let base = (&raw mut DAT_10085170) as *const u8;
                let item = mech_trace::rd!(
                    "10004800",
                    base.byte_offset(off).cast::<i32>(),
                    read_unaligned
                );
                mech_trace::gw!(
                    "10004800",
                    "10083770",
                    DAT_10083770,
                    mech_trace::gr!("10004800", "10083770", DAT_10083770).wrapping_add(item as u32)
                );
            }
            p_var1 = p_var1.wrapping_add(2);
            if !((p_var1 as usize) < ((&raw mut DAT_10083870) as usize)) {
                break;
            }
        }
    }
}
