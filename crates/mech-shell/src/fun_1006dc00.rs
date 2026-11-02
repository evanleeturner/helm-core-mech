// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut u8;
    static mut DAT_100aa6b8: u32;
    static mut DAT_100aa6bc: u32;
    static mut DAT_100aa6c8: *mut c_void;
    static mut DAT_100aaa98: [u8; 0];
    static mut DAT_100aad70: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006b9c0(param_1: i32, param_2: i8, param_3: *mut u32) -> u32;
    fn FUN_1006b450(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
    fn FUN_10063de0(param_1: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10010d80(param_1: *mut c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006dc00(param_1: i8) -> i32 {
    // SAFETY: all globals and pointers are the game's own memory; reads,
    // writes and calls occur in the original's order.
    unsafe {
        let mut puVar4: *mut u32 = &raw mut DAT_100aad70;
        let mut i_var1: i32 = DAT_100aad70 as i32;
        loop {
            if i_var1 == -1 {
                return 0;
            }
            if puVar4.add(0xb).read_unaligned() == 1 {
                break;
            }
            i_var1 = puVar4.add(0xe).read_unaligned() as i32;
            puVar4 = puVar4.add(0xe);
        }
        let base_a = DAT_100aa6b8 as usize;
        let base_b = DAT_100aa6bc as usize;
        let idx = |p: *mut u32| -> i32 {
            let a = (DAT_100aa6b8 as usize).wrapping_add(0x364) as *const i32;
            (p.add(9).read_unaligned() as i32)
                .wrapping_add(a.read_unaligned().wrapping_mul(0xc))
                .wrapping_mul(4)
        };
        i_var1 = idx(puVar4);
        let i_var2 = ((base_b.wrapping_add(0x368).wrapping_add(i_var1 as usize)) as *const i32)
            .read_unaligned();
        if i_var2 == -1 {
            return -3;
        }
        if ((base_a.wrapping_add(0x368).wrapping_add(i_var1 as usize)) as *const i32)
            .read_unaligned()
            == i_var2
        {
            return -2;
        }
        let mut i_stack_4: i32 = 0;
        let u_var3 = FUN_1006b9c0(i_var2, 0, (&raw mut i_stack_4).cast::<u32>());
        if (u_var3 as u8) == 1 {
            if 0 < i_stack_4 {
                *((DAT_100aa6b8 as usize).wrapping_add(0x84c) as *mut u8) = 1;
                *DAT_10083508 = 1;
            }
            if param_1 == 0 {
                i_var1 = idx(puVar4);
                let src = ((DAT_100aa6bc as usize)
                    .wrapping_add(0x368)
                    .wrapping_add(i_var1 as usize)) as *const u32;
                let v = src.read_unaligned();
                let dst = ((DAT_100aa6b8 as usize)
                    .wrapping_add(0x368)
                    .wrapping_add(i_var1 as usize)) as *mut u32;
                dst.write_unaligned(v);
                let cnt =
                    ((DAT_100aa6b8 as usize).wrapping_add(0x364) as *const i32).read_unaligned();
                FUN_1006b450(
                    DAT_100aa6b8 as i32,
                    DAT_100aa6bc as i32,
                    cnt,
                    puVar4.add(9).read_unaligned() as i32,
                    i_var2,
                );
                FUN_10010d80(DAT_100aa6c8);
                FUN_1006b9c0(i_var2, 1, (&raw mut i_stack_4).cast::<u32>());
                if 0 < i_stack_4 {
                    *((DAT_100aa6b8 as usize).wrapping_add(0x84c) as *mut u8) = 1;
                    *DAT_10083508 = 1;
                }
                FUN_10063de0((&raw mut DAT_100aaa98).cast::<i32>());
            }
            return i_stack_4;
        }
        -1
    }
}
