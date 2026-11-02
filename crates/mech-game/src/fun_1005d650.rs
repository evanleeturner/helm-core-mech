// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_10114db0: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005d650(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the caller's structure; the globals and the
    // callee are the game's own, and the table entries are object pointers
    // whose vtable-like slot at +0xe4 holds a function pointer.
    unsafe {
        let base = *param_1 as u32;
        let v = (base.wrapping_add(4) as *const u32).read_unaligned();
        if v == DAT_100adf38 {
            let local_8: i32 = if param_2 == 0 { 2 } else { 5 };
            let mut local_c: i32 = 0;
            while local_c < 0x22 {
                let i_var1 = FUN_10051f43(10);
                if i_var1 < local_8 {
                    let tbl = (&raw mut DAT_10114db0).cast::<u32>();
                    let obj = tbl.offset(local_c as isize).read_unaligned();
                    let f = ((obj.wrapping_add(0xe4)) as *const u32).read_unaligned();
                    let func: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
                    let s = ((obj.wrapping_add(6)) as *const i16).read_unaligned();
                    func(obj, (s as i32).wrapping_add(1));
                }
                local_c = local_c.wrapping_add(1);
            }
        }
    }
}
