// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2dec: u32;
    static mut DAT_100c8978: u8;
    static mut DAT_100c8984: u8;
    static mut DAT_100c2be0: u32;
    static mut DAT_100c23dc: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __isctype(c: i32, ty: i32) -> i32;
    fn _atoi(s: *const i8) -> i32;
    fn FUN_100108a0(param_1: u32, param_2: *const i8) -> u8;
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_10088ec7(param_1: i32, param_2: u32, param_3: *const i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086d8e(param_1: i32, param_2: *mut i8) -> i32 {
    // SAFETY: globals and callees are the game's own; param_2 points at a
    // NUL-terminated string and the channel tables are read as the original does.
    unsafe {
        let local_c: u32;
        if DAT_100c2dec < 2 {
            let c = param_2.read() as i32;
            let p = DAT_100c2be0.wrapping_add(c.wrapping_mul(2) as u32) as usize as *const u16;
            local_c = (p.read_unaligned() as u32) & 4;
        } else {
            local_c = __isctype(param_2.read() as i32, 4) as u32;
        }
        if local_c == 0 {
            if param_1 == -1 {
                return -1;
            }
            let off = (param_1 as isize).wrapping_mul(0x74);
            let tab = (&raw mut DAT_100c8984).byte_offset(off);
            let cnt = (&raw mut DAT_100c8978).byte_offset(off);
            if tab.cast::<i32>().read_unaligned() == 0 {
                FUN_100108a0(0x11, &raw const DAT_100c23dc);
                return -1;
            }
            let mut local_8: i32 = 0;
            while local_8 < cnt.cast::<i32>().read_unaligned() {
                let arr = tab.cast::<i32>().read_unaligned();
                let elem_addr =
                    arr.wrapping_add(local_8.wrapping_mul(4)) as u32 as usize as *const i32;
                let elem = elem_addr.read_unaligned();
                if elem != 0 {
                    let arr2 = tab.cast::<i32>().read_unaligned();
                    let e2 =
                        arr2.wrapping_add(local_8.wrapping_mul(4)) as u32 as usize as *const i32;
                    let s = e2.read_unaligned() as u32 as usize as *const i8;
                    if __strcmpi(s, param_2) == 0 {
                        return local_8;
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
            FUN_10088ec7(
                0x6f,
                param_2 as usize as u32,
                param_1.wrapping_mul(0x74).wrapping_add(0x100c892c) as u32 as usize as *const i8,
            );
            -1
        } else {
            _atoi(param_2)
        }
    }
}
