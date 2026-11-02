// SPDX-License-Identifier: MIT

unsafe extern "C" {}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067af1(param_1: *mut i32) -> i32;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007e8aa(param_1: *mut i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's object; its first field points at
    // another game struct, and all offsets are read and written as the original does.
    unsafe {
        if param_1.add(0xf5).read_unaligned() != 2 {
            return;
        }
        FUN_10067af1(param_1.add(9));
        FUN_10067af1(param_1.add(0x11));
        FUN_10067af1(param_1.add(0xd));
        let b = param_1.read_unaligned() as usize;
        let v = param_1.add(0xb).read_unaligned();
        ((b + 0x50) as *mut i32).write_unaligned(v);
        let b = param_1.read_unaligned() as usize;
        let v = param_1.add(0x13).read_unaligned();
        ((b + 0x54) as *mut i32).write_unaligned(v);
        let b = param_1.read_unaligned() as usize;
        let v = param_1.add(0xf).read_unaligned();
        ((b + 0x58) as *mut i32).write_unaligned(v);

        let b = param_1.read_unaligned() as usize;
        let a1 = ((b + 0x40) as *const i32).read_unaligned();
        let a2 = ((b + 0x50) as *const u32).read_unaligned();
        let a3 = ((b + 0x54) as *const u32).read_unaligned();
        let a4 = ((b + 0x58) as *const u32).read_unaligned();
        FUN_1002f37a(a1, a2, a3, a4);

        let b = param_1.read_unaligned() as usize;
        let a1 = ((b + 0x40) as *const i32).read_unaligned();
        let a2 = ((b + 0x5c) as *const i32).read_unaligned();
        let a3 = ((b + 0x60) as *const i32).read_unaligned();
        let a4 = ((b + 100) as *const i32).read_unaligned();
        FUN_1002f668(a1, a2, a3, a4, 0);

        let b = param_1.read_unaligned() as usize;
        let a1 = ((b + 0x40) as *const i32).read_unaligned();
        FUN_1002fbc0(a1 as *mut i32);

        let b = param_1.read_unaligned() as usize;
        let i_var2 = ((b + 0x60) as *const i32).read_unaligned();
        let i_var1 = FUN_10006b20(i_var2);
        let b = param_1.read_unaligned() as usize;
        ((b + 0xbc) as *mut i32).write_unaligned(i_var1 >> 0xd);
        let r = FUN_10006b9a(i_var2) as i32;
        let b = param_1.read_unaligned() as usize;
        ((b + 0xb8) as *mut i32).write_unaligned(r >> 0xd);
    }
}
