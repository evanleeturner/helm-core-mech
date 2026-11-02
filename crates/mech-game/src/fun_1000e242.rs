// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10068d4e(param_1: i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_10068db6(param_1: u32) -> u8;
    fn FUN_10068dc9(param_1: i32, param_2: *mut i32, param_3: i32, param_4: i32) -> *mut u16;
    fn FUN_100141c0(param_1: *mut u16, param_2: i32) -> u8;
    fn FUN_10089cc9(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u8;
    fn FUN_1001b7cd(param_1: *mut u16, param_2: *mut i32) -> u8;
    fn FUN_1003da99(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e242(param_1: *mut i32, param_2: i32, param_3: i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's node structure of at least 11
    // ints; the callees are the game's own.
    unsafe {
        let r = |i: usize| -> i32 { param_1.add(i).read_unaligned() };
        let abs = |d: i32| -> i32 {
            let s = (d >> 31) as u32;
            ((d as u32) ^ s).wrapping_sub(s) as i32
        };
        let local_14 = abs(r(1).wrapping_sub(r(0)));
        let local_10 = abs(r(3).wrapping_sub(r(2)));
        let local_c = abs(r(5).wrapping_sub(r(4)));
        let local_20 = local_14.wrapping_add(9) / 10;
        let local_1c = local_10.wrapping_mul(2).wrapping_add(0x12) / 10;
        let local_18 = local_c.wrapping_add(9) / 10;
        FUN_10068d4e(local_20, local_1c, local_18);
        FUN_10068db6(4);
        let mut local_24: i32 = 0;
        let local_2c = FUN_10068dc9(param_2, &raw mut local_24, param_3, 0);
        if !local_2c.is_null() {
            let mut local_68 = [0i32; 12];
            FUN_100141c0(local_2c, 4);
            let local_28 = r(1).wrapping_add(r(0)) >> 1;
            let local_38 = r(2).wrapping_add(r(3)) >> 1;
            let local_30 = r(5).wrapping_add(r(4)) >> 1;
            FUN_10089cc9(local_68.as_mut_ptr(), 0, 0, 0, local_28, local_38, local_30);
            FUN_1001b7cd(local_2c, local_68.as_mut_ptr());
            FUN_1003da99(local_2c);
        }
        let mut i: usize = 0;
        while i < 4 {
            FUN_1000e242(r(i + 7) as usize as *mut i32, param_2, param_3);
            i += 1;
        }
    }
}
