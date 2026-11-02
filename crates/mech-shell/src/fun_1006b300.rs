// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000cfe0(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b300(param_1: i32, param_2: i32, param_3: i32, param_4: i8) {
    let i_var3 = param_3.wrapping_mul(0x12);
    let i_var1 = i_var3.wrapping_add(0x760).wrapping_add(param_1);
    let i_var2 = param_2.wrapping_add(0x760).wrapping_add(i_var3);
    // SAFETY: the addresses are offsets into the caller's game structures,
    // read and written where the original does; the callee and global are the
    // game's own.
    unsafe {
        let a = (param_2.wrapping_add(0x76c).wrapping_add(i_var3) as *const i32).read_unaligned();
        let b = (i_var3.wrapping_add(0x768).wrapping_add(param_1) as *const i32).read_unaligned();
        let c = (i_var1.wrapping_add(0xc) as *const i32).read_unaligned();
        let d = (i_var2.wrapping_add(8) as *const i32).read_unaligned();
        let i_var3 = a.wrapping_sub(b).wrapping_sub(c).wrapping_add(d);
        FUN_1000cfe0(i_var3);
        if param_4 != 1 {
            let v8 = (i_var2.wrapping_add(8) as *const u32).read_unaligned();
            (i_var1.wrapping_add(8) as *mut u32).write_unaligned(v8);
            let vc = (i_var2.wrapping_add(0xc) as *const u32).read_unaligned();
            (i_var1.wrapping_add(0xc) as *mut u32).write_unaligned(vc);
            if 0 < i_var3 {
                (param_1.wrapping_add(0x84c) as *mut u8).write(1);
                DAT_10083508.write(1);
            }
        }
    }
}
