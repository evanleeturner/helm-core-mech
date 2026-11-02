// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100b3d64: u32;
    static mut DAT_100b3d68: u32;
    static mut DAT_100b3d6c: u8;
    static mut DAT_100b3d70: [i8; 1];
    static mut DAT_100b3d74: [i8; 1];
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_100af164: *const i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strchr(s: *mut i8, c: i32) -> *mut i8;
    fn fopen(name: *const i8, mode: *const i8) -> *mut u8;
    fn fclose(f: *mut u8) -> i32;
    fn FUN_1006b810(param_1: *mut u8, param_2: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006c015() -> i32 {
    let mut buf = [0u8; 512];
    let mut local_40: i32 = 0;
    // SAFETY: the globals, callees and game structures are the game's own
    // memory; pointers are read and written where the original does.
    unsafe {
        if DAT_100b3d64 == 0 {
            return 0;
        }
        let mut idx: i32 = 0;
        while idx < DAT_100b38d8 {
            let entry = (&raw mut DAT_100eef40)
                .cast::<u32>()
                .add(idx as usize)
                .read();
            let piVar2 = ((entry.wrapping_add(0x20)) as *const u32).read_unaligned() as *mut i32;
            let flags = piVar2.byte_add(0x442).cast::<u16>().read_unaligned();
            if (flags & 2) != 0 {
                let b = buf.as_mut_ptr();
                b.cast::<u32>().write_unaligned(DAT_100b3d68);
                b.add(4).write(DAT_100b3d6c);

                // append the string at piVar2 + 0x11c ints
                let src = piVar2.add(0x11c).cast::<u8>();
                let mut n = 0usize;
                while src.add(n).read() != 0 {
                    n += 1;
                }
                n += 1;
                let mut d = 0usize;
                while b.add(d).read() != 0 {
                    d += 1;
                }
                let mut k = 0usize;
                while k < n {
                    b.add(d + k).write(src.add(k).read());
                    k += 1;
                }

                let dot = strchr(b.cast::<i8>(), 0x2e);
                if dot.is_null() {
                    let s = DAT_100af164.cast::<u8>();
                    let mut n = 0usize;
                    while s.add(n).read() != 0 {
                        n += 1;
                    }
                    n += 1;
                    let mut d = 0usize;
                    while b.add(d).read() != 0 {
                        d += 1;
                    }
                    let mut k = 0usize;
                    while k < n {
                        b.add(d + k).write(s.add(k).read());
                        k += 1;
                    }
                }

                let f = fopen(b.cast::<i8>(), (&raw mut DAT_100b3d70).cast::<i8>());
                if !f.is_null() {
                    fclose(f);
                    let f2 = fopen(b.cast::<i8>(), (&raw mut DAT_100b3d74).cast::<i8>());
                    if !f2.is_null() && !piVar2.is_null() {
                        FUN_1006b810(f2, piVar2);
                        fclose(f2);
                        local_40 = local_40.wrapping_add(1);
                    }
                }
            }
            idx = idx.wrapping_add(1);
        }
    }
    local_40
}
