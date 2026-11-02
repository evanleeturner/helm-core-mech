// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36f0: i32;
    static mut DAT_100b36f4: i32;
    static mut DAT_100b36f8: u32;
    static mut DAT_100b36d4: i32;
    static mut DAT_100f3120: [u32; 0];
    static mut DAT_100af17c: i32;
    static mut DAT_100af0c8: *mut i32;
    static mut DAT_100c7e60: i32;
    static mut DAT_100c7e64: *mut i32;
    static mut DAT_100c7e68: i8;
    static mut _DAT_100c7e69: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10071750(param_1: *mut i32, param_2: i32, param_3: i8, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006088d(param_1: i8, param_2: i32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the source buffer returned by FUN_1008668b and the destination in
    // DAT_100c7e64 are read and written exactly as the original does.
    unsafe {
        if DAT_100b36f0 < 1 && DAT_100b36f4 == 0 {
            DAT_100b36f4 = 1;
            DAT_100b36f8 = (&raw mut DAT_100f3120)
                .cast::<u32>()
                .offset(DAT_100b36d4 as isize)
                .read_unaligned();
            let mut pu_var1 = FUN_1008668b(DAT_100af17c, DAT_100b36f8, DAT_100af0c8);
            if !pu_var1.is_null() {
                FUN_10071750(
                    &raw mut DAT_100c7e60,
                    pu_var1 as usize as i32,
                    param_1,
                    param_2,
                );
                let mut pu_var3 = DAT_100c7e64;
                let mut i_var2: i32 = 0xc0;
                while i_var2 != 0 {
                    pu_var3.write_unaligned(pu_var1.cast::<i32>().read_unaligned());
                    pu_var1 = pu_var1.add(2);
                    pu_var3 = pu_var3.add(1);
                    i_var2 = i_var2.wrapping_sub(1);
                }
                DAT_100c7e68 = 0x80u8 as i8;
                _DAT_100c7e69 = 0x10;
                DAT_100b36f4 = 1;
            }
        }
    }
}
