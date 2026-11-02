// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c558: i32;
    static mut _DAT_1009c68c: i32;
    static mut DAT_1009c690: *mut u8;
    static mut DAT_1009fea0: [i8; 16];
    static mut DAT_1009feb0: [i8; 4];
    static mut DAT_100bd950: [i8; 0x40];
    static mut DAT_100bfed8: [i8; 0x100];
    static mut DAT_100beeb8: [i8; 0xde0];
    static mut DAT_100bdf90: [u32; 0x100];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100576a0(param_1: i32) -> i32;
    fn FUN_10053df0(param_1: i32);
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FID_conflict___wfopen(param_1: *const i8, param_2: *const i8) -> *mut u8;
    fn _fread(param_1: *mut u8, param_2: u32, param_3: u32, param_4: *mut u8) -> u32;
    fn _fclose(param_1: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100540d0() {
    let mut local_20: [i32; 8] = [0; 8];
    // SAFETY: all globals and callees are the game's own; the pointers
    // computed stay within the game's tables exactly as the original does.
    unsafe {
        let i_var2 = FUN_100576a0(DAT_1009c558);
        _sprintf(
            local_20.as_mut_ptr().cast::<i8>(),
            (&raw const DAT_1009fea0).cast::<i8>(),
            (i_var2 as u32).wrapping_add(0x50),
        );
        let file = FID_conflict___wfopen(
            local_20.as_ptr().cast::<i8>(),
            (&raw const DAT_1009feb0).cast::<i8>(),
        );
        if file.is_null() {
            return;
        }
        _fread((&raw mut DAT_100bd950).cast::<u8>(), 0x40, 1, file);
        _fread((&raw mut DAT_100bfed8).cast::<u8>(), 0x10, 0x10, file);
        _fread((&raw mut DAT_100beeb8).cast::<u8>(), 0xde0, 1, file);
        _fclose(file);
        let base = (&raw mut DAT_100beeb8).cast::<u8>();
        let tbl = (&raw mut DAT_100bdf90).cast::<u8>();
        FUN_10053df0(base as i32);
        local_20[0] = 0;
        loop {
            let mut i_var4: i32 = 0;
            let mut i_var2: i32 = 0;
            loop {
                let src = base
                    .byte_add(local_20[0].wrapping_add(i_var4).wrapping_mul(0x18) as usize)
                    .cast::<i32>();
                if DAT_1009c558 == src.add(1).read_unaligned() {
                    let mut i_var6: i32 = 0;
                    let mut i_var3: i32 = i_var2;
                    loop {
                        let dst = tbl.byte_add(i_var3 as usize).cast::<i32>();
                        let differs = dst.add(1).read_unaligned() != src.add(1).read_unaligned()
                            || src.read_unaligned() != dst.read_unaligned()
                            || src.add(2).read_unaligned() != dst.add(2).read_unaligned()
                            || src.add(4).read_unaligned() != dst.add(4).read_unaligned()
                            || src.add(5).read_unaligned() != dst.add(5).read_unaligned()
                            || src.add(3).read_unaligned() != dst.add(3).read_unaligned();
                        if differs {
                            if -1 < dst.add(1).read_unaligned() {
                                i_var3 = i_var3.wrapping_add(0x378);
                                i_var6 = i_var6.wrapping_add(1);
                                if 3 < i_var6 {
                                    break;
                                }
                                continue;
                            }
                            let off = i_var6
                                .wrapping_mul(0x25)
                                .wrapping_add(i_var4)
                                .wrapping_mul(6);
                            let mut p5 = src.cast::<u32>();
                            let mut p7 = (&raw mut DAT_100bdf90).cast::<u32>().offset(off as isize);
                            let mut n = 6;
                            while n != 0 {
                                p7.write_unaligned(p5.read_unaligned());
                                p5 = p5.add(1);
                                p7 = p7.add(1);
                                n -= 1;
                            }
                        }
                        break;
                    }
                }
                i_var2 = i_var2.wrapping_add(0x18);
                i_var4 = i_var4.wrapping_add(1);
                if i_var2 >= 0x378 {
                    break;
                }
            }
            local_20[0] = local_20[0].wrapping_add(0x25);
            if local_20[0] >= 0x94 {
                break;
            }
        }
        _DAT_1009c68c = 0;
        DAT_1009c690 = (&raw mut DAT_100bdf90).cast::<u8>();
    }
}
