// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a678c: *mut i32;
    static mut DAT_100a6798: *mut i32;
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
    fn FUN_10017d70(param_1: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c110() {
    let mut local_8: *mut i32 = core::ptr::null_mut();
    let mut local_4: *mut i32 = core::ptr::null_mut();
    let mut i_var2: i32 = 0;
    // SAFETY: the callees and globals are the game's own; the collection and
    // item pointers come from the game's collection functions and are used
    // as the original does.
    unsafe {
        FUN_10017c90((&raw mut local_8).cast::<u32>(), 10, 0, 4, 0);
        FUN_10017c90((&raw mut local_4).cast::<u32>(), 10, 0, 4, 0);
        if 0 < DAT_100a6798.byte_add(4).read_unaligned() {
            loop {
                let item = FUN_10018060(DAT_100a6798 as i32, i_var2);
                let count = ((item as usize).wrapping_add(0x814) as *const i32).read_unaligned();
                if 0 < count {
                    FUN_10017df0(local_8, item);
                    let key = ((item as usize).wrapping_add(0x810) as *const i32).read_unaligned();
                    let found = FUN_1000ad30(DAT_100a678c as i32, key);
                    FUN_10017df0(local_4, found as u32);
                }
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < DAT_100a6798.byte_add(4).read_unaligned()) {
                    break;
                }
            }
        }
        FUN_100180f0(DAT_100a6798, 0);
        FUN_100180f0(DAT_100a678c, 0);
        FUN_10018090(DAT_100a6798, local_8);
        FUN_10018090(DAT_100a678c, local_4);
        FUN_10017d70(local_8);
        FUN_10017d70(local_4);
    }
}
