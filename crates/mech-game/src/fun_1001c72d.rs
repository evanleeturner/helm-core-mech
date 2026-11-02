// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c72d(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
) {
    // SAFETY: param_1 points at the game's object; its field at 0x20 is either
    // null or a pointer to a record whose entry array is written where the
    // original does.
    unsafe {
        let i_var2 = (param_1 as u32).wrapping_add(0x20) as *const i32;
        let i_var2 = i_var2.read_unaligned();
        if i_var2 != 0 {
            let count = ((i_var2 as u32).wrapping_add(4) as *const i16).read_unaligned();
            let p = (i_var2 as u32)
                .wrapping_add(0x18)
                .wrapping_add((count as i32).wrapping_mul(0x44) as u32)
                as *mut u32;
            p.write_unaligned(param_2);
            p.add(3).write_unaligned(p.read_unaligned());
            p.add(1).write_unaligned(param_3);
            p.add(4).write_unaligned(p.add(1).read_unaligned());
            p.add(2).write_unaligned(param_4);
            p.add(5).write_unaligned(p.add(2).read_unaligned());
            p.add(6).write_unaligned(param_5);
            p.add(7).write_unaligned(param_6);
            p.add(0xb).write_unaligned(0);
            p.add(0xc).write_unaligned(0);
            p.add(0xd).write_unaligned(0);
            let cnt_ptr = (i_var2 as u32).wrapping_add(4) as *mut i16;
            let cur = cnt_ptr.read_unaligned();
            cnt_ptr.write_unaligned((cur as i32 as u32).wrapping_add(1) as i16);
        }
    }
}
