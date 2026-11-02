// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067af1(param_1: *mut i32) -> i32 {
    // SAFETY: param_1 points at the caller's int array; the global is the game's own.
    unsafe {
        let p1 = param_1.add(1).read_unaligned();
        let p2 = param_1.add(2).read_unaligned();
        let i_var1 = (p1 as u32).wrapping_sub(p2 as u32) as i32;
        if i_var1 != 0 {
            let p0 = param_1.read_unaligned();
            let num =
                ((DAT_100b38fc as u32).wrapping_sub(p0 as u32)).wrapping_mul(i_var1 as u32) as i32;
            let divisor = param_1.add(3).read_unaligned();
            let i_var2: i32;
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) divisor,
                inout("eax") num => i_var2,
                out("edx") _,
            );
            if (i_var1 < 0 && i_var2 <= i_var1) || (0 < i_var1 && i_var1 <= i_var2) {
                let v = param_1.add(1).read_unaligned();
                param_1.add(2).write_unaligned(v);
            } else {
                let v = (param_1.add(2).read_unaligned() as u32).wrapping_add(i_var2 as u32) as i32;
                param_1.add(2).write_unaligned(v);
            }
        }
        param_1.write_unaligned(DAT_100b38fc);
        param_1.add(2).read_unaligned()
    }
}
