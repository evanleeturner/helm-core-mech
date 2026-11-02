// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: i32;
    static mut DAT_100b07fc: [i8; 0];
    static mut DAT_10123170: [u8; 0];
    static mut DAT_1012327a: [u8; 0];
    static mut DAT_10123327: [u8; 0];
    static mut s__s_failed_100b07f0: [i8; 0];
    static mut s__s_successful_100b07e0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *mut i8, ...) -> i32;
    fn FUN_1001028b(param_1: *mut i8) -> u32;
    fn FUN_1004d6e0(param_1: *mut i8);
    fn FUN_10008351(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004e12e(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    let mut local_78: [i8; 100] = [0; 100];
    let mut local_14: i32 = 0;
    let local_10: *mut i8;
    // SAFETY: globals and callees are the game's own memory; offsets index
    // the game's tables exactly as the original does.
    unsafe {
        let i_var1 = param_1
            .wrapping_mul(0x3c0a)
            .wrapping_add(param_2.wrapping_mul(0x13f));
        let local_8: *mut u8 = (&raw mut DAT_1012327a)
            .cast::<u8>()
            .byte_add(i_var1 as isize as usize);
        let tbl = (&raw mut DAT_10123170)
            .cast::<u8>()
            .byte_add(param_2.wrapping_mul(4) as isize as usize);
        if param_1 == DAT_100b0710 && tbl.cast::<i32>().read_unaligned() == 0 {
            let name = (&raw mut DAT_10123327)
                .cast::<u8>()
                .byte_add(i_var1 as isize as usize);
            if param_3 == 5 {
                sprintf(
                    local_78.as_mut_ptr(),
                    (&raw mut s__s_successful_100b07e0).cast::<i8>(),
                    name,
                );
                local_14 = local_8.byte_add(0x81).cast::<i32>().read_unaligned();
                let _ = FUN_1001028b(local_8.byte_add(0x89).cast::<i8>());
            } else if param_3 == 6 {
                sprintf(
                    local_78.as_mut_ptr(),
                    (&raw mut s__s_failed_100b07f0).cast::<i8>(),
                    name,
                );
                local_14 = local_8.byte_add(0x85).cast::<i32>().read_unaligned();
                let _ = FUN_1001028b(local_8.byte_add(0x99).cast::<i8>());
            }
            if local_78[0] == 0x20 || local_8.byte_add(0x74).read() == 0 {
                local_10 = (&raw mut DAT_100b07fc).cast::<i8>();
            } else {
                local_10 = local_78.as_mut_ptr();
            }
            FUN_1004d6e0(local_10);
            FUN_10008351(&raw mut local_14);
            tbl.cast::<i32>().write_unaligned(1);
        }
    }
    1
}
