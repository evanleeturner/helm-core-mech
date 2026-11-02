// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7850: i32;
    static mut DAT_100a6740: *mut core::ffi::c_void;
    static mut DAT_100a7fb0: i8;
    static mut DAT_100a7fc8: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10009840(param_1: *mut i8) -> *mut i8;
    fn FUN_1006e7b0(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050c30(
        this_: *mut core::ffi::c_void,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100601fe(in_zf: i32, in_stack_00000104: *mut i32) -> *mut i32 {
    if in_zf != 0 {
        return core::ptr::null_mut();
    }
    let mut buf: [i8; 0x104] = [0; 0x104];
    // SAFETY: the globals and callees are the game's own; the base pointer
    // addresses the game's structure, and in_stack_00000104 points at the
    // caller's int array.
    unsafe {
        let base = DAT_100a7850;
        let mut i_var6: i32 = -1;
        let mut pc = base.wrapping_add(0x100) as *const i8;
        loop {
            if i_var6 == 0 {
                break;
            }
            i_var6 = i_var6.wrapping_sub(1);
            let c = pc.read();
            pc = pc.add(1);
            if c == 0 {
                break;
            }
        }

        let mut i_var3 = (base.wrapping_add(0x204) as *const i32).read_unaligned();
        let mut i_var2 = (base.wrapping_add(0x200) as *const i32).read_unaligned();

        let fmt: *const i8;
        let name: *mut i8;
        if i_var6 == -2 {
            name = FUN_10009840(base.wrapping_add(0x823) as *mut i8);
            fmt = &raw mut DAT_100a7fc8;
        } else {
            name = base.wrapping_add(0x100) as *mut i8;
            fmt = &raw mut DAT_100a7fb0;
        }
        let i_var6 = i_var2 % 10;
        let i_var7 = (i_var2 / 10) % 10;
        i_var2 /= 100;
        let i_var8 = i_var3 % 10;
        let i_var9 = (i_var3 / 10) % 10;
        i_var3 /= 100;

        FUN_1006e7b0(
            buf.as_mut_ptr(),
            fmt,
            name,
            i_var3,
            i_var9,
            i_var8,
            i_var2,
            i_var7,
            i_var6,
        );

        FUN_10050c30(
            DAT_100a6740,
            in_stack_00000104.read_unaligned(),
            in_stack_00000104.add(1).read_unaligned(),
            buf.as_mut_ptr(),
            in_stack_00000104.add(5).read_unaligned(),
        )
    }
}
