// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f462(param_1: i32, param_2: *mut u32) {
    // SAFETY: param_1 is an address of the game's object and param_2 points at
    // nine u32 values; reads and writes happen in the original's order.
    unsafe {
        let base = param_1 as usize as *mut u8;
        base.byte_add(0xc)
            .cast::<u32>()
            .write_unaligned(param_2.read_unaligned());
        base.byte_add(0x10)
            .cast::<u32>()
            .write_unaligned(param_2.add(1).read_unaligned());
        base.byte_add(0x14)
            .cast::<u32>()
            .write_unaligned(param_2.add(2).read_unaligned());
        base.byte_add(0x18)
            .cast::<u32>()
            .write_unaligned(param_2.add(3).read_unaligned());
        base.byte_add(0x1c)
            .cast::<u32>()
            .write_unaligned(param_2.add(4).read_unaligned());
        base.byte_add(0x20)
            .cast::<u32>()
            .write_unaligned(param_2.add(5).read_unaligned());
        base.byte_add(0x24)
            .cast::<u32>()
            .write_unaligned(param_2.add(6).read_unaligned());
        base.byte_add(0x28)
            .cast::<u32>()
            .write_unaligned(param_2.add(7).read_unaligned());
        base.byte_add(0x2c)
            .cast::<u32>()
            .write_unaligned(param_2.add(8).read_unaligned());
        let flags = base.byte_add(0x74).cast::<u32>();
        flags.write_unaligned(flags.read_unaligned() | 1);
    }
}
