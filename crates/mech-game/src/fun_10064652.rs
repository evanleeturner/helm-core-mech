// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100adf38: i32;
    static mut DAT_100ba958: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064712(param_1: i32) -> i32;
    fn FUN_10064776(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064652(param_1: i32, param_2: *mut core::ffi::c_void) {
    // SAFETY: the globals and callees are the game's own; the table entry
    // set by FUN_10064712 points at a record the original writes through.
    unsafe {
        let table = (&raw mut DAT_100eef40).cast::<i32>();
        table.offset(param_1 as isize).write(0);
        let b_var2 = FUN_10064712(param_1);
        if b_var2 != 0 {
            let p1 = table.offset(param_1 as isize).read() as usize as *mut i32;
            FUN_10064776(p1);
            p1.add(1).write(param_1);
            if param_1 == DAT_100adf38 {
                p1.add(0x13).write((&raw mut DAT_100ba958) as usize as i32);
            } else {
                p1.add(0x13).write((p1 as usize as i32).wrapping_add(0x206));
            }
            if p1.add(0x13).read() != 0 {
                let mut p4 = p1.add(0x13).read() as usize as *mut i32;
                let mut i_var3: i32 = 0x12;
                while i_var3 != 0 {
                    p4.write_unaligned(0);
                    p4 = p4.add(1);
                    i_var3 = i_var3.wrapping_sub(1);
                }
                (p4 as *mut u16).write_unaligned(0);
            }
            if !param_2.is_null() {
                let f: unsafe extern "C" fn(i32, *mut i32) = core::mem::transmute(param_2);
                f(param_1, p1);
            }
        }
    }
}
