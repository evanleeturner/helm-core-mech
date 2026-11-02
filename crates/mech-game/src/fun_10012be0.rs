// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_100aa25c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "sprintf"]
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10012be0(param_1: i32, param_2: i32) {
    let mut local_68: [i8; 100] = [0; 100];
    // SAFETY: the globals, callees and param_1 record are the game's own memory;
    // the buffer is the local 100-byte array, written where the original does.
    unsafe {
        let entry = (&raw mut DAT_100eef40)
            .cast::<u32>()
            .offset(param_2 as isize)
            .read_unaligned();
        if entry != 0 {
            (param_1.wrapping_add(0x52) as *mut u8).write(0);
            let entry = (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(param_2 as isize)
                .read_unaligned();
            _sprintf(
                local_68.as_mut_ptr(),
                (&raw const DAT_100aa25c).cast::<i8>(),
                entry.wrapping_add(0x110),
                param_1.wrapping_add(2),
            );
            FUN_1005be24(local_68.as_mut_ptr(), 1, 0x43e, 0x32);
            FUN_10003742(0xf0, 100, 0x40, 5, 0x32);
        }
    }
}
