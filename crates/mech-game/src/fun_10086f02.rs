// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2dec: u32;
    static mut DAT_100c897c: u8;
    static mut DAT_100c898c: u8;
    static mut DAT_100c2be0: u32;
    static mut DAT_100c23fc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __isctype(_C: i32, _Type: i32) -> i32;
    fn _atoi(_Str: *mut i8) -> i32;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8);
    fn __strcmpi(_Str1: *mut i8, _Str2: *mut i8) -> i32;
    fn FUN_10088ec7(param_1: i32, param_2: u32, param_3: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086f02(param_1: i32, param_2: *mut i8) -> i32 {
    // SAFETY: globals and callees are the game's own; param_2 points at the
    // caller's NUL-terminated string, and the channel tables are read as the
    // original does.
    unsafe {
        let local_c: u32;
        if DAT_100c2dec < 2 {
            let c = param_2.read() as i32;
            let addr = DAT_100c2be0.wrapping_add(c.wrapping_mul(2) as u32);
            local_c = (u32::from((addr as *const u16).read_unaligned())) & 4;
        } else {
            local_c = __isctype(param_2.read() as i32, 4) as u32;
        }
        if local_c == 0 {
            if param_1 == -1 {
                return -1;
            }
            let off = param_1.wrapping_mul(0x74) as isize;
            let p898c = (&raw mut DAT_100c898c).byte_offset(off);
            let p897c = (&raw mut DAT_100c897c).byte_offset(off);
            if p898c.cast::<i32>().read_unaligned() == 0 {
                FUN_100108a0(0x11, &raw mut DAT_100c23fc as *mut i8);
                return -1;
            }
            let mut local_8: i32 = 0;
            while local_8 < p897c.cast::<i32>().read_unaligned() {
                let table = p898c.cast::<i32>().read_unaligned();
                let entry =
                    (table.wrapping_add(local_8.wrapping_mul(4)) as *const i32).read_unaligned();
                if entry != 0 {
                    let table2 = p898c.cast::<i32>().read_unaligned();
                    let entry2 = (table2.wrapping_add(local_8.wrapping_mul(4)) as *const i32)
                        .read_unaligned();
                    let r = __strcmpi(entry2 as *mut i8, param_2);
                    if r == 0 {
                        return local_8;
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
            FUN_10088ec7(
                0x6f,
                param_2 as u32,
                param_1.wrapping_mul(0x74).wrapping_add(0x100c892c) as *mut i8,
            );
            -1
        } else {
            _atoi(param_2)
        }
    }
}
