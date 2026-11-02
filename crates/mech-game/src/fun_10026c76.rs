// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad1bc: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006312b(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10026c76(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's object; the global table pointer
    // and callee are the game's own, accessed where the original does.
    unsafe {
        let c_var1 = param_1.add(0x62).read_unaligned() as i8;
        if c_var1 == 0 {
            param_1.add(0x6a).write_unaligned(1);
            param_1.add(0x6e).write_unaligned(0);
            param_1.add(0x6b).write_unaligned(0x21);
        } else if c_var1 == 1 {
            param_1.add(0x6a).write_unaligned(1);
            param_1.add(0x6e).write_unaligned(2);
            param_1.add(0x6b).write_unaligned(0x21);
        } else if c_var1 == 2 {
            param_1.add(0x6a).write_unaligned(0);
            param_1.add(0x6e).write_unaligned(2);
            param_1.add(0x6b).write_unaligned(100);
        }

        let b_idx = param_1.byte_add(0x189).cast::<i8>().read();
        let pu_var2 =
            DAT_100ad1bc.byte_add((b_idx as i32).wrapping_mul(0x1c) as u32 as usize) as *mut u32;

        if param_1.read_unaligned() == 1 {
            let u_var4 = pu_var2.read_unaligned();
            param_1
                .byte_add(0x202)
                .cast::<u32>()
                .write_unaligned(u_var4);
        } else {
            param_1.byte_add(0x202).cast::<u32>().write_unaligned(0);
        }

        let u_var4 = pu_var2.add(1).read_unaligned();
        param_1
            .byte_add(0x1fe)
            .cast::<u32>()
            .write_unaligned(u_var4);

        let u_var4 = pu_var2.add(2).read_unaligned();
        param_1
            .byte_add(0x192)
            .cast::<u32>()
            .write_unaligned(u_var4);

        let f = f32::from_bits(pu_var2.add(3).read_unaligned());
        param_1.add(0x6c).write_unaligned(f as i32);

        param_1
            .add(0x6d)
            .write_unaligned(pu_var2.add(4).read_unaligned() as i32);
        param_1
            .add(0x6f)
            .write_unaligned(pu_var2.add(5).read_unaligned() as i32);
        param_1
            .add(0x70)
            .write_unaligned(pu_var2.add(6).read_unaligned() as i32);

        let i_var3 = param_1.byte_add(0x192).cast::<i32>().read_unaligned();
        if i_var3 != 0 {
            FUN_1006312b(param_1.add(8).read_unaligned(), 0);
        }
    }
}
