// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008379c: u8;
    static mut DAT_10083800: u8;
    static mut DAT_10083870: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10004420(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own tables; the pointers walk them
    // exactly as the original does, with reads and writes in the same order.
    unsafe {
        let mut p5 = (&raw mut DAT_10083800) as *mut i32;
        let mut p6 = (&raw mut DAT_10083800) as *mut i32;
        let end1 = (&raw mut DAT_10083870) as *mut i32;
        loop {
            if p6.read_unaligned() == param_1 {
                p6.write_unaligned(-1);
            }
            if p6.read_unaligned() != -1 {
                if p5.read_unaligned() == -1 {
                    let i1 = p6.add(1).read_unaligned();
                    p5.write_unaligned(p6.read_unaligned());
                    p5.add(1).write_unaligned(i1);
                    p6.write_unaligned(-1);
                }
                let i1 = p5.read_unaligned();
                if (i1 / -100 == param_1 / -100) && (param_1 % 100 < i1 % 100) {
                    p5.write_unaligned(i1.wrapping_sub(1));
                }
                p5 = p5.add(2);
            }
            p6 = p6.add(2);
            if !(p6 < end1) {
                break;
            }
        }

        let mut q5 = (&raw mut DAT_1008379c) as *mut i32;
        let mut q6 = (&raw mut DAT_1008379c) as *mut i32;
        let end2 = (&raw mut DAT_10083800) as *mut i32;
        let i1 = param_1.wrapping_mul(100).wrapping_add(0x2711);
        let i3 = i1 / 100;
        let mut i4: i32;
        loop {
            if q5.read_unaligned() / 100 == i3 {
                q5.write_unaligned(-1);
            }
            i4 = q5.read_unaligned();
            if i4 != -1 {
                if q6.read_unaligned() == -1 {
                    q6.write_unaligned(i4);
                    q5.write_unaligned(-1);
                }
                let i2 = q6.read_unaligned();
                i4 = i1 / -10000;
                if i2 / -10000 == i4 {
                    i4 = i3 / 100;
                    if i3 % 100 < (i2 / 100) % 100 {
                        q6.write_unaligned(i2.wrapping_sub(100));
                    }
                }
                q6 = q6.add(1);
            }
            q5 = q5.add(1);
            if !(q5 < end2) {
                break;
            }
        }
        i4
    }
}
