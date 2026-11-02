// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_100399ec(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a783(param_1: i32) -> bool {
    let mut local_18: u32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: u32 = 0;
    // SAFETY: param_1 points at the game's object; all offsets, globals and
    // callees are the game's own memory and code, accessed as the original does.
    unsafe {
        let p = param_1 as usize as *const u8;
        let local_c = p.byte_add(0x1f6).cast::<i32>().read_unaligned() as usize as *mut i32;
        FUN_1002f322(
            p.byte_add(0x1d2).cast::<i32>().read_unaligned(),
            &raw mut local_10,
            (&raw mut local_14).cast::<u32>(),
            &raw mut local_18,
        );
        if local_14 < 1 {
            let q = param_1 as usize as *mut u8;
            let w = q.byte_add(0x10).cast::<u16>();
            w.write_unaligned(w.read_unaligned() & 0xfffe);
            FUN_1002f727(q.byte_add(0x1d2).cast::<i32>().read_unaligned());
            FUN_100399ec(
                local_c.byte_add(32).read_unaligned(),
                local_c.read_unaligned(),
            );
        } else {
            let c5 = local_c.byte_add(20).read_unaligned();
            let ea = p.byte_add(0x1ea).cast::<i32>().read_unaligned();
            let i_var2 = (c5.wrapping_add(ea) as u32).wrapping_mul(DAT_100b3904) as i32;
            let i_var1 = p.byte_add(0x1ee).cast::<i32>().read_unaligned();
            let i_var3 = FUN_10051f43(10);
            let f2 = p.byte_add(0x1f2).cast::<i32>().read_unaligned();
            let c7 = local_c.byte_add(28).read_unaligned();
            let local_8 = ((f2.wrapping_add(c7) as u32).wrapping_mul(DAT_100b3904) / 0xb5) as i32;
            let c6 = local_c.byte_add(24).read_unaligned();
            let a3 = ((i_var1.wrapping_sub(i_var3).wrapping_add(c6) as u32)
                .wrapping_mul(DAT_100b3904)
                / 0xb5) as i32;
            FUN_1002f3a7(
                p.byte_add(0x1d2).cast::<i32>().read_unaligned(),
                i_var2 / 0xb5,
                a3,
                local_8,
            );
            FUN_1002fbc0(p.byte_add(0x1d2).cast::<i32>().read_unaligned() as usize as *mut i32);
            (param_1 as usize as *mut u8)
                .byte_add(0x1e6)
                .cast::<u32>()
                .write_unaligned(0x5e);
        }
    }
    0 < local_14
}
