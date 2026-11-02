// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab378: u32;
    static mut DAT_100ab37c: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_1016acc0: [u32; 0];
    static mut DAT_1016cc50: i32;
    static mut DAT_1016cc60: [u8; 0];
    static mut DAT_1016cc68: [u8; 0];
    static mut DAT_1016ce36: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023a8c(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100241e5() {
    // SAFETY: all globals, the list nodes and the callbacks are the game's
    // own memory; reads, writes and calls happen in the original's order.
    unsafe {
        let mut local_8: i32 = 0;
        if DAT_100ab37c != 0 && DAT_100b3904 != 0 {
            DAT_1016cc50 = 0;
            let mut local_10: i32 = DAT_100ab37c as i32;
            while local_10 != 0 {
                let node = local_10 as usize as *const u8;
                let val = node.byte_add(8).cast::<u32>().read_unaligned();
                let arr = (&raw mut DAT_1016acc0).cast::<u32>();
                arr.offset(DAT_1016cc50 as isize).write_unaligned(val);
                DAT_1016cc50 = DAT_1016cc50.wrapping_add(1);
                local_10 = node.byte_add(4).cast::<i32>().read_unaligned();
            }
            let mut local_c: i32 = 0;
            while local_c < DAT_1016cc50 {
                let arr = (&raw mut DAT_1016acc0).cast::<u32>();
                let idx = arr.offset(local_c as isize).read_unaligned();
                let i_var1 = idx.wrapping_mul(0x1fa) as i32;
                let off = i_var1 as usize;
                let fp_addr = (&raw mut DAT_1016ce36).cast::<u8>().wrapping_add(off);
                if fp_addr.cast::<i32>().read_unaligned() != 0 {
                    let target = fp_addr.cast::<u32>().read_unaligned();
                    let f: unsafe extern "C" fn(*mut u8) -> i32 =
                        core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8) -> i32>(
                            target as usize,
                        );
                    let arg = (&raw mut DAT_1016cc60).cast::<u8>().wrapping_add(off);
                    let i_var2 = f(arg);
                    if i_var2 == 0 {
                        local_8 = local_8.wrapping_add(1);
                        let p = (&raw mut DAT_1016cc68).cast::<u8>().wrapping_add(off);
                        FUN_10023a8c(p.cast::<i32>().read_unaligned());
                    }
                }
                local_c = local_c.wrapping_add(1);
            }
            DAT_100ab378 = DAT_100ab378.wrapping_sub(local_8 as u32);
        }
    }
}
