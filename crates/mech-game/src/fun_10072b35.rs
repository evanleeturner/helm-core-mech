// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d8e0() -> i32;
    fn FUN_1005db1f(param_1: i32) -> *mut c_void;
    fn _strchr(str: *const i8, c: i32) -> *mut i8;
    fn _sscanf(buffer: *const i8, format: *const i8, ...) -> i32;
    fn __ftol() -> i64;
    fn _atoi(str: *const i8) -> i32;
    fn FUN_10032c10(param_1: i32) -> i32;
    fn FUN_1000c8ee(param_1: i32) -> i32;
    fn FUN_1000c9e3(param_1: i32) -> *mut c_void;
    fn FUN_1001d5cb(param_1: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002f6a8(param_1: *mut c_void, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
}

unsafe fn ftol_low(f: *const f32) -> i32 {
    let lo: i32;
    // SAFETY: loads the float onto the x87 stack, then calls the runtime's
    // __ftol which consumes it and returns the result in edx:eax.
    unsafe {
        core::arch::asm!(
            "fld dword ptr [{p}]",
            "call {f}",
            p = in(reg) f,
            f = sym __ftol,
            out("eax") lo,
            out("edx") _,
            clobber_abi("C"),
        );
    }
    lo
}

unsafe fn idiv(num: i32, den: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's idiv, faulting on zero as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {d}",
            d = in(reg) den,
            inout("eax") num => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10072b35(param_1: i32, param_2: *mut i8, param_3: i32) -> i32 {
    // SAFETY: the callees and globals are the game's own; pointers are the
    // caller's and the heap block of 0x1c bytes, accessed as the original does.
    unsafe {
        if param_1 == 0 {
            let local_10 = HeapAlloc(DAT_100adf74, 1, 0x1c) as *mut i32;
            if local_10.is_null() {
                return 0;
            }
            let i_var3 = FUN_1005d8e0();
            let p_var1 = FUN_1005db1f(i_var3) as *mut *mut i32;
            p_var1.write_unaligned(local_10);
            local_10.add(4).write_unaligned(0);
            let v = local_10.add(4).read_unaligned();
            local_10.add(3).write_unaligned(v);
            let v = local_10.add(3).read_unaligned();
            local_10.add(2).write_unaligned(v);
            local_10.add(1).write_unaligned(0);
            let mut local_c = _strchr(param_2 as *const i8, 0x3b);
            if !local_c.is_null() {
                *local_c = 0;
                local_c = local_c.add(1);
                let mut local_14: f32 = 0.0;
                let mut local_18: f32 = 0.0;
                let mut local_1c: f32 = 0.0;
                let mut local_20: f32 = 0.0;
                _sscanf(
                    local_c as *const i8,
                    b"%f,%f,%f,%f\0".as_ptr() as *const i8,
                    &raw mut local_14,
                    &raw mut local_18,
                    &raw mut local_1c,
                    &raw mut local_20,
                );
                local_10
                    .add(2)
                    .write_unaligned(ftol_low(&raw const local_14));
                local_10
                    .add(3)
                    .write_unaligned(ftol_low(&raw const local_18));
                local_10
                    .add(4)
                    .write_unaligned(ftol_low(&raw const local_1c));
                local_10
                    .add(5)
                    .write_unaligned(ftol_low(&raw const local_20));
                if local_10.add(5).read_unaligned() == 0 {
                    local_10.add(5).write_unaligned(0xb5);
                }
                local_10.add(6).write_unaligned(param_3);
            }
            let i_var3 = _atoi(param_2 as *const i8);
            let i_var3 = FUN_10032c10(i_var3);
            let i_var3 = FUN_1000c8ee(i_var3);
            if i_var3 == -1 {
                return 0;
            }
            let p_var2 = FUN_1000c9e3(i_var3);
            local_10.write_unaligned(p_var2 as usize as i32);
        } else if param_1 == 1 {
            let i_var3 = FUN_1005d8e0();
            let local_10 = (FUN_1005db1f(i_var3) as *const *mut i32).read_unaligned();
            if local_10.is_null() {
                return 0;
            }
            if local_10.read_unaligned() == 0 {
                return 0;
            }
            let inner = local_10.read_unaligned();
            if (inner as usize as *const i32).read_unaligned() == 0 {
                return 0;
            }
            let i_var3 = FUN_1001d5cb((inner as usize as *const i32).read_unaligned());
            local_10.add(1).write_unaligned(i_var3);
            let num = param_3
                .wrapping_sub(local_10.add(6).read_unaligned())
                .wrapping_mul(0x10000);
            let i_var3 = idiv(num, local_10.add(5).read_unaligned());
            let i_var4 = FUN_10003d60(local_10.add(2).read_unaligned(), i_var3);
            let i_var5 = FUN_10003d60(local_10.add(3).read_unaligned(), i_var3);
            let local_8 = FUN_10003d60(local_10.add(4).read_unaligned(), i_var3);
            local_10.add(6).write_unaligned(param_3);
            FUN_1002f6a8(
                local_10.add(1).read_unaligned() as usize as *mut c_void,
                i_var4,
                i_var5,
                local_8,
                0,
            );
            FUN_1002fbc0(local_10.add(1).read_unaligned() as usize as *mut i32);
        }
    }
    1
}
