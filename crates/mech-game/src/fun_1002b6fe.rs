// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: u32;
    static mut DAT_101600e4: u8;
    static mut DAT_101600e8: u8;
    static mut DAT_101600fc: u8;
    static mut DAT_1016010c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b6fe(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut u_var2: u16 = 6;
    if param_3 == 0 {
        u_var2 = 0x16;
    }
    let i_var1: i32 = param_1.wrapping_mul(0x3c);
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    // SAFETY: all globals are the game's own tables; table entries point at
    // game records whose u16 at offset 0x14 is read, and param_2 is the
    // caller's output array written where the original does.
    unsafe {
        let tbl = (&raw mut DAT_100eef40).cast::<u32>();
        let b_e4 = (&raw mut DAT_101600e4).cast::<u8>();
        let b_e8 = (&raw mut DAT_101600e8).cast::<u8>();
        let b_fc = (&raw mut DAT_101600fc).cast::<u8>();
        let b_10c = (&raw mut DAT_1016010c).cast::<u8>();
        while local_10
            < b_e4
                .byte_offset(i_var1 as isize)
                .cast::<i32>()
                .read_unaligned()
        {
            let idx = b_fc
                .byte_offset(local_10.wrapping_mul(4).wrapping_add(i_var1) as isize)
                .cast::<i32>()
                .read_unaligned();
            let entry = tbl.offset(idx as isize).read_unaligned();
            let flags = (entry.wrapping_add(0x14) as usize as *const u16).read_unaligned();
            if (u_var2 & flags) == 0 {
                let idx2 = b_fc
                    .byte_offset(local_10.wrapping_mul(4).wrapping_add(i_var1) as isize)
                    .cast::<i32>()
                    .read_unaligned();
                let v = tbl.offset(idx2 as isize).read_unaligned();
                (param_2.wrapping_add(local_14.wrapping_mul(4)) as usize as *mut u32)
                    .write_unaligned(v);
                local_14 = local_14.wrapping_add(1);
            }
            local_10 = local_10.wrapping_add(1);
        }
        if b_e8
            .byte_offset(i_var1 as isize)
            .cast::<i32>()
            .read_unaligned()
            != 0
        {
            let idx = b_10c
                .byte_offset(i_var1 as isize)
                .cast::<i32>()
                .read_unaligned();
            let entry = tbl.offset(idx as isize).read_unaligned();
            let flags = (entry.wrapping_add(0x14) as usize as *const u16).read_unaligned();
            if (u_var2 & flags) == 0 {
                let idx2 = b_10c
                    .byte_offset(i_var1 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                let v = tbl.offset(idx2 as isize).read_unaligned();
                (param_2.wrapping_add(0x10) as usize as *mut u32).write_unaligned(v);
            }
        }
    }
    local_14
}
