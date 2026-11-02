// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6798: u32;
    static mut DAT_100a67a4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ce20() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000adb0(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100431c0() {
    // SAFETY: the callees and globals are the game's own; the pointers
    // derived from their results address the game's live objects, read and
    // written where the original does.
    unsafe {
        let i_var1 = FUN_1000ce20();
        if (i_var1.wrapping_add(0x20) as *const i32).read_unaligned() == -1
            && ((DAT_100a6798.wrapping_add(4)) as *const i32).read_unaligned() == 1
        {
            let i_var2 = FUN_10018060(DAT_100a6798 as i32, 0) as i32;
            if ((i_var2.wrapping_add(0x80c)) as *const i32).read_unaligned() != -1 {
                let i_var3 = FUN_1000adb0(
                    DAT_100a67a4 as i32,
                    ((i_var2.wrapping_add(0x80c)) as *const i32).read_unaligned(),
                );
                if i_var3 != 0 {
                    (i_var3.wrapping_add(0x20) as *mut u32).write_unaligned(0xffffffff);
                    (i_var2.wrapping_add(0x80c) as *mut u32).write_unaligned(0xffffffff);
                }
            }
            let v = (i_var1.wrapping_add(0x24) as *const u32).read_unaligned();
            (i_var2.wrapping_add(0x80c) as *mut u32).write_unaligned(v);
            let w = (i_var2.wrapping_add(0x810) as *const u32).read_unaligned();
            (i_var1.wrapping_add(0x20) as *mut u32).write_unaligned(w);
        }
    }
}
