// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001b82d(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100141fb(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut i32,
) -> i32 {
    // SAFETY: pointers are the game's own memory addressed by the integer
    // parameters, read and written in the original's order; callee and globals
    // belong to the game.
    unsafe {
        let off = ((param_1 as u32).wrapping_add(0) as usize) as u32;
        let base = ((param_1.wrapping_add(4)) as u32) as *const i32;
        let idx = ((base.read_unaligned()).wrapping_add(param_1) as u32) as *const u8;
        let _ = off;
        let i_var2 = (u32::from(idx.read()).wrapping_mul(0x44)).wrapping_add(param_2 as u32);
        let i_var1 = ((i_var2.wrapping_add(0x10)) as *const i32).read_unaligned();
        let a = (param_1.wrapping_add(0x14) as *const i32).read_unaligned();
        let b = (param_1.wrapping_add(0x18) as *const i32).read_unaligned();
        let c = (param_1.wrapping_add(0x1c) as *const i32).read_unaligned();
        let d = (param_3 as u32)
            .wrapping_sub(((i_var2.wrapping_add(0xc)) as *const u32).read_unaligned())
            as i32;
        let e = (param_5 as u32)
            .wrapping_sub(((i_var2.wrapping_add(0x14)) as *const u32).read_unaligned())
            as i32;
        let i_var2 = FUN_1001b82d(a, b, c, 0, d, e);
        param_6.write_unaligned((i_var1 as u32).wrapping_sub(i_var2 as u32) as i32);
        DAT_100aa540 = (param_1.wrapping_add(0x14) as *const i32).read_unaligned() >> 0xd;
        DAT_100aa544 = (param_1.wrapping_add(0x18) as *const i32).read_unaligned() >> 0xd;
        DAT_100aa548 = (param_1.wrapping_add(0x1c) as *const i32).read_unaligned() >> 0xd;
        (param_4 < param_6.read_unaligned()) as i32
    }
}
