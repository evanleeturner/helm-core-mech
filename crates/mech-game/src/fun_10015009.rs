// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100150df(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_100157bd(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1001544e(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015009(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> u32 {
    // SAFETY: param_1 points at the game's own structure; the callees are the game's.
    unsafe {
        if ((param_1 as *const u8)
            .byte_add(2)
            .cast::<u16>()
            .read_unaligned())
            < 3
        {
            return 0;
        }
        let a = (param_1 as *const u8)
            .byte_add(0x14)
            .cast::<i32>()
            .read_unaligned();
        let b = (param_1 as *const u8)
            .byte_add(0x18)
            .cast::<i32>()
            .read_unaligned();
        let c = (param_1 as *const u8)
            .byte_add(0x1c)
            .cast::<i32>()
            .read_unaligned();
        let s = a >> 31;
        let i_var2 = (a ^ s).wrapping_sub(s);
        let s = b >> 31;
        let i_var3 = (b ^ s).wrapping_sub(s);
        let s = c >> 31;
        let i_var4 = (c ^ s).wrapping_sub(s);
        if i_var2 < i_var3 && i_var4 < i_var3 {
            FUN_100150df(param_1, param_2, param_3, param_5)
        } else if i_var4 < i_var2 {
            FUN_100157bd(param_1, param_2, param_4, param_5)
        } else {
            FUN_1001544e(param_1, param_2, param_3, param_4)
        }
    }
}
