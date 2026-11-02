// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100b3940: i32;
    static mut _DAT_100b3944: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005aae4(param_1: i32) -> u32;
    fn FUN_1005aaf7(param_1: i32, param_2: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066ad9(param_1: *mut i8, param_2: i32, param_3: i32) -> i32 {
    let mut local_c: u32 = 0;
    let mut local_18: u32 = 0;
    if param_3 == 0 || param_1.is_null() || param_2 == 0 {
        return 0;
    }
    // SAFETY: the callees and globals are the game's own; param_1 points at a
    // NUL-terminated string and param_3 at a record of at least 0x14 bytes.
    unsafe {
        let mut local_10: *const i8 = param_1;
        let u_var2 = FUN_1005aae4(param_2);
        let mut local_8: u32 = u_var2;
        loop {
            let c_var1 = local_10.read();
            local_10 = local_10.add(1);
            if c_var1 == 0 {
                break;
            }
            if c_var1 == 10 {
                if (local_c as i32) < (local_18 as i32) {
                    local_c = local_18;
                }
                local_18 = 0;
                local_8 = local_8.wrapping_add(u_var2);
            } else {
                let i_var3 = FUN_1005aaf7(param_2, c_var1 as i32);
                local_18 = local_18.wrapping_add(i_var3 as u32);
            }
        }
        if (local_c as i32) < (local_18 as i32) {
            local_c = local_18;
        }
        let base = param_3 as usize as *mut u8;
        base.byte_add(8).cast::<u32>().write_unaligned(0);
        let v = base.byte_add(8).cast::<u32>().read_unaligned();
        base.byte_add(4).cast::<u32>().write_unaligned(v);
        let u_var4 = FUN_10002570(local_c, 0x10000i32.wrapping_sub(_DAT_100b3940));
        base.byte_add(0xc).cast::<u32>().write_unaligned(u_var4);
        let u_var4 = FUN_10002570(local_8, 0x10000i32.wrapping_sub(_DAT_100b3944));
        base.byte_add(0x10).cast::<u32>().write_unaligned(u_var4);
    }
    param_3
}
