// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_10092020: u8;
    static mut DAT_10092024: u8;
    static mut DAT_10092298: [u8; 0];
    static mut DAT_10092618: [u8; 0];
    static mut DAT_10092dc0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063ea0(param_1: *mut i32) -> u8;
    fn FUN_10063d30(param_1: *mut i32) -> u8;
    fn FUN_10031e40(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032e50(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's object, and DAT_10083508 holds an address of a game struct.
    unsafe {
        let field = (param_1 as u32).wrapping_add(0x24) as *const i32;
        if field.read_unaligned() == 0 {
            DAT_10092024 = 1;
            DAT_10092020 = 0;
            FUN_10063ea0(&raw mut DAT_10092618 as *mut i32);
            FUN_10063d30(&raw mut DAT_10092298 as *mut i32);
            FUN_10063ea0(&raw mut DAT_10092dc0 as *mut i32);
            FUN_10063d30(&raw mut DAT_10092dc0 as *mut i32);
            return;
        }
        if (param_1 as u32).wrapping_add(0x24) as *const i32 == field && field.read_unaligned() == 9
        {
            DAT_10092020 = 1;
            DAT_10092024 = 0;
            FUN_10063ea0(&raw mut DAT_10092298 as *mut i32);
            FUN_10063ea0(&raw mut DAT_10092dc0 as *mut i32);
            FUN_10063ea0(&raw mut DAT_10092618 as *mut i32);
            FUN_10063d30(&raw mut DAT_10092618 as *mut i32);
        } else {
            DAT_10092020 = 0;
            DAT_10092024 = 0;
            FUN_10063ea0(&raw mut DAT_10092618 as *mut i32);
            FUN_10063ea0(&raw mut DAT_10092298 as *mut i32);
            FUN_10063ea0(&raw mut DAT_10092dc0 as *mut i32);
            FUN_10063d30(&raw mut DAT_10092dc0 as *mut i32);
            let u_var1 = FUN_10031e40(param_1);
            if (u_var1 as u8) == 1
                && ((DAT_10083508.wrapping_add(0x23bb)) as *const i32).read_unaligned() == 1
            {
                let p = (param_1 as u32).wrapping_add(0x28) as *mut i32;
                p.write_unaligned(p.read_unaligned().wrapping_sub(1));
            }
        }
    }
}
