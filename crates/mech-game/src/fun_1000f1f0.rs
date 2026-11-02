// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9d6c: [i8; 1];
    static mut DAT_100a9d70: [i8; 1];
    static mut DAT_100adf38: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100b2ce8: u32;
    static mut DAT_100eebbe: u8;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_101686f0: u32;
    static mut DAT_100af0b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
    fn FUN_10066609(param_1: *mut i32, param_2: i32) -> u8;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller guarantees addr is a readable address in the game's memory.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f1f0(param_1: i32) {
    // SAFETY: all addresses are the game's own structures and globals; reads
    // and calls happen in the original's order.
    unsafe {
        if rd(param_1.wrapping_add(0xc)) > -1 && DAT_100b2ce8 != 0 {
            let idx = DAT_100adf38 as usize;
            let entry = (&raw const DAT_100eef40)
                .cast::<u32>()
                .add(idx)
                .read_unaligned() as i32;
            let i_var1 = rd(entry.wrapping_add(0x20));
            let local_8 = rd(param_1.wrapping_add(0xc))
                .wrapping_mul(0x40)
                .wrapping_add(rd(i_var1.wrapping_add(0x54)));
            if rd(local_8.wrapping_add(4)) > -1 {
                let mut local_50: i32 = 0;
                match rd(local_8.wrapping_add(8)) {
                    0 => local_50 = 0xb,
                    1 => match rd(local_8.wrapping_add(0xc)) {
                        0 => local_50 = 0xe,
                        1 => local_50 = 0xfe,
                        2 => local_50 = 3,
                        3 => local_50 = 5,
                        4 => local_50 = 7,
                        _ => {}
                    },
                    2 => local_50 = 0xe,
                    -1 => local_50 = 8,
                    _ => local_50 = 0xb,
                }
                let mut local_48 = [0u8; 64];
                let puv = FUN_1008668b(
                    DAT_100af17c as i32,
                    DAT_101686f0.wrapping_add(1),
                    DAT_100af0b8 as usize as *mut i32,
                );
                if !puv.is_null() {
                    DAT_100eebbe = local_50 as u8;
                    if rd(local_8.wrapping_add(0x20)) == 9999 {
                        sprintf(
                            local_48.as_mut_ptr().cast::<i8>(),
                            (&raw const DAT_100a9d6c).cast::<i8>(),
                            param_1.wrapping_add(0x10),
                        );
                        let p = rd(param_1.wrapping_add(0xa8));
                        FUN_1005acaa(
                            rd(param_1.wrapping_add(0xa4)) as usize as *mut i32,
                            rd(p),
                            rd(p.wrapping_add(4)),
                            puv as usize as i32,
                            local_48.as_mut_ptr(),
                            0x100eebb0,
                        );
                    } else {
                        let v = rd(i_var1
                            .wrapping_add(0x66)
                            .wrapping_add(rd(local_8.wrapping_add(0x20)).wrapping_mul(0x3a)));
                        sprintf(
                            local_48.as_mut_ptr().cast::<i8>(),
                            (&raw const DAT_100a9d70).cast::<i8>(),
                            param_1.wrapping_add(0x10),
                            v,
                        );
                        let p = rd(param_1.wrapping_add(0xa8));
                        FUN_1005acaa(
                            rd(param_1.wrapping_add(0xa4)) as usize as *mut i32,
                            rd(p),
                            rd(p.wrapping_add(4)),
                            puv as usize as i32,
                            local_48.as_mut_ptr(),
                            0x100eebb0,
                        );
                    }
                    DAT_100eebbe = 0xe;
                    FUN_1008664f(
                        DAT_101686f0.wrapping_add(1),
                        DAT_100af0b8 as usize as *mut i32,
                    );
                }
                if rd(param_1.wrapping_add(0xc)) == rd(i_var1.wrapping_add(0x3e0)) {
                    FUN_10066609(
                        rd(param_1.wrapping_add(0xa4)) as usize as *mut i32,
                        local_50,
                    );
                }
            }
        }
    }
}
