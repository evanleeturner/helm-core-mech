// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1d8: u32;
    static mut DAT_100aa1dc: *mut u16;
    static mut DAT_100aa2b4: u16;
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006f9a(param_1: u32, param_2: u32, param_3: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001355e(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointers read and written are the game's buffers at the original's offsets.
    unsafe {
        let p = DAT_100aa1dc;
        p.write_unaligned(DAT_100aa2b4);
        p.add(1)
            .cast::<i32>()
            .write_unaligned(param_2.wrapping_neg());
        p.add(3)
            .cast::<i32>()
            .write_unaligned(param_3.wrapping_neg());
        p.add(5)
            .cast::<i32>()
            .write_unaligned(param_4.wrapping_neg());
        let idx = DAT_100adf38 as usize;
        let entry = (&raw const DAT_100eef40)
            .cast::<u32>()
            .add(idx)
            .read_unaligned();
        let i_var1 = ((entry.wrapping_add(0x20)) as *const u32).read_unaligned();
        p.add(7)
            .cast::<u32>()
            .write_unaligned(((i_var1.wrapping_add(0x434)) as *const u32).read_unaligned());
        p.add(9)
            .cast::<u32>()
            .write_unaligned(((i_var1.wrapping_add(0x438)) as *const u32).read_unaligned());
        p.add(0xb)
            .cast::<u32>()
            .write_unaligned(((i_var1.wrapping_add(0x43c)) as *const u32).read_unaligned());
        let base = ((DAT_100aa1d8.wrapping_add(0x10)) as *const u32).read_unaligned();
        let arg =
            ((base.wrapping_add((param_1 as u32).wrapping_mul(4))) as *const u32).read_unaligned();
        FUN_10006f9a(arg, p as u32, 0x1a);
    }
}
