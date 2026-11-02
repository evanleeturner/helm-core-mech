// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10067af1(param_1: *mut i32) -> i32;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c8e4(param_1: *mut i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's object; the callees are the game's
    // own, and every access is made at the offsets the original uses.
    unsafe {
        param_1.add(0xf1).write_unaligned(0);
        FUN_10067af1(param_1.add(1));
        FUN_10067af1(param_1.add(5));
        if param_1.add(0xe5).read_unaligned() != 0 {
            FUN_1002f668(
                param_1.add(0xe5).read_unaligned(),
                param_1.add(7).read_unaligned(),
                0,
                0,
                0,
            );
        }
        if param_1.add(0xe4).read_unaligned() != 0 {
            let base = param_1.read_unaligned() as u32;
            ((base.wrapping_add(0x68)) as *mut i32).write_unaligned(0);
            ((param_1.read_unaligned() as u32).wrapping_add(0x6c) as *mut i32)
                .write_unaligned(param_1.add(3).read_unaligned());
            ((param_1.read_unaligned() as u32).wrapping_add(0x70) as *mut i32).write_unaligned(0);
            let a = param_1.add(0xe4).read_unaligned();
            let b = ((param_1.read_unaligned() as u32).wrapping_add(0x68) as *const i32)
                .read_unaligned();
            let c = ((param_1.read_unaligned() as u32).wrapping_add(0x6c) as *const i32)
                .read_unaligned();
            let d = ((param_1.read_unaligned() as u32).wrapping_add(0x70) as *const i32)
                .read_unaligned();
            FUN_1002f668(a, b, c, d, 0);
        }
        let p =
            ((param_1.read_unaligned() as u32).wrapping_add(0x40) as *const i32).read_unaligned();
        FUN_1002fbc0(p as *mut i32);
    }
}
