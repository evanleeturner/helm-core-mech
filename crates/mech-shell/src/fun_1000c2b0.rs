// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: *mut i32;
    static mut DAT_100a6798: *mut i32;
    static mut DAT_100a679c: *mut i32;
    static mut DAT_100a67a0: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017c90(
        param_1: *mut u32,
        param_2: i32,
        param_3: u32,
        param_4: u8,
        param_5: u32,
    ) -> u32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
    fn FUN_10018090(param_1: *mut i32, param_2: *mut i32) -> u8;
    fn FUN_1000be30(param_1: *mut u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c2b0() {
    // SAFETY: all callees and globals are the game's own; collection pointers
    // and item records are read and written where the original does.
    unsafe {
        let mut local_8: *mut i32 = core::ptr::null_mut();
        let mut local_4: *mut i32 = core::ptr::null_mut();
        let mut i_var4: i32 = 0;
        FUN_10017c90((&raw mut local_8).cast::<u32>(), 10, 0, 4, 0);
        FUN_10017c90((&raw mut local_4).cast::<u32>(), 10, 0, 4, 0);
        if 0 < DAT_100a6798.add(1).read_unaligned() {
            loop {
                let mut i_var1 = FUN_10018060(DAT_100a6798 as i32, i_var4) as i32;
                if 0 < (i_var1 as usize as *const u8)
                    .byte_add(0x814)
                    .cast::<i32>()
                    .read_unaligned()
                {
                    FUN_10017df0(local_8, i_var1 as u32);
                    let key = (i_var1 as usize as *const u8)
                        .byte_add(0x810)
                        .cast::<i32>()
                        .read_unaligned();
                    i_var1 = FUN_1000ad30(DAT_100a678c as i32, key);
                    FUN_10017df0(local_4, i_var1 as u32);
                }
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < DAT_100a6798.add(1).read_unaligned()) {
                    break;
                }
            }
        }
        FUN_100180f0(DAT_100a6798, 0);
        FUN_100180f0(DAT_100a678c, 0);
        FUN_10018090(DAT_100a6798, local_8);
        FUN_10018090(DAT_100a678c, local_4);
        i_var4 = 0;
        if 0 < DAT_100a679c.add(1).read_unaligned() {
            loop {
                let i_var1 = i_var4.wrapping_add(1);
                let pu_var2 = FUN_10018060(DAT_100a679c as i32, i_var4);
                FUN_10017df0(DAT_100a6798, pu_var2);
                let u_var3 = FUN_1000be30(pu_var2 as usize as *mut u32);
                FUN_10017df0(DAT_100a678c, u_var3);
                i_var4 = i_var1;
                if !(i_var1 < DAT_100a679c.add(1).read_unaligned()) {
                    break;
                }
            }
        }
        FUN_100180f0(DAT_100a679c, 0);
        FUN_100180f0(DAT_100a67a0, 0);
    }
}
