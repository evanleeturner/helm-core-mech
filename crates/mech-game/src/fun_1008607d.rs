// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ba7bc: u32;
    static mut DAT_100c9054: u32;
    static mut DAT_100c9058: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10085f40(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1008607d() {
    mech_trace::enter!("1008607d");
    // SAFETY: the globals and the callee are the game's own; the list
    // pointers are read and written at the offsets the original uses.
    unsafe {
        mech_trace::gw!("1008607d", "100c9054", DAT_100c9054, 0);
        mech_trace::gw!("1008607d", "100c9058", DAT_100c9058, 0);
        let mut local_c: u32 = 0;
        while local_c < 0x3f1 {
            let mut local_8: u32 = mech_trace::rd!(
                "1008607d",
                (mech_trace::gr!("1008607d", "100ba7bc", DAT_100ba7bc)
                    .wrapping_add(local_c.wrapping_mul(4)) as usize as *const u32),
                read_unaligned
            );
            while local_8 != 0 {
                let p = local_8 as usize as *mut u8;
                if mech_trace::rd!("1008607d", p.byte_add(2).cast::<i16>(), read_unaligned) == 0 {
                    mech_trace::wr!("1008607d", p.byte_add(2).cast::<i16>(), write_unaligned, 1);
                    mech_trace::cr!("1008607d", "10085f40", FUN_10085f40(local_8 as i32));
                }
                local_8 = mech_trace::rd!("1008607d", p.byte_add(8).cast::<u32>(), read_unaligned);
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
