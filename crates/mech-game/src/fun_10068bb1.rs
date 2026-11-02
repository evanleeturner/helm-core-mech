// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100f3040: u8;
    static mut DAT_10114ec8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_10003fc7(param_1: i32, param_2: *mut i32) -> *mut u32;
    fn FUN_100682e7(param_1: i32) -> u8;
}

unsafe fn copy5(src: *const u32, dst: *mut u32) {
    let mut i = 0;
    while i < 5 {
        // SAFETY: caller guarantees both point at five dwords.
        unsafe {
            dst.add(i).write_unaligned(src.add(i).read_unaligned());
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068bb1(param_1: i32) {
    let p = param_1 as usize;
    let mut local_34: [u32; 5] = [0; 5];
    let mut local_20: [u32; 5] = [0; 5];
    // SAFETY: param_1 points at the game's object; globals and callees are
    // the game's own; copies touch five-dword records as the original does.
    unsafe {
        if ((((p + 4) as *const i16).read_unaligned() != 0)
            && (((p + 0x30) as *const i32).read_unaligned() != 0)
            && (((p + 0x30) as *const i32).read_unaligned() != 2)
            && (((p + 0x30) as *const i32).read_unaligned() != 1))
        {
            let local_c = ((p + 0xac) as *const *mut i32).read_unaligned();
            if !local_c.is_null() {
                let b0 = ((p + 0xb0) as *const i32).read_unaligned();
                if b0 != 0 && b0 != 3 && b0 != 4 {
                    FUN_10003d80(local_c);
                }
                let local_8 = FUN_10003fc7(1, local_c);
                if !local_8.is_null() {
                    let base = &raw mut DAT_100f3040 as usize;
                    let idx = ((p + 0xc) as *const i32).read_unaligned() as usize;
                    let slot = base.wrapping_add(idx.wrapping_mul(0x14)) as *mut u32;
                    copy5(slot, local_20.as_mut_ptr());
                    let a4 = ((p + 0xa4) as *const *mut u32).read_unaligned();
                    copy5(a4, local_34.as_mut_ptr());
                    let idx = ((p + 0xc) as *const i32).read_unaligned() as usize;
                    let slot = base.wrapping_add(idx.wrapping_mul(0x14)) as *mut u32;
                    copy5(local_8, slot);
                    let a4 = ((p + 0xa4) as *const *mut u32).read_unaligned();
                    copy5(local_8, a4);
                    FUN_100682e7(param_1);
                    let idx = ((p + 0xc) as *const i32).read_unaligned() as usize;
                    let slot = base.wrapping_add(idx.wrapping_mul(0x14)) as *mut u32;
                    copy5(local_20.as_ptr(), slot);
                    let a4 = ((p + 0xa4) as *const *mut u32).read_unaligned();
                    copy5(local_34.as_ptr(), a4);
                }
            }
            ((p + 0xb0) as *mut u32).write_unaligned(DAT_10114ec8);
        }
    }
}
