// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> u8;
    fn FUN_10006b20(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067951(
    param_1: i32,
    param_2: *mut i32,
    param_3: u32,
    param_4: i32,
    param_5: *mut i32,
) -> *mut i32 {
    // SAFETY: pointers and globals belong to the game; reads and writes
    // occur where the original performs them.
    unsafe {
        let mut local_14: i32 = param_2.read_unaligned();
        let mut local_10: i32 = param_2.byte_add(4).read_unaligned();
        let w0 = ((param_1 as u32).wrapping_add(0xc) as *const i32).read_unaligned();
        let w1 = ((param_1 as u32).wrapping_add(4) as *const i32).read_unaligned();
        let i_var1 = w0.wrapping_sub(w1).wrapping_add(1) / 2;
        let tbl = DAT_100aaf68.wrapping_add(0x44) as *const i32;
        let mut i_var2 = FUN_10003d60(i_var1.wrapping_sub(1), tbl.read_unaligned());
        i_var2 = i_var2.wrapping_sub(1);
        match param_3 {
            0..=3 => {
                let i_var3 = FUN_10006b9a(param_4) as i32;
                let a = FUN_10003d60(i_var1.wrapping_sub(2), i_var3);
                local_14 = local_14.wrapping_sub(a >> 0xd);
                let b = FUN_10006b20(param_4);
                let c = FUN_10003d60(i_var2, b);
                local_10 = local_10.wrapping_sub(c >> 0xd);
            }
            5 => {
                local_10 = local_10.wrapping_add(i_var2);
            }
            7 => {
                local_10 = local_10.wrapping_sub(i_var2);
            }
            _ => {
                local_10 = 0;
                local_14 = 0;
            }
        }
        param_5.write_unaligned(local_14);
        param_5.byte_add(4).write_unaligned(local_10);
        param_5
    }
}
