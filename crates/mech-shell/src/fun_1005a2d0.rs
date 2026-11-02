// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6aa0: i32;
    static mut DAT_100c6aa6: i32;
    static mut DAT_100c6aca: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10059e80(param_1: u32) -> *mut u8;
    #[link_name = "FUN_10059f10"]
    fn ReadFileInChunks(param_1: i32, param_2: *mut u8, param_3: u32) -> u32;
    fn FUN_10059ea0(param_1: *mut u8);
    fn __write(param_1: i32, param_2: *mut u8, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005a2d0(param_1: i32) -> i32 {
    // SAFETY: the globals, table memory and callees are the game's own; every
    // read and write goes where the original does.
    unsafe {
        let i_var5 = param_1.wrapping_mul(0x14a);
        let mut local_a: u16 = 0;
        let i_var3: i32 = (&raw mut DAT_100c6aa6)
            .cast::<u8>()
            .byte_offset(i_var5 as isize)
            .cast::<i32>()
            .read_unaligned();
        let base = i_var3 as usize as *mut u8;
        let u_var2: u16 = base.byte_add(0xc).cast::<u16>().read_unaligned();
        if u_var2 != 0 {
            loop {
                let u_var6 = local_a as u32;
                let i_var1 = base
                    .byte_add(0xe)
                    .byte_add((u_var6.wrapping_mul(0x18)) as usize);
                let t: i32 = base
                    .byte_add(0x12)
                    .byte_add((u_var6.wrapping_mul(0x18)) as usize)
                    .cast::<i32>()
                    .read_unaligned();
                if t != 0 {
                    let u_var4: u32 = i_var1.byte_add(8).cast::<u32>().read_unaligned();
                    let pv = FUN_10059e80(u_var4);
                    if pv.is_null() {
                        return -1;
                    }
                    let temp_int: i32 = (&raw mut DAT_100c6aa0)
                        .cast::<u8>()
                        .byte_offset(i_var5 as isize)
                        .cast::<i32>()
                        .read_unaligned();
                    let temp_ptr: *mut u8 = i_var1.byte_add(4).cast::<*mut u8>().read_unaligned();
                    __write(temp_int, temp_ptr, 0);
                    let u_var8 = ReadFileInChunks(temp_int, pv, u_var4);
                    if u_var8 != u_var4 {
                        FUN_10059ea0(pv);
                        return -1;
                    }
                    (&raw mut DAT_100c6aca)
                        .cast::<u8>()
                        .byte_add(u_var6.wrapping_mul(8) as usize)
                        .byte_offset(i_var5 as isize)
                        .cast::<*mut u8>()
                        .write_unaligned(pv);
                }
                local_a = local_a.wrapping_add(1);
                if local_a >= u_var2 {
                    break;
                }
            }
        }
        0
    }
}
