// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_100af0cc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, str2: u32, param_3: *mut i32) -> *mut u16;
    fn __strcmpi(_str1: *mut i8, _str2: *mut i8) -> i32;
    fn FUN_1008664f(param_1: u32, str2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007b890(param_1: u32, str2: *mut i8) -> i32 {
    let mut local_2c: [i8; 12] = [0; 12];
    let mut local_10: i32 = -1;
    // SAFETY: globals and callees are the game's own; the record pointer
    // returned by FUN_1008668b is read at the offsets the original uses.
    unsafe {
        let local_14 = FUN_1008668b(DAT_100af17c as i32, param_1, DAT_100af0cc as *mut i32);
        if !local_14.is_null() {
            let mut local_18 = local_14.add(6);
            for i in 0..10usize {
                let v = 0x100i32.wrapping_sub(local_18.cast::<i8>().add(i).read() as i32);
                local_2c[i] = v as i8;
            }
            let mut local_1c: i32 = 0;
            let local_c = local_14;
            while local_1c < (local_c.add(4).read_unaligned() as i16 as i32)
                && __strcmpi(local_2c.as_mut_ptr(), str2) != 0
            {
                local_18 = local_18.add(6);
                local_1c = local_1c.wrapping_add(1);
                for i in 0..10usize {
                    let v = 0x100i32.wrapping_sub(local_18.cast::<i8>().add(i).read() as i32);
                    local_2c[i] = v as i8;
                }
            }
            if __strcmpi(local_2c.as_mut_ptr(), str2) == 0 {
                local_10 = local_18.add(5).read_unaligned() as i16 as i32;
            }
            FUN_1008664f(param_1, DAT_100af0cc as *mut i32);
        }
    }
    local_10
}
