// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10090420: u32;
    static mut DAT_10090428: u32;
    static mut DAT_10090430: u32;
    static mut DAT_1009042c: u32;
    static mut DAT_10090518: u32;
    static mut DAT_1009051c: u32;
    static mut DAT_10090520: u32;
    static mut DAT_100904b0: u32;
    static mut DAT_1001e6f0: [Option<unsafe extern "C" fn()>; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f35f(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    param_8: u32,
) {
    // SAFETY: the globals and the handler table are the game's own memory;
    // param_1 points at a pointer to the caller's u32 pair, read as the original does.
    unsafe {
        let p = param_1.read_unaligned() as usize as *const u32;
        DAT_10090420 = p.add(1).read_unaligned().wrapping_add(1);
        let p = param_1.read_unaligned() as usize as *const u32;
        DAT_10090428 = p.read_unaligned();
        DAT_10090430 = param_3.wrapping_mul(0x18).wrapping_add(param_2) as u32;
        DAT_1009042c = param_2 as u32;
        DAT_10090518 = param_6;
        DAT_1009051c = param_7;
        DAT_10090520 = param_8;
        DAT_100904b0 = param_5;
        let table = (&raw mut DAT_1001e6f0).cast::<Option<unsafe extern "C" fn()>>();
        let entry = table.offset(param_4 as isize).read_unaligned();
        if let Some(f) = entry {
            f();
        }
    }
}
