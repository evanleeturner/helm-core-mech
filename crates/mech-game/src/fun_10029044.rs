// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut _DAT_1016ac70: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: u16);
    fn FUN_10043f2c(param_1: i32);
    fn FUN_1002954c(param_1: u16) -> i32;
    fn FUN_10044054(param_1: *mut i32) -> u8;
    fn FUN_100295f4(param_1: *mut i32, param_2: i32) -> u64;
    fn FUN_100297b4(param_1: i32) -> i32;
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32);
    fn FUN_1002c41d(param_1: *mut i32);
    fn FUN_1002a8fe(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10029044(param_1: *mut i32, param_2: u16) {
    // SAFETY: param_1 points at the game's object; the offsets read and written
    // are the ones the original uses, and the callees and globals are the game's own.
    unsafe {
        FUN_1002950f(param_1 as i32, param_2);
        FUN_10043f2c(param_1 as i32);
        let i_var2 = FUN_1002954c(param_2);
        _DAT_1016ac70 = DAT_100b38fc;
        let sel = param_1.byte_add(0x176).cast::<i16>().read_unaligned();
        match sel {
            2 => {
                let p = param_1.add(0x13).read_unaligned();
                ((p.wrapping_add(0x25)) as *mut u8).write(0);
                let p = param_1.add(0x13).read_unaligned();
                ((p.wrapping_add(0x27)) as *mut u8).write(0);
                let b = FUN_10044054(param_1);
                let local_8;
                if b == 0 {
                    let u = FUN_100295f4(param_1, i_var2);
                    let p = param_1.add(0x13).read_unaligned();
                    ((p.wrapping_add(8)) as *mut i32).write_unaligned(u as i32);
                    local_8 = FUN_100297b4(param_1 as i32);
                } else {
                    local_8 = FUN_1002a132(param_1 as i32);
                }
                FUN_10022b70(param_1 as i32, local_8);
            }
            4 => {
                let v = param_1.add(0x35).read_unaligned() as u32;
                param_1
                    .add(0x35)
                    .write_unaligned((v.wrapping_add(0xb40000) % 0x1680000) as i32);
                let b = FUN_10044054(param_1);
                if b == 0 {
                    let p = param_1.add(0x13).read_unaligned();
                    ((p.wrapping_add(8)) as *mut i32).write_unaligned(0x400);
                    FUN_100297b4(param_1 as i32);
                }
                _DAT_1016ac70 = (_DAT_1016ac70 as u32).wrapping_add(0xb5) as i32;
            }
            5 => {
                FUN_1002c41d(param_1);
                let b = FUN_10044054(param_1);
                if b == 0 {
                    let u = FUN_100295f4(param_1, i_var2);
                    let p = param_1.add(0x13).read_unaligned();
                    ((p.wrapping_add(8)) as *mut i32).write_unaligned(u as i32);
                    FUN_100297b4(param_1 as i32);
                }
                FUN_1002a8fe(param_1 as i32);
                _DAT_1016ac70 = (_DAT_1016ac70 as u32).wrapping_add(0xb5) as i32;
            }
            6 | 7 | 8 => {
                FUN_1002c41d(param_1);
                let b = FUN_10044054(param_1);
                if b == 0 {
                    let u = FUN_100295f4(param_1, i_var2);
                    let p = param_1.add(0x13).read_unaligned();
                    ((p.wrapping_add(8)) as *mut i32).write_unaligned(u as i32);
                    FUN_100297b4(param_1 as i32);
                }
                _DAT_1016ac70 = (_DAT_1016ac70 as u32).wrapping_add(0xb5) as i32;
            }
            _ => {
                if param_1.add(4).read_unaligned() == 2 {
                    let p = param_1.add(0x13).read_unaligned();
                    ((p.wrapping_add(8)) as *mut i32).write_unaligned(0x333);
                }
            }
        }
    }
}
