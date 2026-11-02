// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100bac8d: u8;
    static mut DAT_100a6798: u32;
    static mut DAT_100d08e8: u32;
    static mut DAT_1009bd18: i8;
    static mut DAT_1009bd20: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bbb0(param_1: *mut i8) -> *mut u8;
    fn FUN_1005b1b0(param_1: *mut i8);
    fn FUN_10038710(param_1: *mut i8) -> u32;
}

unsafe extern "stdcall" {
    fn PostMessageA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004e850() -> u32 {
    // SAFETY: globals and callees are the game's own; all pointers are
    // derived from the game's state structure and read/written where the
    // original does.
    unsafe {
        let base = DAT_10083508;
        let field_23bb = ((base.wrapping_add(0x23bb)) as *const i32).read_unaligned();
        if field_23bb != 1 {
            return base & 0xffffff00;
        }

        if DAT_100bac8d == 4 {
            return base & 0xffffff00;
        }

        let field_1fe = ((DAT_10083508.wrapping_add(0x1fe)) as *const i32).read_unaligned();
        if field_1fe == 4 {
            return base & 0xffffff00;
        }

        let p = DAT_100a6798;
        if ((p.wrapping_add(4)) as *const i32).read_unaligned() != 0 {
            return p & 0xffffff00;
        }

        let mut pb = FUN_1005bbb0(DAT_10083508.wrapping_add(0x1d6) as *mut i8) as *const u8;
        let mut pc = &raw mut DAT_1009bd18 as *const i8;
        let mut i_var3: i32 = 0;
        loop {
            let mut b = *pb;
            let c = *pc;
            if (b as i32) != (c as i32) {
                let lt = (b as i32) < (c as u8 as i32);
                i_var3 = (1 - lt as i32) - (lt as i32);
                break;
            }
            if b == 0 {
                break;
            }
            b = *pb.add(1);
            let c1 = *pc.add(1);
            if (b as i32) != (c1 as i32) {
                let lt = (b as i32) < (c1 as u8 as i32);
                i_var3 = (1 - lt as i32) - (lt as i32);
                break;
            }
            pb = pb.add(2);
            pc = pc.add(2);
            if b == 0 {
                break;
            }
        }

        if i_var3 == 0 {
            let r = PostMessageA(DAT_100d08e8, 0xc02b, 0xc00a, 0);
            return ((r as u32) & 0xffffff00) | 1;
        }

        FUN_1005b1b0(&raw mut DAT_1009bd20);
        PostMessageA(DAT_100d08e8, 0xc01c, 0xc00a, 0);
        let u_var5 = FUN_10038710(DAT_10083508.wrapping_add(0x1d6) as *mut i8);
        *(DAT_10083508.wrapping_add(0x239d) as *mut u8) = 1;
        *(DAT_10083508.wrapping_add(0x239c) as *mut u8) = 0;
        *(DAT_10083508.wrapping_add(0x23a2) as *mut u8) = 0;
        (u_var5 & 0xffffff00) | 1
    }
}
