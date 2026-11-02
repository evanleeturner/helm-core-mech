// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160eb4: u8;
    static mut DAT_10160eb8: u8;
    static mut DAT_10160ec4: i32;
    static mut DAT_10160ec0: i32;
    static mut DAT_10160edc: i32;
    static mut DAT_10160ee8: i32;
    static mut DAT_10160ed0: i32;
    static mut DAT_10160ee0: i32;
    static mut DAT_101150f8: u8;
    static mut DAT_10115168: u8;
    static mut DAT_10115160: i32;
    static mut DAT_1011515c: i32;
    static mut DAT_10115100: [*mut u32; 20];
}

#[inline(always)]
unsafe fn idiv64(num: i64, div: i32) -> i32 {
    let lo = num as u32;
    let hi = (num >> 32) as u32;
    let q: u32;
    // SAFETY: the machine's idiv faults on a zero divisor or an overflowing
    // quotient, exactly as the original does.
    unsafe {
        core::arch::asm!(
            "idiv {d:e}",
            d = in(reg) div,
            inout("eax") lo => q,
            inout("edx") hi => _,
            options(nomem, nostack)
        );
    }
    q as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10080639(param_1: *mut u32) -> *mut u32 {
    // SAFETY: param_1 points at the caller's point record; the globals and
    // the table are the game's own memory.
    unsafe {
        let p8 = param_1.cast::<u8>();
        if (p8.add(0x1d).read() & 1) == 0 {
            let sh = (DAT_10160eb4 & 0x1f) as u32;
            let num = ((param_1.read_unaligned() as i32 as i64) as u64).wrapping_shl(sh) as i64;
            let div = param_1.add(2).read_unaligned() as i32;
            let q = idiv64(num, div);
            let x = (q.wrapping_add(2) >> 2).wrapping_add(DAT_10160ec4);
            param_1.add(3).write_unaligned(x as u32);

            let mut b: u8 = 0;
            if DAT_10160edc < x {
                b = 2;
            }
            if x < DAT_10160ec0 {
                b |= 1;
            }

            let sh = (DAT_10160eb8 & 0x1f) as u32;
            let num =
                ((param_1.add(1).read_unaligned() as i32 as i64) as u64).wrapping_shl(sh) as i64;
            let div = param_1.add(2).read_unaligned() as i32;
            let q = idiv64(num, div);
            let y = DAT_10160ee8.wrapping_sub(q.wrapping_add(2) >> 2);
            param_1.add(4).write_unaligned(y as u32);

            if DAT_10160ed0 < y {
                b |= 8;
            }
            if y < DAT_10160ee0 {
                b |= 4;
            }
            p8.add(0x1c).write(b);
            let f = p8.add(0x1d).read();
            p8.add(0x1d).write(f | 1);
        }

        let oc = p8.add(0x1c).read();
        DAT_101150f8 |= oc;
        DAT_10115168 &= p8.add(0x1c).read();

        if DAT_10115160 < 0x14 {
            (&raw mut DAT_10115100)
                .cast::<*mut u32>()
                .offset(DAT_10115160 as isize)
                .write(param_1);
            DAT_10115160 = DAT_10115160.wrapping_add(1);
        } else {
            DAT_1011515c = 0;
        }
        param_1
    }
}
