// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_100eef40: [i32; 256];
    static mut _DAT_1016a6c8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004c350(param_1: i32, param_2: i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028b0a(
    param_1: i32,
    param_2: u16,
    param_3: i16,
    param_4: i32,
) -> bool {
    // SAFETY: the globals and the callee are the game's own; param_1 is an
    // address of the caller's structure and the table entry points at the
    // game's data, read and written where the original does.
    unsafe {
        let idx = ((param_2 as i16 as i32) & 0xff) as usize;
        let base = (&raw const DAT_100eef40)
            .cast::<i32>()
            .add(idx)
            .read_unaligned();
        let u_var2: u16 =
            (base.wrapping_add(0x14) as u32 as usize as *const u16).read_unaligned() & 0x10;
        let mut local_8: bool = true;
        let s1a6 = (param_1.wrapping_add(0x1a6) as u32 as usize as *const i16).read_unaligned();
        if param_4 == 0 || DAT_100b38fc <= s1a6 as i32 {
            local_8 = u_var2 == 0;
        } else {
            let c0 = (param_1.wrapping_add(0xc0) as u32 as usize as *const i32).read_unaligned();
            if c0 < param_3 as i32
                && (param_2 & 0x200) != 0
                && u_var2 != 0
                && !FUN_1004c350(param_1, 1)
            {
                local_8 = false;
                _DAT_1016a6c8 = _DAT_1016a6c8.wrapping_add(1);
            }
            let v = (DAT_100b38fc as i16 as i32).wrapping_add(0x21f) as i16;
            (param_1.wrapping_add(0x1a6) as u32 as usize as *mut i16).write_unaligned(v);
        }
        local_8
    }
}
