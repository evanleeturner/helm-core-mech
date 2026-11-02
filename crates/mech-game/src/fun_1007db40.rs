// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40cc: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_10160be4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10053021(param_1: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004839c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007db40(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: u32,
    param_5: u32,
    param_6: i32,
) {
    let _ = (param_1, param_2, param_4, param_5);
    if param_6 == 0 {
        return;
    }
    // SAFETY: param_6 points at the caller's structure; the globals and
    // callees are the game's own, accessed in the original's order.
    unsafe {
        let field = (param_6.wrapping_add(0x10) as *const i32).read_unaligned();
        if (field == param_3 && DAT_10160be4 == 0xd)
            || DAT_10160be4.wrapping_sub(0x31) == param_3 as u32
        {
            ((DAT_100adf34.wrapping_add(1)) as *mut u8).write(0);
            let u_var1 = DAT_100a40cc;
            DAT_100a40cc = 0;
            FUN_10053021(0x71);
            DAT_100a40cc = u_var1;
            FUN_1004839c();
        }
    }
}
