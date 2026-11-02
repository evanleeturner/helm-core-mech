// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1f4: u32;
    static mut DAT_100ab094: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c203(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> u8;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
    fn FUN_10050ecc(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10080048(param_1: *mut i32) {
    // SAFETY: param_1 and the pointers stored in it and in the game's globals
    // are the game's own live objects; reads, writes and calls happen in the
    // original's order.
    unsafe {
        let base = param_1.read_unaligned() as u32 as usize;
        let cond = DAT_100aa1f4 == 0
            || ((base + 4) as *const i32).read_unaligned() as u32 == DAT_100adf38
            || param_1.add(0xf5).read_unaligned() == 4;
        if cond {
            let base = param_1.read_unaligned() as u32 as usize;
            let f = FUN_10050ecc as unsafe extern "C" fn(i32) -> u8;
            FUN_1007c203(
                ((base + 0x40) as *const i32).read_unaligned() as u32 as usize as *mut i32,
                f as usize as *mut u8,
                999,
            );
            let p = param_1.add(0x110).cast::<u16>();
            p.write_unaligned(p.read_unaligned() | 0x100);
            let base = param_1.read_unaligned() as u32 as usize;
            let g = DAT_100aaf68 as usize;
            let a = ((base + 0x50) as *const i32)
                .read_unaligned()
                .wrapping_sub((g as *const i32).read_unaligned());
            let g = DAT_100aaf68 as usize;
            let base = param_1.read_unaligned() as u32 as usize;
            let b = ((base + 0x54) as *const i32)
                .read_unaligned()
                .wrapping_sub(((g + 4) as *const i32).read_unaligned());
            let g = DAT_100aaf68 as usize;
            let base = param_1.read_unaligned() as u32 as usize;
            let c = ((base + 0x58) as *const i32)
                .read_unaligned()
                .wrapping_sub(((g + 8) as *const i32).read_unaligned());
            FUN_100037f0(a as u32, b as u32, c as u32, 0xa9, DAT_100ab094 as i32);
        }
    }
}
