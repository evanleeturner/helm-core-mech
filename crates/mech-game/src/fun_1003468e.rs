// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_10034c0a(param_1: i32, param_2: i32);
    fn FUN_1004d60d(param_1: i32) -> u32;
    fn FUN_10034e74(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_10066f36(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> *mut i32;
    fn FUN_100340f0(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10083808() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003468e(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object whose first field is a
    // pointer to a record; callees and globals are the game's own.
    unsafe {
        let local_c: i32 = param_1.read();
        let mut local_24: u32 =
            ((local_c as u32).wrapping_add(0xdc) as *const u32).read_unaligned();
        if local_24 != 0 && (local_24 & 0x1000) == 0 {
            let local_8: u32 = local_24 & 0xff;
            local_24 &= 0xf00;
            if local_24 == 0x100 {
                let local_1c: i32 = 0xc4;
                let local_20: i32 = 0xca;
                let mut local_10: i32 =
                    ((local_c as u32).wrapping_add(200) as *const i32).read_unaligned();
                let mut local_14: i32 =
                    ((local_c as u32).wrapping_add(0xcc) as *const i32).read_unaligned();
                let mut local_18: i32 =
                    ((local_c as u32).wrapping_add(0xd0) as *const i32).read_unaligned();
                let i_var1 = FUN_1003ba89(&raw mut local_10, &raw mut local_14, &raw mut local_18);
                if i_var1 == 0 {
                    let mut pair: [i32; 2] = [local_10, local_14];
                    let p = pair.as_mut_ptr();
                    FUN_10066f36(0x101f52e0, p, p);
                    FUN_1003510c(pair[0], pair[1], local_20);
                } else {
                    FUN_1003510c(local_10, local_14, local_1c);
                }
            } else if local_24 == 0x200 {
                let i_var1 = FUN_1004d5e3(local_8 as i32) as i32;
                let v = (&raw mut DAT_100eef40 as *const u32)
                    .add(local_8 as usize)
                    .read_unaligned();
                FUN_10034c0a(v as i32, i_var1);
            } else if local_24 == 0x400 {
                let i_var1 = FUN_1004d60d(local_8 as i32) as i32;
                let i_var2 = FUN_10083808() as i32;
                FUN_10034e74(local_8 as i32, i_var2, i_var1);
            }
        }
        if local_24 != 0x100 {
            FUN_100340f0(param_1);
        }
    }
}
