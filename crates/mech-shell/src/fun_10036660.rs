// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093c60: u32;
    static mut DAT_100b5298: [u32; 0];
    static mut DAT_10093c64: u32;
    static mut _DAT_100b52a0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036660(param_1: *const u32, param_2: u32) {
    mech_trace::enter!("10036660", param_1, param_2);
    // SAFETY: the globals are the game's own memory; the source and the
    // destination buffer are read and written exactly where the original does.
    unsafe {
        if (mech_trace::gr!("10036660", "10093c60", DAT_10093c60).wrapping_add(param_2) as i32)
            < 0x801
        {
            let mut source = param_1 as *const u8;
            let mut destination = (&raw mut DAT_100b5298 as *mut u8).byte_add(mech_trace::gr!(
                "10036660",
                "10093c60",
                DAT_10093c60
            )
                as usize);
            let mut counter = param_2 >> 2;
            while counter != 0 {
                mech_trace::wr!(
                    "10036660",
                    (destination as *mut u32),
                    write_unaligned,
                    mech_trace::rd!("10036660", (source as *const u32), read_unaligned)
                );
                source = source.byte_add(4);
                destination = destination.byte_add(4);
                counter = counter.wrapping_sub(1);
            }
            let aligned_size = param_2.wrapping_add(3) & 0xffff_fffc;
            counter = param_2 & 3;
            while counter != 0 {
                mech_trace::dw!("10036660", destination, mech_trace::dr!("10036660", source));
                source = source.byte_add(1);
                destination = destination.byte_add(1);
                counter = counter.wrapping_sub(1);
            }
            mech_trace::gw!(
                "10036660",
                "10093c60",
                DAT_10093c60,
                mech_trace::gr!("10036660", "10093c60", DAT_10093c60).wrapping_add(aligned_size)
            );
            if (mech_trace::gr!("10036660", "10093c64", DAT_10093c64) as i32)
                < (aligned_size as i32)
            {
                mech_trace::gw!("10036660", "10093c64", DAT_10093c64, aligned_size);
                mech_trace::gw!("10036660", "_DAT_100b52a0", _DAT_100b52a0, aligned_size);
            }
        }
    }
}
