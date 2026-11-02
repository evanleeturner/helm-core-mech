// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb4: u32;
    static mut DAT_10160e80: [u8; 0];
    static mut DAT_10160e90: [u8; 0];
    static mut PTR_DAT_100af0c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn AIL_sequence_status(param_1: u32) -> u32;
    fn AIL_stop_sequence(param_1: u32) -> u32;
    fn AIL_release_sequence_handle(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c260() {
    // SAFETY: the globals and callees are the game's own; the arrays hold
    // eight 4-byte and eight 2-byte entries, accessed where the original does.
    unsafe {
        if DAT_100aefb4 == 0 {
            let mut local_8: i32 = 0;
            while local_8 < 8 {
                let p90 = (&raw mut DAT_10160e90)
                    .cast::<u8>()
                    .byte_add(local_8.wrapping_mul(4) as usize);
                if p90.cast::<i32>().read_unaligned() != -1 {
                    let u_var1 = p90.cast::<u32>().read_unaligned();
                    let i_var2 = AIL_sequence_status(u_var1);
                    if i_var2 as i32 == 4 {
                        AIL_stop_sequence(u_var1);
                    }
                    AIL_release_sequence_handle(u_var1);
                    p90.cast::<u32>().write_unaligned(0xffff_ffff);
                }
                let p80 = (&raw mut DAT_10160e80)
                    .cast::<u8>()
                    .byte_add(local_8.wrapping_mul(2) as usize);
                if p80.cast::<i16>().read_unaligned() != 0 {
                    let v = p80.cast::<i16>().read_unaligned();
                    FUN_1008664f(v as i32 as u32, PTR_DAT_100af0c4 as *mut i32);
                    p80.cast::<u16>().write_unaligned(0);
                }
                local_8 = local_8.wrapping_add(1);
            }
        }
    }
}
