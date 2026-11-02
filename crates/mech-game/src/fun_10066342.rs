// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52f4: i32;
    static mut DAT_101f52d4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066342(param_1: i32, param_2: *const i32, param_3: i32) -> i32 {
    let _ = param_1;
    let mut local_18: [i32; 5] = [0; 5];
    // SAFETY: param_2 points at a caller-provided 5-int record; param_3 is the
    // address of a caller-provided record with int fields at +4..+0x10; the
    // globals are the game's own.
    unsafe {
        let mut src = param_2;
        for i in 0..5 {
            local_18[i] = src.read_unaligned();
            src = src.add(1);
        }

        let u_w = (local_18[3] as u32).wrapping_sub(local_18[1] as u32);
        let i_var2 = ((DAT_101f52f4 as u32)
            .wrapping_sub(u_w.wrapping_add(1))
            .wrapping_sub(1) as i32)
            / 2;

        let u_h = (local_18[4] as u32).wrapping_sub(local_18[2] as u32);
        let i_var1 = ((DAT_101f52d4 as u32)
            .wrapping_sub(u_h.wrapping_add(1))
            .wrapping_sub(1) as i32)
            / 2;

        let base = param_3 as *mut u8;
        base.byte_add(4).cast::<i32>().write_unaligned(i_var2);
        base.byte_add(0xc)
            .cast::<i32>()
            .write_unaligned(u_w.wrapping_add(i_var2 as u32) as i32);
        base.byte_add(8).cast::<i32>().write_unaligned(i_var1);
        base.byte_add(0x10)
            .cast::<i32>()
            .write_unaligned((i_var1 as u32).wrapping_add(u_h) as i32);
    }
    param_3
}
