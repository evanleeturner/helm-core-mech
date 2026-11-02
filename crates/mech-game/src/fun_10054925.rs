// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafc8: i32;
    static mut DAT_100b0f18: i32;
    static mut DAT_100b0f1c: i32;
    static mut DAT_100b0f20: i32;
    static mut DAT_10115150: i32;
    static mut DAT_10115154: i32;
    static mut DAT_1011515c: i32;
    static mut DAT_10115170: i32;
    static mut DAT_10115174: i32;
    static mut DAT_10115178: i32;
    static mut DAT_1016828c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10035f13();
    fn FUN_10054330(param_1: i32, param_2: i32);
    fn FUN_10054a76();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054925(param_1: i32) {
    // SAFETY: the callees and globals are the game's own memory; param_1 and
    // the list nodes it links point at game structures, accessed as the
    // original does.
    unsafe {
        FUN_10035f13();
        if param_1 != 0 {
            let p = param_1 as *const u8;
            let f8 = p.byte_add(8).cast::<i32>().read_unaligned();
            if f8 != 0 && p.byte_add(4).cast::<i32>().read_unaligned() != f8 {
                DAT_100b0f18 = 0;
                DAT_100b0f1c = 0;
                DAT_1011515c = 1;
                DAT_10115170 = DAT_1016828c;
                let mut local_8 = f8 as *mut u16;
                while !local_8.is_null() {
                    DAT_10115178 = DAT_10115178.wrapping_add(1);
                    let func = DAT_100aafc8 as usize;
                    let f: unsafe extern "C" fn(*mut u16) -> i32 = core::mem::transmute(func);
                    // SAFETY: DAT_100aafc8 holds a function pointer.
                    let iv = f(local_8);
                    if iv == 0 {
                        DAT_10115150 = DAT_10115150.wrapping_add(1);
                        let v = local_8.read_unaligned();
                        DAT_10115174 = v as u32 as i32;
                        if v & 0x100 == 0 {
                            FUN_10054330(local_8 as i32, DAT_10115154);
                            if DAT_1011515c == 0 || DAT_100b0f20 <= DAT_100b0f1c {
                                break;
                            }
                        } else {
                            local_8.write_unaligned(v | 0x8000);
                            let base = DAT_100b0f18.wrapping_mul(0x20).wrapping_add(DAT_10115170);
                            (base as usize as *mut i32).write_unaligned(local_8 as i32);
                            (base.wrapping_add(4) as usize as *mut i32)
                                .write_unaligned(DAT_10115154);
                            DAT_100b0f18 = DAT_100b0f18.wrapping_add(1);
                        }
                    }
                    local_8 =
                        local_8.byte_add(4).cast::<i32>().read_unaligned() as usize as *mut u16;
                }
                FUN_10054a76();
            }
        }
    }
}
