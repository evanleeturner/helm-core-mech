// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10064509() {
    // SAFETY: the globals are the game's own memory; the object at iVar1 is
    // read at the offsets the original uses, and the callback is called
    // only when non-null, as the original does.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let i_var1 = base.add(DAT_100adf38 as usize).read_unaligned();
        if i_var1 != 0 {
            let obj = i_var1 as *const u8;
            let func = obj.byte_add(0x38).cast::<u32>().read_unaligned();
            if func != 0 {
                let arg = obj.byte_add(0x20).cast::<u32>().read_unaligned();
                if arg != 0 {
                    let f: extern "C" fn(u32) = core::mem::transmute(func as usize);
                    let func2 = obj.byte_add(0x38).cast::<u32>().read_unaligned();
                    let f2: extern "C" fn(u32) = core::mem::transmute(func2 as usize);
                    let _ = f;
                    let arg2 = obj.byte_add(0x20).cast::<u32>().read_unaligned();
                    f2(arg2);
                }
            }
        }
    }
}
