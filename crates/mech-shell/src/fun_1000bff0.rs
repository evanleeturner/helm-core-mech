// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a790(param_1: i32) -> u8;
    fn FUN_1000a800(param_1: i32) -> u8;
    fn FUN_1000a8c0(param_1: i32) -> u8;
    fn FUN_1000aa80(param_1: i32) -> u8;
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000bff0(param_1: i32, param_2: i32) {
    if param_2 == 0 {
        return;
    }
    let d = param_1 as usize as *mut u8;
    let s = param_2 as usize as *const u8;
    // SAFETY: both params point at the caller's game structures, large enough
    // for every offset read and written, as in the original; callees are the
    // game's own.
    unsafe {
        let mut i: usize = 0;
        while i < 0x19 {
            let v = s.byte_add(0x28c + i * 4).cast::<u32>().read_unaligned();
            d.byte_add(0x28c + i * 4).cast::<u32>().write_unaligned(v);
            i += 1;
        }
        i = 0;
        while i < 0x1c {
            let v = s.byte_add(0x2f0 + i * 4).cast::<u32>().read_unaligned();
            d.byte_add(0x2f0 + i * 4).cast::<u32>().write_unaligned(v);
            i += 1;
        }
        i = 0;
        while i < 96 {
            let v = s.byte_add(0x368 + i * 4).cast::<u32>().read_unaligned();
            d.byte_add(0x368 + i * 4).cast::<u32>().write_unaligned(v);
            i += 1;
        }
        i = 0;
        while i < 8 {
            let so = 0x760 + i * 0x12;
            let v0 = s.byte_add(so).cast::<u32>().read_unaligned();
            d.byte_add(so).cast::<u32>().write_unaligned(v0);
            let v1 = s.byte_add(so + 4).cast::<u32>().read_unaligned();
            d.byte_add(so + 4).cast::<u32>().write_unaligned(v1);
            let v2 = s.byte_add(so + 8).cast::<u32>().read_unaligned();
            d.byte_add(so + 8).cast::<u32>().write_unaligned(v2);
            let v3 = s.byte_add(so + 12).cast::<u32>().read_unaligned();
            d.byte_add(so + 12).cast::<u32>().write_unaligned(v3);
            let v4 = s.byte_add(so + 16).cast::<u16>().read_unaligned();
            d.byte_add(so + 16).cast::<u16>().write_unaligned(v4);
            i += 1;
        }
        let offs: [usize; 29] = [
            0x200, 0x204, 0x208, 0x220, 0x224, 0x228, 0x22c, 0x230, 0x234, 0x238, 0x23c, 0x240,
            0x244, 0x248, 0x24c, 0x250, 0x254, 0x258, 0x25c, 0x260, 0x264, 0x268, 0x26c, 0x270,
            0x274, 0x278, 0x27c, 0, 0,
        ];
        i = 0;
        while i < 27 {
            let o = offs[i];
            // The original copies +0x24c from +0x240.
            let so = if o == 0x24c { 0x240 } else { o };
            let v = s.byte_add(so).cast::<u32>().read_unaligned();
            d.byte_add(o).cast::<u32>().write_unaligned(v);
            i += 1;
        }
        FUN_1000a790(param_1);
        FUN_1000a800(param_1);
        FUN_1000a8c0(param_1);
        FUN_1000aa80(param_1);
        FUN_1000a890(param_1);
    }
}
