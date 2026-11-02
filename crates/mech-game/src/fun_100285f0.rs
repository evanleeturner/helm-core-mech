// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100285f0(param_1: i32, param_2: u32, param_3: u16) {
    let mut local_8: i32 = 0;
    // SAFETY: the globals and the records they point to are the game's own
    // memory; fields are read and written at the original's offsets.
    unsafe {
        while local_8 < DAT_100b38d8 {
            let i_var1 = (&raw mut DAT_100eef40)
                .cast::<u32>()
                .add(local_8 as usize)
                .read_unaligned();
            let base = i_var1 as usize as *mut u8;
            let flags = base.byte_add(0x17a).cast::<u16>();
            if base.byte_add(8).cast::<i32>().read_unaligned() == param_1
                && (flags.read_unaligned() & 0xf000) == 0
                && (flags.read_unaligned() & 0x200) != 0
                && (flags.read_unaligned() & 0xff) as u32 == param_2
            {
                flags.write_unaligned(param_3 | 0x200);
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
