// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut u8;
    static mut DAT_10168410: [u8; 0];
    static mut DAT_10168450: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut u8, flags: u32, mem: *mut u8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086951(param_1: u32) -> u32;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8);
    fn FUN_100013e0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032095(param_1: *mut u32) -> u32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's record and source bytes, and the table slots are
    // read and written where the original does.
    unsafe {
        let mut local_8: u32 = 0;
        let i_var1 = param_1.add(2).read_unaligned() as i32;
        let off = (i_var1 as isize).wrapping_mul(4);
        let slot = (&raw mut DAT_10168410)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<u32>();
        let slot2 = (&raw mut DAT_10168450)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<u32>();

        if slot.read_unaligned() != 0 {
            HeapFree(DAT_100adf74, 1, slot.read_unaligned() as usize as *mut u8);
        }
        slot.write_unaligned(0);
        slot2.write_unaligned(0);
        let u_var2 = FUN_10086951(param_1.add(1).read_unaligned());
        slot.write_unaligned(u_var2);
        if slot.read_unaligned() == 0 {
            FUN_100108a0(0x40, core::ptr::null_mut());
        } else {
            local_8 = 1;
            let mut u_var4 = param_1.add(1).read_unaligned();
            let mut src = param_1;
            let mut dst = slot.read_unaligned() as usize as *mut u32;
            let mut u_var3 = u_var4 >> 2;
            while u_var3 != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                u_var3 = u_var3.wrapping_sub(1);
            }
            let mut bsrc = src.cast::<u8>();
            let mut bdst = dst.cast::<u8>();
            u_var4 &= 3;
            while u_var4 != 0 {
                bdst.write(bsrc.read());
                bsrc = bsrc.add(1);
                bdst = bdst.add(1);
                u_var4 = u_var4.wrapping_sub(1);
            }
            let rec = slot.read_unaligned() as usize as *const u8;
            let v = rec.byte_add(4).cast::<i32>().read_unaligned() as u32;
            slot2.write_unaligned(v.wrapping_sub(8) / 0x97);
            FUN_100013e0(slot.read_unaligned() as i32);
        }
        local_8
    }
}
