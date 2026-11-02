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
    fn FUN_10090110(param_1: u32, param_2: i32, param_3: u32) -> u32;
    fn FUN_1001e041(param_1: i32, param_2: *mut core::ffi::c_void, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ead0(
    param_1: i32,
    param_2: *mut i8,
    param_3: u16,
    param_4: *mut core::ffi::c_void,
) -> u32 {
    // SAFETY: the globals, callees and the structures they point to are the
    // game's own memory; reads happen in the original's order.
    unsafe {
        let u_var1 = FUN_1001e55a(param_1, param_2);
        if u_var1 == 0xffff {
            return 0xffff_ffff;
        }
        let idx = (u_var1 as i16) as i32;
        let rec = (&raw mut DAT_101e856a)
            .wrapping_byte_offset(idx.wrapping_mul(8) as isize)
            .wrapping_byte_offset(param_1.wrapping_mul(0x14a) as isize)
            as *const i32;
        let tab = (&raw mut DAT_101e8546).wrapping_byte_offset(param_1.wrapping_mul(0x14a) as isize)
            as *const i32;
        let fh = (&raw mut DAT_101e8540).wrapping_byte_offset(param_1.wrapping_mul(0x14a) as isize)
            as *const i32;
        let off8 = (param_3 as i32).wrapping_mul(8) as isize;

        if rec.read_unaligned() == 0 {
            let r = FUN_1001e632(param_1) as i32;
            if r == -1 {
                return 0xffff_ffff;
            }
        }
        let p1 = rec.read_unaligned() as usize as *const u8;
        let p2 = tab.read_unaligned() as usize as *const u8;
        let ent = p2.wrapping_byte_offset(0x22isize.wrapping_add(idx.wrapping_mul(0x18) as isize))
            as *const u16;
        let end = p1.wrapping_byte_offset(0x1a + off8) as *const i32;
        let u_var3 = (end.read_unaligned() as u32).wrapping_sub(ent.read_unaligned() as u32);

        let p1b = rec.read_unaligned() as usize as *const u8;
        let p2b = tab.read_unaligned() as usize as *const u8;
        let entb = p2b.wrapping_byte_offset(0x22isize.wrapping_add(idx.wrapping_mul(0x18) as isize))
            as *const u16;
        let startp = p1b.wrapping_byte_offset(0x16 + off8) as *const i32;
        let pos = (entb.read_unaligned() as i32).wrapping_add(startp.read_unaligned());
        let h = fh.read_unaligned() as u32;
        let d_var4 = FUN_10090110(h, pos, 0);
        if d_var4 == 0xffff_ffff {
            return 0xffff_ffff;
        }
        let u_var6 = FUN_1001e041(fh.read_unaligned(), param_4, u_var3);
        if u_var6 == u_var3 { 0 } else { 0xffff_ffff }
    }
}
