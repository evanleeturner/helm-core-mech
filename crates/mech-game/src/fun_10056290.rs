// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056290(param_1: *mut u32) -> u32 {
    mech_trace::enter!("10056290", param_1);
    // SAFETY: param_1 points at a game-owned slot holding a __cdecl function
    // address returning a NUL-terminated string; the destination is the
    // game's fixed buffer at 0x100b12b4, which the original writes in place.
    unsafe {
        let f: unsafe extern "C" fn() -> *const i8 =
            core::mem::transmute::<u32, unsafe extern "C" fn() -> *const i8>(mech_trace::rd!(
                "10056290",
                param_1,
                read_unaligned
            ));
        let mut pc_var2: *const i8 = mech_trace::cr!("10056290", "ind", f());
        let mut pc_var3: *mut i8 = 0x100b12b4usize as *mut i8;
        loop {
            let c_var1 = mech_trace::rd!("10056290", pc_var2, read);
            mech_trace::wr!("10056290", pc_var3, write, c_var1);
            pc_var3 = pc_var3.add(1);
            pc_var2 = pc_var2.add(1);
            if c_var1 == 0 {
                break;
            }
        }
    }
    mech_trace::rt!("10056290", 0x100b12b4)
}
