// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf34: *mut u8;
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100adf74: *mut u8;
    static mut DAT_10114e40: [i32; 0x22];
    static mut DAT_10114db0: [*mut i8; 0x22];
    static mut DAT_100b3220: [i32; 0];
    static mut DAT_100b2cf0: [*mut u8; 0];
    static mut DAT_100b2f98: [u8; 0];
    static mut DAT_100b3188: [i32; 0];
    static mut DAT_100ab0a8: i32;
    static mut DAT_100ab0ac: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut u8, flags: u32, size: u32) -> *mut u8;
    fn FUN_10011120();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10045d90(param_1: *mut i8);
    fn FUN_1005c34a(param_1: i32);
    fn FUN_10015e98();
    fn FUN_10067e7c();
    fn FUN_10033ae3();
    fn FUN_10008608();

    fn FUN_100682e7();
    fn FUN_100689bf();
    fn FUN_10068a69();
    fn FUN_10068bb1();
    fn FUN_1006147f();
    fn FUN_10061a73();
    fn FUN_10060c50();
    fn FUN_10061ab8();
    fn FUN_10061bc8();
    fn FUN_1002db2d();
    fn FUN_1002e029();
    fn FUN_1002e711();
    fn FUN_1002e9c4();
    fn FUN_1002ea87();
    fn FUN_1002ed27();
    fn FUN_1002e635();
}

unsafe fn obj(n: usize) -> *mut i8 {
    // SAFETY: n is within the game's 0x22-entry object table.
    unsafe { (&raw mut DAT_10114db0).cast::<*mut i8>().add(n).read() }
}

unsafe fn fnptr(o: *mut i8, off: usize) -> usize {
    // SAFETY: o is a live object whose slot at off holds a function pointer.
    unsafe { o.byte_add(off).cast::<usize>().read_unaligned() }
}

unsafe fn call_i(o: *mut i8, off: usize, a: i32) {
    // SAFETY: the slot holds a cdecl function taking (object, int).
    unsafe {
        let f: unsafe extern "C" fn(*mut i8, i32) = core::mem::transmute(fnptr(o, off));
        f(o, a);
    }
}

unsafe fn call_p(o: *mut i8, off: usize, a: *mut u8) {
    // SAFETY: the slot holds a cdecl function taking (object, pointer).
    unsafe {
        let f: unsafe extern "C" fn(*mut i8, *mut u8) = core::mem::transmute(fnptr(o, off));
        f(o, a);
    }
}

unsafe fn call_0(o: *mut i8, off: usize) {
    // SAFETY: the slot holds a cdecl function taking (object).
    unsafe {
        let f: unsafe extern "C" fn(*mut i8) = core::mem::transmute(fnptr(o, off));
        f(o);
    }
}

unsafe fn set(o: *mut i8, off: usize, v: usize) {
    // SAFETY: o is a live 0x100-byte object; off is within it.
    unsafe { o.byte_add(off).cast::<usize>().write_unaligned(v) }
}

unsafe fn set_i(o: *mut i8, off: usize, v: i32) {
    // SAFETY: o is a live 0x100-byte object; off is within it.
    unsafe { o.byte_add(off).cast::<i32>().write_unaligned(v) }
}

unsafe fn std_handlers(n: usize, kind: i32, arg: i32) {
    // SAFETY: obj(n) is a live object allocated by the loop; callbacks are the game's.
    unsafe {
        set(obj(n), 0xf4, FUN_100682e7 as usize);
        set(obj(n), 0xfc, FUN_100689bf as usize);
        set(obj(n), 0xf0, FUN_10068a69 as usize);
        set(obj(n), 0xf8, FUN_10068bb1 as usize);
        set_i(obj(n), 0xc, kind);
        call_i(obj(n), 0xd4, arg);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ca54() {
    // SAFETY: all globals, tables and callees are the game's own memory;
    // objects are allocated here with 0x100 bytes before use.
    unsafe {
        let en = (&raw mut DAT_10114e40).cast::<i32>();
        let mut i: i32 = 0;
        while i < 0x22 {
            en.add(i as usize).write(1);
            i = i.wrapping_add(1);
        }
        en.add(1).write(0);
        en.add(24).write(0);
        en.add(25).write(0);
        if DAT_100adf34.add(9).read() == 0 && DAT_100aa1ac != 0 {
            en.add(17).write(0);
        }
        let base = (&raw mut DAT_100eef40)
            .cast::<i32>()
            .add(DAT_100adf38 as usize)
            .read();
        let i_var1 = ((base as u32 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        let tbl = (&raw mut DAT_10114db0).cast::<*mut i8>();
        let mut i: i32 = 0;
        while i < 0x22 {
            let u = i as usize;
            tbl.add(u)
                .write(HeapAlloc(DAT_100adf74, 1, 0x100).cast::<i8>());
            FUN_10045d90(obj(u));
            call_i(
                obj(u),
                0xc4,
                (&raw mut DAT_100b3220).cast::<i32>().add(u).read(),
            );
            call_p(
                obj(u),
                0xd0,
                (&raw mut DAT_100b2cf0)
                    .cast::<*mut u8>()
                    .add(u.wrapping_mul(5))
                    .cast::<u8>(),
            );
            call_p(
                obj(u),
                0xd8,
                (&raw mut DAT_100b2f98).cast::<u8>().add(u.wrapping_mul(8)),
            );
            call_i(
                obj(u),
                0xdc,
                (&raw mut DAT_100b3188).cast::<i32>().add(u).read(),
            );
            call_i(obj(u), 0xd4, 0);
            if en.add(u).read() == 0 {
                call_0(obj(u), 0xec);
            } else {
                call_0(obj(u), 0xe8);
            }
            i = i.wrapping_add(1);
        }
        FUN_1005c34a(i_var1);

        std_handlers(2, 5, 1);
        std_handlers(22, 6, 9);
        std_handlers(26, 4, 10);
        std_handlers(31, 3, 0);
        std_handlers(32, 2, 0xc);
        std_handlers(33, 1, 0xd);

        set(obj(17), 0xf4, FUN_1006147f as usize);
        set(obj(17), 0xf4, FUN_100682e7 as usize);
        set(obj(17), 0xfc, FUN_10061a73 as usize);
        set(obj(18), 0xf4, FUN_10060c50 as usize);
        set(obj(17), 0xf0, FUN_10061ab8 as usize);
        set(obj(17), 0xf8, FUN_10061bc8 as usize);
        set_i(obj(17), 0xc, 7);
        call_i(obj(17), 0xd4, 8);

        set(obj(19), 0xf4, FUN_1002db2d as usize);
        set(obj(20), 0xf4, FUN_1002e029 as usize);
        set(obj(21), 0xf4, FUN_1002e711 as usize);
        set(obj(23), 0xf4, FUN_1002e9c4 as usize);
        set(obj(24), 0xf4, FUN_1002ea87 as usize);
        set(obj(25), 0xf4, FUN_1002ed27 as usize);
        set(obj(29), 0xf4, FUN_1002e635 as usize);

        FUN_10015e98();
        FUN_10011120();
        DAT_100ab0a8 = i_var1.wrapping_add(0x404);
        DAT_100ab0ac = i_var1.wrapping_add(0xc);
        FUN_10067e7c();
        FUN_10033ae3();
        if DAT_100adf34.add(9).read() != 0 || DAT_100aa1ac == 0 {
            FUN_10008608();
        }
    }
}
