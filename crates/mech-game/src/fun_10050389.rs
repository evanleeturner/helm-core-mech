// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: i32;
    static mut DAT_100eeed0: u32;
    static mut DAT_100eeed4: u32;
    static mut DAT_100eeed8: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_101155c0: u8;
    static mut DAT_101155e0: u8;
    static mut DAT_101155e4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003dfde(param_1: *mut u16) -> u8;
    fn FUN_100504ef(param_1: i32, param_2: i32, param_3: *mut i32) -> u32;
    fn FUN_1003df3a(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050389(param_1: i32, param_2: i32) -> u32 {
    let mut local_10: u32 = 1;
    if param_2 == -1 {
        local_10 = 1;
    } else {
        // SAFETY: all globals and callees are the game's own memory; the table
        // entries are read at the offsets the original uses.
        unsafe {
            DAT_100eeed0 = 1;
            DAT_100eeed4 = 0x100;
            DAT_100eeed8 = param_1 as u32;
            let mut local_8: i32 = 0;
            while local_8 < DAT_100b0864 {
                let i_var1 = local_8.wrapping_mul(0x48) as isize;
                let p0 = (&raw mut DAT_101155c0).wrapping_offset(i_var1);
                let p1 = (&raw mut DAT_101155e0).wrapping_offset(i_var1);
                let p2 = (&raw mut DAT_101155e4).wrapping_offset(i_var1);
                let key = p0.cast::<i32>().read_unaligned();
                if key == param_1 {
                    let ent = (&raw mut DAT_100eef40)
                        .cast::<u32>()
                        .wrapping_offset(key as isize)
                        .read_unaligned();
                    let flags = (ent.wrapping_add(0x14) as usize as *const u16).read_unaligned();
                    if (flags & 2) == 0 || p1.cast::<i32>().read_unaligned() != 0 {
                        let i_var2 = FUN_100504ef(local_8, param_2, core::ptr::null_mut());
                        if i_var2 == 0 {
                            local_10 = 0;
                        }
                    } else {
                        FUN_1003dfde(p2.cast::<u32>().read_unaligned() as usize as *mut u16);
                    }
                    if p1.cast::<i32>().read_unaligned() == 0 {
                        FUN_1003df3a(p2.cast::<u32>().read_unaligned() as usize as *mut u16);
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
            DAT_100eeed0 = 0;
        }
    }
    local_10
}
