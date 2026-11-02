// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1f4: u32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_100ad670: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "sprintf"]
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005dd5a(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a9bb(param_1: i32, param_2: u32) {
    let mut local_58: [i8; 80] = [0; 80];
    // SAFETY: globals and callees are the game's own; the mech record
    // pointer comes from the game's table and is read and written at the
    // original's offsets.
    unsafe {
        if DAT_100adf38 == param_1 || DAT_100aa1f4 != 0 {
            let local_8: i32 = (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(param_1 as isize)
                .read_unaligned() as i32;
            let base = local_8 as *mut u8;
            if u32::from(base.byte_add(0x178).cast::<u16>().read_unaligned()) != (param_2 | 0x200)
                || base.byte_add(0x176).cast::<i16>().read_unaligned() != 3
            {
                sprintf(
                    local_58.as_mut_ptr(),
                    (&raw const DAT_100ad670).cast::<i8>(),
                    DAT_100b38fc,
                    base.byte_add(8).cast::<u32>().read_unaligned(),
                    param_1,
                    param_2,
                );
                FUN_1005dd5a(local_58.as_mut_ptr() as u32);
            }
            base.byte_add(0x17a)
                .cast::<u16>()
                .write_unaligned((param_2 as u16) | 0x200);
            base.byte_add(0x176).cast::<i16>().write_unaligned(3);
        }
    }
}
