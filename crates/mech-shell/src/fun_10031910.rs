// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031910(param_1: i32) {
    let mut i_var1: i32 = 0;
    loop {
        // SAFETY: DAT_10092000 is the game's own pointer to a structure; the
        // offsets read and written are within it as in the original.
        unsafe {
            let base = DAT_10092000;
            let a = base.wrapping_add(0x2f0).wrapping_add(i_var1 as u32) as *mut i32;
            if a.read_unaligned() == param_1 {
                let a = DAT_10092000.wrapping_add(0x2f0).wrapping_add(i_var1 as u32) as *mut u32;
                a.write_unaligned(0xffffffff);
                let b = DAT_10092000.wrapping_add(0x2f4).wrapping_add(i_var1 as u32) as *mut u32;
                b.write_unaligned(0xffffffff);
            }
        }
        i_var1 = i_var1.wrapping_add(8);
        if i_var1 >= 0x70 {
            break;
        }
    }
}
