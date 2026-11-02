// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16);
    fn FUN_1002a132(param_1: i32) -> u32;
    fn FUN_10022b70(param_1: i32, param_2: u32);
    fn FUN_10044054(param_1: *mut i32) -> i32;
    fn FUN_100297b4(param_1: i32) -> i32;
    fn FUN_100295f4(param_1: *mut i32, param_2: i32) -> i32;
    fn FUN_10044a83(param_1: i32);
    fn FUN_1004318b(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100432c1(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object; all offsets read and
    // written are the ones the original uses, and the callees are the game's own.
    unsafe {
        let obj = param_1 as usize as i32;

        FUN_1002950f(obj, param_1.byte_add(0x17a).cast::<i16>().read_unaligned());
        let u_var2 = FUN_1002a132(obj);
        FUN_10022b70(obj, u_var2);

        if param_1.add(0x76).read_unaligned() == 0 {
            FUN_1002950f(obj, param_1.byte_add(0x178).cast::<i16>().read_unaligned());
            if FUN_10044054(param_1) == 0 {
                FUN_100297b4(obj);
                let r = FUN_100295f4(param_1, 4000);
                let p = param_1.add(0x13).read_unaligned() as usize as *mut u8;
                p.byte_add(8).cast::<i32>().write_unaligned(r);
            }
            let sign = ((u_var2 as i32) >> 0x1f) as u32;
            let p = param_1.add(0x13).read_unaligned() as usize as *mut u8;
            if p.byte_add(8).cast::<i32>().read_unaligned() == 0
                && 0x50000 < ((u_var2 ^ sign).wrapping_sub(sign) as i32)
            {
                param_1.add(0x76).write_unaligned(1);
            }
            FUN_10044a83(obj);
        }

        if param_1.add(0x76).read_unaligned() == 1 {
            let local_8 = if FUN_10044054(param_1) == 0 {
                FUN_100297b4(obj)
            } else {
                FUN_1002a132(obj) as i32
            };
            let i_var3 = FUN_1004318b(obj, local_8);
            if i_var3 == 0 {
                let p = param_1.add(0x13).read_unaligned() as usize as *mut u8;
                p.byte_add(8).cast::<i32>().write_unaligned(0);
            } else {
                FUN_10044a83(obj);
            }
            FUN_1002950f(obj, param_1.byte_add(0x178).cast::<i16>().read_unaligned());
            if 0x1194 < param_1.add(0x30).read_unaligned() {
                param_1.add(0x76).write_unaligned(0);
            }
        }
    }
}
