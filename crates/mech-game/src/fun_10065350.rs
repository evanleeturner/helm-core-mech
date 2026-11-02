// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7f98: i32;
    static mut DAT_100c7fa0: i32;
    static mut DAT_100c7e88: u8;
    static mut DAT_100c7e89: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064e6a(param_1: i32) -> i32;
    fn FUN_10064cd8(param_1: *mut i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065350(mut param_1: u32, mut param_2: i32) -> i32 {
    let mut local_c: i32 = 0;
    // SAFETY: the globals and callees are the game's own; the bit buffer is
    // indexed by the game's own counter exactly as the original does.
    unsafe {
        loop {
            let mut i_var1 = param_2;
            if (DAT_100c7f98 == 0xfe && DAT_100c7fa0 == 0) || 0xfe < DAT_100c7f98 {
                let i_var2 = FUN_10064e6a(0xff);
                if i_var2 != 0 {
                    return -1;
                }
                let i_var2 = FUN_10064cd8((&raw mut DAT_100c7e88).cast::<i32>(), 0xff);
                if i_var2 != 0 {
                    return -1;
                }
                DAT_100c7f98 = 0;
                DAT_100c7e88 = 0;
                DAT_100c7fa0 = 8;
            }
            if DAT_100c7fa0 < param_2 {
                let b = (DAT_100c7fa0 as u8) as u32;
                let mask = (1i32.wrapping_shl(b & 0x1f) as u32).wrapping_sub(1) & param_1;
                let sh = ((8i32.wrapping_sub(b as i32)) & 0x1f) as u32;
                let p = (&raw mut DAT_100c7e88).offset(DAT_100c7f98 as isize);
                let v = (mask.wrapping_shl(sh) as u8) | p.read();
                p.write(v);
                i_var1 = DAT_100c7fa0;
                let i_var2 = DAT_100c7f98;
                param_1 =
                    ((param_1 as i32).wrapping_shr(((DAT_100c7fa0 as u8) as u32) & 0x1f)) as u32;
                param_2 = param_2.wrapping_sub(DAT_100c7fa0);
                DAT_100c7f98 = DAT_100c7f98.wrapping_add(1);
                (&raw mut DAT_100c7e89).offset(i_var2 as isize).write(0);
                DAT_100c7fa0 = 8;
            } else {
                let b = (param_2 as u8) as u32;
                let mask = (1i32.wrapping_shl(b & 0x1f) as u32).wrapping_sub(1) & param_1;
                let sh = ((8i32.wrapping_sub((DAT_100c7fa0 as u8) as i32)) & 0x1f) as u32;
                let p = (&raw mut DAT_100c7e88).offset(DAT_100c7f98 as isize);
                let v = (mask.wrapping_shl(sh) as u8) | p.read();
                p.write(v);
                DAT_100c7fa0 = DAT_100c7fa0.wrapping_sub(param_2);
                param_2 = 0;
            }
            local_c = local_c.wrapping_add(i_var1);
            if param_2 == 0 {
                return local_c;
            }
        }
    }
}
