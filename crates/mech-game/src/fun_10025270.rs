// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aca24: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10025799(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
    ) -> *mut u32;
    fn FUN_10025417(param_1: *mut i32, param_2: u8, param_3: i32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025270(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; param_1 and the
    // structures reached through it are the caller's memory, accessed as the
    // original does.
    unsafe {
        if DAT_100aca24 != 0 {
            return;
        }
        let i_var1: i32 = ((param_1.wrapping_add(0x1c)) as *const i32).read_unaligned();
        if i_var1 == 0 {
            return;
        }
        let a = ((i_var1.wrapping_add(0x24)) as *const u32).read_unaligned();
        let b = ((i_var1.wrapping_add(0x28)) as *const u32).read_unaligned();
        let c = ((i_var1.wrapping_add(0x2c)) as *const u32).read_unaligned();
        let pi_var2 = FUN_10025799(a, a, b, b, c, c, 0) as *mut i32;
        ((param_1.wrapping_add(0x4c)) as *mut i32).write_unaligned(pi_var2 as usize as i32);
        if pi_var2.is_null() {
            return;
        }
        let mut local_14: i32 = i_var1.wrapping_add(0x18);
        let mut local_18: i32 = 1;
        while local_18 < ((i_var1.wrapping_add(4)) as *const i16).read_unaligned() as i32 {
            let v50 = ((local_14.wrapping_add(0x50)) as *const i32).read_unaligned();
            if pi_var2.add(1).read_unaligned() < v50 {
                pi_var2.add(1).write_unaligned(v50);
            }
            let v50 = ((local_14.wrapping_add(0x50)) as *const i32).read_unaligned();
            if v50 < pi_var2.read_unaligned() {
                pi_var2.write_unaligned(v50);
            }
            let v58 = ((local_14.wrapping_add(0x58)) as *const i32).read_unaligned();
            if pi_var2.add(5).read_unaligned() < v58 {
                pi_var2.add(5).write_unaligned(v58);
            }
            let v58 = ((local_14.wrapping_add(0x58)) as *const i32).read_unaligned();
            if v58 < pi_var2.add(4).read_unaligned() {
                pi_var2.add(4).write_unaligned(v58);
            }
            let v54 = ((local_14.wrapping_add(0x54)) as *const i32).read_unaligned();
            if pi_var2.add(3).read_unaligned() < v54 {
                pi_var2.add(3).write_unaligned(v54);
            }
            let v54 = ((local_14.wrapping_add(0x54)) as *const i32).read_unaligned();
            if v54 < pi_var2.add(2).read_unaligned() {
                pi_var2.add(2).write_unaligned(v54);
            }
            local_14 = local_14.wrapping_add(0x44);
            local_18 = local_18.wrapping_add(1);
        }
        let mut local_1c: i32 = 0;
        while local_1c < 4 {
            let pu_var3 = FUN_10025417(pi_var2, local_1c as u8, i_var1);
            pi_var2
                .add(local_1c as usize)
                .add(7)
                .write_unaligned(pu_var3 as usize as i32);
            local_1c = local_1c.wrapping_add(1);
        }
    }
}
