// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c4c: i32;
    static mut DAT_100b3c9c: [i32; 5];
    static mut DAT_100b3c50: i32;
    static mut _DAT_100b3c58: i32;
    static mut DAT_100b3c54: i32;
    static mut DAT_100b3c48: i32;
    static mut DAT_100aa1ac: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ab7b();
    fn FUN_1006ac3b();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a7a4(param_1: i32, param_2: i32) {
    if -1 < param_1 {
        // SAFETY: the globals and callees are the game's own memory and code;
        // the array pointer addresses elements 0..=4 of the game's table.
        unsafe {
            let arr = &raw mut DAT_100b3c9c as *mut i32;
            match param_1 {
                0 => {
                    DAT_100b3c4c = param_2;
                    arr.add(1).write(param_2);
                }
                1 => {
                    DAT_100b3c50 = param_2;
                    arr.add(2).write(param_2);
                    if param_2 == 0 {
                        _DAT_100b3c58 = _DAT_100b3c58 & 0xfffffffdu32 as i32;
                    } else {
                        _DAT_100b3c58 = _DAT_100b3c58 | 2;
                    }
                }
                2 => {
                    DAT_100b3c54 = param_2;
                    arr.add(3).write(param_2);
                    if param_2 == 0 {
                        arr.add(4).write(arr.add(4).read() & 0xfffffff7u32 as i32);
                        arr.add(4).write(arr.add(4).read() & 0xfffffffbu32 as i32);
                        FUN_1006ac3b();
                    } else {
                        arr.add(4).write(arr.add(4).read() | 8);
                        arr.add(4).write(arr.add(4).read() | 4);
                        if DAT_100aa1ac == 0 {
                            FUN_1006ab7b();
                        }
                    }
                    _DAT_100b3c58 = arr.add(4).read();
                }
                3 => {
                    DAT_100b3c48 = param_2;
                    arr.write(param_2);
                }
                4 => {
                    if param_2 == 0 {
                        arr.add(4).write(arr.add(4).read() & 0xfffffffdu32 as i32);
                    } else {
                        arr.add(4).write(arr.add(4).read() | 2);
                    }
                    _DAT_100b3c58 = arr.add(4).read();
                }
                _ => {}
            }
        }
    }
}
