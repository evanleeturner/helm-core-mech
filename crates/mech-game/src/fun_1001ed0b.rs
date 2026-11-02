// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101e8540: u8;
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
pub unsafe extern "C" fn FUN_1001ed0b(
    param_1: i32,
    param_2: *mut i8,
    param_3: u16,
    param_4: *mut u32,
) -> i32 {
    // SAFETY: the callees and globals are the game's own memory; the computed
    // addresses point into the game's tables exactly as the original reads them.
    unsafe {
        let u_var1 = FUN_1001e55a(param_1, param_2);
        if u_var1 == 0xffff {
            return 0;
        }
        let idx = (u_var1 as i16 as i32) as u32;
        let off = (param_1 as u32).wrapping_mul(0x14a);
        let b40 = (&raw mut DAT_101e8540) as usize as u32;
        let b46 = (&raw mut DAT_101e8546) as usize as u32;
        let b6a = (&raw mut DAT_101e856a) as usize as u32;

        let a1 = b6a.wrapping_add(idx.wrapping_mul(8)).wrapping_add(off);
        if (a1 as usize as *const i32).read_unaligned() == 0 {
            let r = FUN_1001e632(param_1);
            if r as i32 == -1 {
                return 0;
            }
        }
        let local_18 = param_3 as u32;
        let p46 = (b46.wrapping_add(off) as usize as *const u32).read_unaligned();
        let a2 = p46.wrapping_add(0x22).wrapping_add(idx.wrapping_mul(0x18));
        let w = (a2 as usize as *const u16).read_unaligned() as u32;
        let p6a = (a1 as usize as *const u32).read_unaligned();
        let a3 = p6a
            .wrapping_add(0x16)
            .wrapping_add(local_18.wrapping_mul(8));
        let v = (a3 as usize as *const u32).read_unaligned();
        let i_var2 = w.wrapping_add(v) as i32;
        param_4.write_unaligned((b40.wrapping_add(off) as usize as *const u32).read_unaligned());
        i_var2
    }
}
