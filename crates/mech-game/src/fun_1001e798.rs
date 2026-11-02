// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101e8546: u8;
    static mut DAT_101e856a: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001e55a(param_1: i32, param_2: *mut i8) -> u16;
    fn FUN_1001e632(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001e798(param_1: i32, param_2: *mut i8, param_3: u16) -> i32 {
    if param_3 == 0 {
        return -1;
    }
    // SAFETY: the callees and the globals are the game's own tables; the
    // addresses computed are read exactly where the original reads them.
    unsafe {
        let u_var1 = FUN_1001e55a(param_1, param_2);
        let s = (u_var1 as i16) as i32;
        if s < 0 {
            return -1;
        }
        let slot = (&raw mut DAT_101e856a)
            .cast::<u8>()
            .wrapping_add(s.wrapping_mul(8) as usize)
            .wrapping_add(param_1.wrapping_mul(0x14a) as usize)
            .cast::<i32>();
        if slot.read_unaligned() == 0 && FUN_1001e632(param_1) == 0xFFFF_FFFF {
            return -1;
        }
        let ptr = slot.read_unaligned();
        let a = (ptr
            .wrapping_add(0x1a)
            .wrapping_add((param_3 as i32).wrapping_mul(8)) as usize) as *const i32;
        let tbl = (&raw mut DAT_101e8546)
            .cast::<u8>()
            .wrapping_add(param_1.wrapping_mul(0x14a) as usize)
            .cast::<i32>()
            .read_unaligned();
        let b = (tbl.wrapping_add(0x22).wrapping_add(s.wrapping_mul(0x18)) as usize) as *const u16;
        let r = a
            .read_unaligned()
            .wrapping_sub(b.read_unaligned() as u32 as i32);
        if r < 1 { 0 } else { r }
    }
}
