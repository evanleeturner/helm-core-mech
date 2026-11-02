// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1002c39a() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ab38(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036142(param_1: i32) -> i32 {
    // SAFETY: the callees and globals are the game's own; the table entry is
    // a pointer to a structure whose field at +0x176 is read as in the original.
    unsafe {
        let i_var1 = FUN_1002c39a() as i32;
        let mut local_10: i32 = if i_var1.wrapping_sub(1) < param_1 && param_1 != 4 {
            0
        } else {
            1
        };
        if local_10 != 0 {
            let u_var2 = FUN_1002ab38(param_1) as u32;
            if u_var2 < DAT_100b38d8 {
                let base = (&raw mut DAT_100eef40).cast::<u32>();
                let ptr = base.add(u_var2 as usize).read_unaligned();
                let v = (ptr.wrapping_add(0x176) as *const i16).read_unaligned();
                if v != 0xc {
                    local_10 = 1;
                } else {
                    local_10 = 0;
                }
            } else {
                local_10 = 0;
            }
        }
        local_10
    }
}
