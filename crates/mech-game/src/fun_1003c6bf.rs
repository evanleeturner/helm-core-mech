// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb4: i32;
    static mut DAT_10160e90: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_sequence_status_4(param_1: u32) -> u32;
    fn FUN_1003c260() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c6bf() -> u32 {
    mech_trace::enter!("1003c6bf");
    let mut s_var1: i16 = 0;
    let mut in_eax: u32 = 0;
    let mut local_8: u32 = 0;
    let u_var2: u32;

    // SAFETY: the globals and callees are the game's own memory and code;
    // the table is indexed with the C's pointer arithmetic on an int pointer.
    unsafe {
        if mech_trace::gr!("1003c6bf", "100aefb4", DAT_100aefb4) == 0 {
            while local_8 < 8 {
                let p = (&raw mut DAT_10160e90).wrapping_add(local_8.wrapping_mul(4) as usize);
                let v = mech_trace::rd!("1003c6bf", p, read_unaligned);
                if v != -1 {
                    in_eax = mech_trace::cr!(
                        "1003c6bf",
                        "_AIL_sequence_status_4",
                        _AIL_sequence_status_4(v as u32)
                    );
                    if in_eax == 4 {
                        s_var1 = 1;
                        break;
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
            let mut hi = (in_eax >> 0x10) << 16;
            if s_var1 == 0 {
                in_eax = mech_trace::cr!("1003c6bf", "1003c260", FUN_1003c260());
                hi = (in_eax >> 0x10) << 16;
            }
            u_var2 = hi | (s_var1 as i32 as u32);
        } else {
            u_var2 = in_eax & 0xffff0000;
        }
    }
    mech_trace::rt!("1003c6bf", u_var2)
}
