// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: i32;
    static mut DAT_100b04b0: u32;
    static mut DAT_100adf74: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004652f();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(a: u32, b: u32, c: u32, d: u32) -> u32;
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046dbc(param_1: *mut i32) -> u32 {
    let mut local_14: [i32; 4] = [0; 4];
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's 3-int structure, and the heap block is sized by the original.
    unsafe {
        if DAT_100b04c4 == 0 {
            return 1;
        }
        local_14[2] = 3;
        let m = mciSendCommandA(DAT_100b04b0, 0x814, 0x100, local_14.as_mut_ptr() as u32);
        if m != 0 {
            FUN_1004652f();
            return 0;
        }
        param_1.write_unaligned(1);
        param_1.add(1).write_unaligned(local_14[1]);
        let pv = HeapAlloc(
            DAT_100adf74,
            1,
            (local_14[1] as u32).wrapping_mul(4).wrapping_add(4),
        );
        param_1.add(2).write_unaligned(pv as usize as i32);
        if param_1.add(2).read_unaligned() == 0 {
            DAT_100b04c4 = 0;
            return 1;
        }
        let mut local_18: i32 = param_1.read_unaligned();
        while local_18 <= param_1.add(1).read_unaligned() {
            local_14[2] = 2;
            local_14[3] = local_18;
            let m = mciSendCommandA(DAT_100b04b0, 0x814, 0x110, local_14.as_mut_ptr() as u32);
            if m != 0 {
                FUN_1004652f();
                return 0;
            }
            let off = (local_18 as u32)
                .wrapping_sub(param_1.read_unaligned() as u32)
                .wrapping_mul(4) as i32;
            let addr = param_1.add(2).read_unaligned().wrapping_add(off);
            (addr as usize as *mut i32).write_unaligned(local_14[1]);
            local_18 = local_18.wrapping_add(1);
        }
        let off = (local_18 as u32)
            .wrapping_sub(param_1.read_unaligned() as u32)
            .wrapping_mul(4) as i32;
        let addr = param_1.add(2).read_unaligned().wrapping_add(off);
        let v = ((param_1.add(1).read_unaligned() as i8 as i32).wrapping_add(1)) as u8 as u32;
        (addr as usize as *mut u32).write_unaligned(v);
        0
    }
}
