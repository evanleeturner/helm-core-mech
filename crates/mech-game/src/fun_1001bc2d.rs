// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002634e(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001bc2d(param_1: i32, param_2: *mut i32) -> i32 {
    // SAFETY: param_1 is an address of the caller's structure and param_2
    // points at the caller's int array; reads happen where the original reads.
    unsafe {
        let i_var7 = ((param_1.wrapping_add(0x40)) as *const i32).read_unaligned();
        if i_var7 < 1 {
            return 0x7fffffff;
        }
        let i_var3 = (((param_1.wrapping_add(0x34)) as *const i32).read_unaligned() as u32)
            .wrapping_sub(param_2.read_unaligned() as u32) as i32;
        let i_var4 = (((param_1.wrapping_add(0x38)) as *const i32).read_unaligned() as u32)
            .wrapping_sub(param_2.add(1).read_unaligned() as u32) as i32;
        let i_var5 = (((param_1.wrapping_add(0x3c)) as *const i32).read_unaligned() as u32)
            .wrapping_sub(param_2.add(2).read_unaligned() as u32) as i32;

        let l_var2 = ((i_var4 as i64).wrapping_mul(i_var4 as i64) as u64)
            .wrapping_add((i_var3 as i64).wrapping_mul(i_var3 as i64) as u64)
            .wrapping_add((i_var5 as i64).wrapping_mul(i_var5 as i64) as u64);
        let l_var6 = (i_var7 as i64).wrapping_mul(i_var7 as i64) as u64;

        if l_var2 < l_var6 {
            return 10;
        }
        let l_var9 = l_var2.wrapping_sub(l_var6);

        let l_var10 = ((i_var4 as i64).wrapping_mul(param_2.add(7).read_unaligned() as i64) as u64)
            .wrapping_add(
                (i_var3 as i64).wrapping_mul(param_2.add(6).read_unaligned() as i64) as u64,
            );
        let l_var11 = (i_var5 as i64).wrapping_mul(param_2.add(8).read_unaligned() as i64) as u64;
        let l_var12 = l_var10.wrapping_add(l_var11);

        if l_var12 < l_var10 {
            return 0x7fffffff;
        }
        let l_var14 = l_var12 as i64;
        let i_var8 = param_2.add(0xc).read_unaligned();

        let l_var15: i64;
        if i_var8 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
            l_var15 = 0;
        } else if i_var8 == -1 {
            l_var15 = 0u64.wrapping_sub(l_var14 as u64) as i64;
        } else {
            l_var15 = l_var14 / (i_var8 as i64);
        }
        if l_var15 < -2147483648i64 || l_var15 > 2147483647i64 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let mut local_1c = l_var15 as i32;

        let l_var13 = (local_1c as i64).wrapping_mul(local_1c as i64) as u64;
        if l_var13 < l_var9 {
            return 0x7fffffff;
        }
        local_1c = (local_1c as u32).wrapping_sub(i_var7 as u32) as i32;
        if local_1c < 0 {
            local_1c = 0;
        }
        let i_var7b = FUN_1002634e(param_2 as i32);
        if i_var7b <= local_1c {
            local_1c = 0x7fffffff;
        }
        local_1c
    }
}
