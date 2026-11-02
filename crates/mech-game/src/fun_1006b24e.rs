// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_100f3120: [u8; 0];
    static mut DAT_100af0c8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
    fn FUN_10060563(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b24e(param_1: i32) {
    let local_8: i32 = if param_1 == 0 { 0x10 } else { 0x11 };
    // SAFETY: the globals and callees are the game's own; the table at
    // DAT_100f3120 is read at the same offset the original uses.
    unsafe {
        let p = (&raw mut DAT_100f3120)
            .cast::<u8>()
            .byte_add(local_8.wrapping_mul(4) as usize)
            .cast::<u32>();
        let puv1 = FUN_1008668b(
            DAT_100af17c as i32,
            p.read_unaligned(),
            DAT_100af0c8 as usize as *mut i32,
        );
        if !puv1.is_null() {
            FUN_1008664f(p.read_unaligned(), DAT_100af0c8 as usize as *mut i32);
            FUN_10060563(local_8);
        }
    }
}
