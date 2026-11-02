// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100984cc: u32;
    static mut DAT_100d08c0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10042210(param_1: i32) -> u8;
    fn FUN_1006e790(param_1: *mut c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100423d0(param_1: *mut i16) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's own record; the hash table and the
    // chain links are the game's memory, accessed where the original does.
    unsafe {
        param_1.byte_add(2).write_unaligned(0);
        FUN_10042210(param_1 as i32);
        let u_var1: u32 = param_1.byte_add(4).cast::<u32>().read_unaligned();
        let c6 = (u_var1 >> 0x10) as u8 as i8;
        let c5 = (u_var1 >> 0x18) as u8 as i8;
        let c7 = (u_var1 >> 8) as u8 as i8;
        let c8 = u_var1 as u8 as i8;
        let sum = (param_1.read_unaligned() as i32)
            .wrapping_add(c6 as i32)
            .wrapping_add(c5 as i32)
            .wrapping_add(c7 as i32)
            .wrapping_add(c8 as i32);
        let i_var4: i32 = sum % 0x3f1;
        let slot = DAT_100984cc.wrapping_add((i_var4 as u32).wrapping_mul(4)) as *mut u32;
        let mut ps3: *mut i16 = slot.read_unaligned() as *mut i16;
        if param_1 == ps3 {
            slot.write_unaligned(param_1.byte_add(8).cast::<u32>().read_unaligned());
        } else {
            if ps3.is_null() {
                return;
            }
            loop {
                let ps2 = ps3.byte_add(8).cast::<u32>().read_unaligned() as *mut i16;
                if ps2.is_null() || param_1 == ps2 {
                    break;
                }
                ps3 = ps2;
                if ps2.is_null() {
                    break;
                }
            }
            if ps3.is_null() {
                return;
            }
            let next = ps3.byte_add(8).cast::<u32>().read_unaligned();
            if next == 0 {
                return;
            }
            let nn = (next.wrapping_add(8) as *const u32).read_unaligned();
            ps3.byte_add(8).cast::<u32>().write_unaligned(nn);
        }
        FUN_1006e790(param_1 as *mut c_void);
        DAT_100d08c0 = DAT_100d08c0.wrapping_sub(1);
    }
}
