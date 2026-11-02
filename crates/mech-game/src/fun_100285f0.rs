// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100285f0(param_1: i32, param_2: u32, param_3: u16) {
    mech_trace::enter!("100285f0", param_1, param_2, param_3);
    let mut local_8: i32 = 0;
    // SAFETY: the globals and the records they point to are the game's own
    // memory; fields are read and written at the original's offsets.
    unsafe {
        while local_8 < mech_trace::gr!("100285f0", "100b38d8", DAT_100b38d8) {
            let i_var1 = mech_trace::rd!(
                "100285f0",
                (&raw mut DAT_100eef40).cast::<u32>().add(local_8 as usize),
                read_unaligned
            );
            let base = i_var1 as usize as *mut u8;
            let flags = base.byte_add(0x17a).cast::<u16>();
            if mech_trace::rd!("100285f0", base.byte_add(8).cast::<i32>(), read_unaligned)
                == param_1
                && (mech_trace::rd!("100285f0", flags, read_unaligned) & 0xf000) == 0
                && (mech_trace::rd!("100285f0", flags, read_unaligned) & 0x200) != 0
                && (mech_trace::rd!("100285f0", flags, read_unaligned) & 0xff) as u32 == param_2
            {
                mech_trace::wr!("100285f0", flags, write_unaligned, param_3 | 0x200);
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
