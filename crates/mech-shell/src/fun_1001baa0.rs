// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6738: *mut c_void;
    static mut DAT_100b12a8: [u32; 0];
    static mut DAT_100b12c8: [u8; 0];
    static mut DAT_100b13e8: [u8; 0];
    static mut DAT_100b13f0: [u8; 0];
    static mut DAT_100b50f0: u32;
    static mut s__s_02dstd_1008fd14: [i8; 0];
    static mut s_mek__s__usr_mek_1008fd20: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *mut i8, ...) -> i32;
    fn __findfirst(param_1: *const c_void, param_2: *mut u32) -> *mut c_void;
    fn _strncpy(dest: *mut i8, source: *mut i8, count: u32) -> *mut i8;
    fn __findnext(param_1: *mut c_void, param_2: *mut u32) -> u32;
    fn FUN_1006f590(param_1: *mut c_void) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10036390(this: *mut c_void, param_1: *mut i8, param_2: u32) -> i32;
}

#[repr(C)]
struct FindData {
    head: [u32; 5],
    name: [i8; 260],
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001baa0(param_1: u32, param_2: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets and copies mirror the original's addressing.
    unsafe {
        let mut local: FindData = FindData {
            head: [0; 5],
            name: [0; 260],
        };
        let arr = (&raw mut DAT_100b12a8).cast::<u32>();
        let buf = (&raw mut DAT_100b12c8).cast::<u8>();
        let base = (&raw mut DAT_100b13e8).cast::<u8>();

        arr.offset(param_2 as isize).write_unaligned(0);
        let mut i_var6: i32 = 1;
        let mut p = base.offset((DAT_100b50f0 as i32).wrapping_mul(0xa28) as isize);
        for _ in 0..0x28au32 {
            p.cast::<u32>().write_unaligned(0);
            p = p.add(4);
        }
        loop {
            _sprintf(
                buf.cast::<i8>(),
                (&raw mut s__s_02dstd_1008fd14).cast::<i8>(),
                param_1,
                i_var6,
            );
            let i_var3 = FUN_10036390(DAT_100a6738, buf.cast::<i8>(), 8);
            if -1 < i_var3 {
                let mut len: usize = 0;
                while *buf.add(len) != 0 {
                    len += 1;
                }
                len += 1;
                let off = i_var6
                    .wrapping_add((DAT_100b50f0 as i32).wrapping_mul(200))
                    .wrapping_mul(0xd);
                let dst = base.offset(off as isize);
                for k in 0..len {
                    *dst.add(k) = *buf.add(k);
                }
            }
            i_var6 = i_var6.wrapping_add(1);
            if 99 < i_var6 {
                _sprintf(
                    buf.cast::<i8>(),
                    (&raw mut s_mek__s__usr_mek_1008fd20).cast::<i8>(),
                    param_1,
                );
                let h = __findfirst(buf.cast::<c_void>(), local.head.as_mut_ptr());
                if h as usize != 0xffff_ffff {
                    let mut i_var3: i32 = 0;
                    loop {
                        let idx = DAT_100b50f0 as i32;
                        let a = arr.offset(param_2 as isize);
                        a.write_unaligned(a.read_unaligned().wrapping_add(1));
                        let off = i_var3.wrapping_add(idx.wrapping_mul(200)).wrapping_mul(0xd);
                        _strncpy(
                            base.offset(off as isize).cast::<i8>(),
                            local.name.as_mut_ptr(),
                            8,
                        );
                        let off2 = i_var3
                            .wrapping_add((DAT_100b50f0 as i32).wrapping_mul(200))
                            .wrapping_mul(0xd);
                        (&raw mut DAT_100b13f0)
                            .cast::<u8>()
                            .offset(off2 as isize)
                            .write(0);
                        let r = __findnext(h, local.head.as_mut_ptr()) as i32;
                        i_var3 = i_var3.wrapping_add(1);
                        if r != 0 {
                            break;
                        }
                    }
                    FUN_1006f590(h);
                }
                return;
            }
        }
    }
}
