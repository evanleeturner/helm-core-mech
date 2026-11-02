// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f64a0: u8;
    static mut DAT_101f64a4: u8;
    static mut DAT_101f64a8: u8;
    static mut DAT_101f64ac: u8;
    static mut DAT_101f64b0: u8;
    static mut DAT_101f64b4: u8;
    static mut DAT_101f64b8: u8;
    static mut DAT_101f64bc: u8;
    static mut DAT_101f64c0: u8;
    static mut DAT_101f64c4: u8;
    static mut DAT_101f64c8: u8;
    static mut DAT_101f64cc: u8;
    static mut DAT_101f64d0: u8;
    static mut DAT_101f64d4: u8;
    static mut DAT_101f64d8: u8;
    static mut DAT_101f64dc: u8;
    static mut DAT_101f64e0: u8;
    static mut DAT_101f64e4: u8;
    static mut DAT_101f6518: u8;
}

#[inline(always)]
fn slot(base: *mut u8, off: u32) -> *mut u32 {
    base.wrapping_byte_add(off as usize).cast::<u32>()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d05f(param_1: i32) {
    let i_var1: u32 = (param_1 as u32).wrapping_mul(0x7c);
    // SAFETY: the globals are the game's own arrays; each slot is written and
    // read at the same offset and order as the original.
    unsafe {
        slot(&raw mut DAT_101f64a0, i_var1).write_unaligned(0xffff_ffff);
        slot(&raw mut DAT_101f64a4, i_var1).write_unaligned(0xffff_ffff);
        slot(&raw mut DAT_101f64a8, i_var1).write_unaligned(0xffff_ffff);
        slot(&raw mut DAT_101f64ac, i_var1).write_unaligned(0);
        slot(&raw mut DAT_101f64b0, i_var1).write_unaligned(0);
        slot(&raw mut DAT_101f64b4, i_var1).write_unaligned(0xffff_ffff);
        slot(&raw mut DAT_101f64b8, i_var1).write_unaligned(0xffff_ffff);
        slot(&raw mut DAT_101f64bc, i_var1).write_unaligned(0);
        slot(&raw mut DAT_101f64c0, i_var1).write_unaligned(0);
        slot(&raw mut DAT_101f64cc, i_var1).write_unaligned(1);
        let v = slot(&raw mut DAT_101f64cc, i_var1).read_unaligned();
        slot(&raw mut DAT_101f64c8, i_var1).write_unaligned(v);
        let v = slot(&raw mut DAT_101f64c8, i_var1).read_unaligned();
        slot(&raw mut DAT_101f64c4, i_var1).write_unaligned(v);
        slot(&raw mut DAT_101f64d8, i_var1).write_unaligned(0);
        let v = slot(&raw mut DAT_101f64d8, i_var1).read_unaligned();
        slot(&raw mut DAT_101f64d4, i_var1).write_unaligned(v);
        let v = slot(&raw mut DAT_101f64d4, i_var1).read_unaligned();
        slot(&raw mut DAT_101f64d0, i_var1).write_unaligned(v);
        slot(&raw mut DAT_101f64e4, i_var1).write_unaligned(0);
        let v = slot(&raw mut DAT_101f64e4, i_var1).read_unaligned();
        slot(&raw mut DAT_101f64e0, i_var1).write_unaligned(v);
        let v = slot(&raw mut DAT_101f64e0, i_var1).read_unaligned();
        slot(&raw mut DAT_101f64dc, i_var1).write_unaligned(v);
        slot(&raw mut DAT_101f6518, i_var1).write_unaligned(0);
    }
}
