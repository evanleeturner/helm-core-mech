// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab094: u32;
    static mut DAT_100eecb0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100707b3(param_1: i32, param_2: *mut i32) -> *mut i32;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100709b2(param_1: i32, param_2: i32, param_3: *mut u32) {
    let mut param_3 = param_3;
    let mut local_18: [i32; 3] = [0; 3];
    // SAFETY: all pointers are the game's own memory; the offsets and reads
    // mirror the original function exactly.
    unsafe {
        let p1 = param_1 as *const u8;
        let local_8: i32 = p1.byte_add(4).cast::<i32>().read_unaligned();
        let base = (&raw mut DAT_100eecb0) as *mut u8;
        let slot = base.byte_offset(local_8.wrapping_mul(4) as isize);
        let rd84 = || p1.byte_add(0x84).cast::<i32>().read_unaligned();
        let rd88 = || p1.byte_add(0x88).cast::<i32>().read_unaligned();
        let rds = || slot.cast::<i32>().read_unaligned();

        if (rd84() != rd88() && rds() != rd84()) || (rd84() == rd88() && rds() != -2) {
            let local_c: u32;
            if rd84() == 0 && rds() == 2 {
                local_c = (param_2.wrapping_add(4) as *const u32).read_unaligned();
            } else {
                let addr = rd84()
                    .wrapping_add(1)
                    .wrapping_mul(0x10)
                    .wrapping_add(4)
                    .wrapping_add(rd88().wrapping_mul(4))
                    .wrapping_add(param_2);
                local_c = (addr as *const u32).read_unaligned();
            }
            if local_c != 0xffff_ffff {
                if param_3.is_null() {
                    param_3 = FUN_100707b3(param_1, local_18.as_mut_ptr()) as *mut u32;
                }
                let a = param_3.read_unaligned();
                let b = param_3.add(1).read_unaligned();
                let c = param_3.add(2).read_unaligned();
                FUN_100037f0(a, b, c, local_c, DAT_100ab094 as i32);
            }
            if rd84() == rd88() {
                slot.cast::<u32>().write_unaligned(0xffff_fffe);
            } else {
                let v = rd84();
                slot.cast::<i32>().write_unaligned(v);
            }
        }
    }
}
