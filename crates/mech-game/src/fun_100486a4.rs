// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_101686f0: u32;
    static mut DAT_100af0b4: u32;
    static mut DAT_100af0b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100486a4(param_1: i32) {
    let base = param_1 as *mut u8;
    // SAFETY: param_1 points at the caller's record; the globals and the
    // callee are the game's own. Reads, writes and calls keep the original order.
    unsafe {
        let v10 = base.byte_add(0x10).cast::<i32>().read_unaligned();
        if v10 == -1 {
            base.byte_add(0x14).cast::<u32>().write_unaligned(0);
        } else {
            let p = FUN_1008668b(
                DAT_100af17c as i32,
                (v10 as u32).wrapping_add(DAT_101686f0),
                DAT_100af0b4 as *mut i32,
            );
            base.byte_add(0x14).cast::<*mut u16>().write_unaligned(p);
        }
        let v1c = base.byte_add(0x1c).cast::<i32>().read_unaligned();
        if v1c == -1 {
            base.byte_add(0x20).cast::<u32>().write_unaligned(0);
        } else {
            let p = FUN_1008668b(
                DAT_100af17c as i32,
                (v1c as u32).wrapping_add(DAT_101686f0),
                DAT_100af0b4 as *mut i32,
            );
            base.byte_add(0x20).cast::<*mut u16>().write_unaligned(p);
        }
        let v2c = base.byte_add(0x2c).cast::<i32>().read_unaligned();
        let p = FUN_1008668b(
            DAT_100af17c as i32,
            (v2c as u32).wrapping_add(DAT_101686f0),
            DAT_100af0b8 as *mut i32,
        );
        base.byte_add(0x30).cast::<*mut u16>().write_unaligned(p);
    }
}
