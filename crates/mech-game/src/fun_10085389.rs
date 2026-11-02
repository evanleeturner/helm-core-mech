// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f90: i32;
    static mut DAT_100b9f8c: i32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f88: *mut i32;
    static mut DAT_100b9a2c: *mut i32;
    static mut DAT_100c9070: i32;
    static mut DAT_100c91f4: i32;
    static mut DAT_100b9a30: [u8; 0];
    static mut DAT_100b9a32: [u8; 0];
    static mut DAT_100ba59c: [i8; 0];
    static mut DAT_100ba5c4: [i8; 0];
    static mut DAT_100ba5f0: [i8; 0];
    static mut DAT_100ba61c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, ...);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10085dfd() -> i32;
}

unsafe fn slot(obj: *mut i32, off: usize) -> usize {
    // SAFETY: obj is a COM-style object whose first dword is a vtable pointer.
    unsafe {
        let vt = obj.read_unaligned() as u32 as usize as *const u8;
        vt.add(off).cast::<u32>().read_unaligned() as usize
    }
}

unsafe fn call0(obj: *mut i32, off: usize) -> i32 {
    // SAFETY: the slot holds a stdcall method taking only `this`.
    unsafe {
        let f = core::mem::transmute::<usize, unsafe extern "stdcall" fn(*mut i32) -> i32>(slot(
            obj, off,
        ));
        f(obj)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10085389(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let mut local_40c = MaybeUninit::<[u8; 1024]>::uninit();
    let buf = local_40c.as_mut_ptr().cast::<u8>();
    let mut local_c: i32 = 0;
    // SAFETY: globals and callees are the game's own; param_3 points at the
    // caller's RGB triples; buf has room for 256 four-byte entries.
    unsafe {
        if DAT_100b9f90 == 0 {
            return -1;
        }
        if param_3 == 0 || param_1 < 0 || 0xff < param_1 || param_2 < 1 || 0x100 - param_1 < param_2
        {
            return -1;
        }
        let mut local_8: i32 = 0;
        while local_8 < param_2 {
            let p2 = param_3.wrapping_add(local_8.wrapping_mul(3)) as u32 as usize as *const u8;
            let i_var3 = local_8.wrapping_add(param_1).wrapping_mul(3);
            let s_val = p2.cast::<u16>().read_unaligned();
            (&raw mut DAT_100b9a30)
                .cast::<u8>()
                .offset(i_var3 as isize)
                .cast::<u16>()
                .write_unaligned(s_val);
            (&raw mut DAT_100b9a32)
                .cast::<u8>()
                .offset(i_var3 as isize)
                .write(p2.add(2).read());
            let o = (local_8 as isize) * 4;
            buf.offset(o).write(p2.read() << 2);
            buf.offset(o + 1).write(p2.add(1).read() << 2);
            buf.offset(o + 2).write(p2.add(2).read() << 2);
            local_8 += 1;
        }
        if DAT_100b9f8c != 0 {
            local_c = 1;
            let obj = DAT_100b9f7c;
            let f = core::mem::transmute::<usize, unsafe extern "stdcall" fn(*mut i32, i32) -> i32>(
                slot(obj, 0x80),
            );
            DAT_100c9070 = f(obj, DAT_100c91f4);
            DAT_100b9a2c.write(0);
            DAT_100b9f8c = 0;
            if DAT_100c9070 != 0 {
                FUN_10002168((&raw mut DAT_100ba59c).cast::<i8>(), DAT_100c9070);
            }
        }
        let i_var3 = call0(DAT_100b9f74, 0x60);
        if i_var3 == -0x7789fe3e {
            DAT_100c9070 = call0(DAT_100b9f74, 0x6c);
            let i2 = call0(DAT_100b9f7c, 0x60);
            if i2 == -0x7789fe3e {
                DAT_100c9070 = call0(DAT_100b9f7c, 0x6c);
            }
        } else {
            DAT_100c9070 = 0;
        }
        if DAT_100c9070 != 0 {
            FUN_10002168((&raw mut DAT_100ba5c4).cast::<i8>(), DAT_100c9070);
        }
        let pal = DAT_100b9f88;
        let f = core::mem::transmute::<
            usize,
            unsafe extern "stdcall" fn(*mut i32, i32, i32, i32, *mut u8) -> i32,
        >(slot(pal, 0x18));
        DAT_100c9070 = f(pal, 0, param_1, param_2, buf);
        if DAT_100c9070 == 0 {
            if local_c == 0 {
                return 0;
            }
            DAT_100c9070 = FUN_10085dfd();
            if DAT_100c9070 == 0 {
                0
            } else {
                FUN_10002168((&raw mut DAT_100ba61c).cast::<i8>(), DAT_100c9070);
                -1
            }
        } else {
            FUN_10002168((&raw mut DAT_100ba5f0).cast::<i8>(), DAT_100c9070);
            -1
        }
    }
}
