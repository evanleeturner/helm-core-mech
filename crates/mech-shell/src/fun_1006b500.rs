// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut i32;
    static mut DAT_100aa6c8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> i32;
    fn FUN_1006b9c0(param_1: i32, param_2: i8, param_3: *mut u32) -> u32;
    fn FUN_1006b450(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_10010d80(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b500(
    param_1: i32,
    param_2: i32,
    param_3: i8,
    param_4: i32,
    param_5: i32,
) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the callees and globals are the game's own; param_4 and param_5
    // are addresses of game structures, accessed at the offsets the original uses.
    unsafe {
        let idx = param_2.wrapping_add(param_1.wrapping_mul(12));
        let i_var2 = ((param_5 as u32)
            .wrapping_add(0x368)
            .wrapping_add((idx as u32).wrapping_mul(4)) as *const i32)
            .read_unaligned();
        let off = (idx as u32).wrapping_mul(4);
        let slot4 = (off.wrapping_add(0x368).wrapping_add(param_4 as u32)) as *mut i32;
        let slot5 = (off.wrapping_add(0x368).wrapping_add(param_5 as u32)) as *const i32;
        let flag = (param_4 as u32).wrapping_add(0x84c) as *mut u8;

        if i_var2 < 0 {
            return 0;
        }
        if slot4.read_unaligned() == i_var2 {
            return 0;
        }
        let i_var4 = FUN_1000d230(i_var2);
        let local_4 = FUN_1000d260(i_var4, i_var2);
        let c_var3 = FUN_1000d230(slot4.read_unaligned());
        let i_var5 = FUN_1000d260(c_var3, slot4.read_unaligned());
        if c_var3 == i_var4 && i_var4 == 1 && local_4 == i_var5 {
            return 0;
        }
        let u_var6 = FUN_1006b9c0(i_var2, 0, &raw mut local_8);
        if (u_var6 as u8) != 1 {
            return 0xffffffff;
        }
        if param_3 == 0 {
            slot4.write_unaligned(slot5.read_unaligned());
            flag.write(1);
            DAT_10083508.write_unaligned(1);
        }
        let c_var3 = FUN_1000d230(i_var2);
        let mut skip = false;
        if c_var3 == 2 {
            if (i_var2 / 100).wrapping_mul(100) != 8000 {
                skip = true;
            }
        } else if param_3 != 0 {
            return local_8;
        }
        if !skip {
            FUN_1006b450(param_4, param_5, param_1, param_2, i_var2);
            flag.write(1);
            DAT_10083508.write_unaligned(1);
        }
        if param_3 == 0 {
            FUN_10010d80(DAT_100aa6c8);
            FUN_1006b9c0(i_var2, 1, &raw mut local_8);
            flag.write(1);
            DAT_10083508.write_unaligned(1);
        }
        local_8
    }
}
