// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_100b9a18: u32;
    static mut DAT_100f3120: [u8; 0];
    static mut DAT_100af0c8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060563(param_1: i32) {
    // SAFETY: the globals and callees are the game's own memory; the table
    // entry and the vtable slot are read where the original reads them.
    unsafe {
        let pu_var1 = (&raw mut DAT_100f3120)
            .cast::<u8>()
            .byte_offset(param_1.wrapping_mul(4) as isize)
            .cast::<u32>();
        if 0 < (pu_var1.read_unaligned() as i32) {
            let pu_var2 = FUN_1008668b(
                DAT_100af17c as i32,
                pu_var1.read_unaligned(),
                DAT_100af0c8 as *mut i32,
            );
            if !pu_var2.is_null() {
                let vtable = DAT_100b9a18 as *const u8;
                let target = vtable.byte_add(0x18).cast::<u32>().read_unaligned();
                let f: unsafe extern "C" fn(*mut u16) =
                    core::mem::transmute::<usize, unsafe extern "C" fn(*mut u16)>(target as usize);
                f(pu_var2);
                FUN_1008664f(pu_var1.read_unaligned(), DAT_100af0c8 as *mut i32);
            }
        }
    }
}
