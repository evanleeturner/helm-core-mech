// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa4e0: [u32; 0];
    static mut DAT_100aa4e4: [u32; 0];
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002634e(param_1: i32) -> i32;
    fn FUN_100266c9(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_10014d06(param_1: i32, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014b90(param_1: i32, param_2: *mut i32, param_3: i32) -> u32 {
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // a game object, param_2 at six ints, and the tables hold the game's
    // function pointers indexed as the original does.
    unsafe {
        let iv = FUN_1002634e(param_2 as i32);
        if iv < 1 {
            return 1;
        }
        let idx = ((param_1.wrapping_add(0x24) as usize as *const i32).read_unaligned() as u32)
            .wrapping_mul(3) as usize;
        let e4 = (&raw mut DAT_100aa4e4)
            .cast::<u32>()
            .add(idx)
            .read_unaligned();
        if e4 == 0 {
            let e0 = (&raw mut DAT_100aa4e0)
                .cast::<u32>()
                .add(idx)
                .read_unaligned();
            if e0 == 0 {
                FUN_10014d06(param_1, param_2, param_3);
                1
            } else {
                let pc: unsafe extern "C" fn(i32, i32, i32, i32) -> i32 = core::mem::transmute(e0);
                let mut local_2c = param_2.read_unaligned();
                let mut local_14 = param_2.add(1).read_unaligned();
                let mut local_18 = param_2.add(2).read_unaligned();
                let i2 = param_2.add(3).read_unaligned().wrapping_sub(local_2c);
                let i4 = param_2.add(4).read_unaligned().wrapping_sub(local_14);
                let i5 = param_2.add(5).read_unaligned().wrapping_sub(local_18);
                let mut n = 0;
                while n < 4 {
                    local_2c = local_2c.wrapping_add(i2 >> 2);
                    local_14 = local_14.wrapping_add(i4 >> 2);
                    local_18 = local_18.wrapping_add(i5 >> 2);
                    let r = pc(param_1, local_2c, local_14, local_18);
                    if r != 0 {
                        FUN_100266c9(param_2, local_2c, local_14, local_18);
                        DAT_100aa540 = 0;
                        DAT_100aa544 = 0;
                        DAT_100aa548 = 0;
                        return 1;
                    }
                    n += 1;
                }
                0
            }
        } else {
            let f: unsafe extern "C" fn(i32, *mut i32) -> u32 = core::mem::transmute(e4);
            f(param_1, param_2)
        }
    }
}
