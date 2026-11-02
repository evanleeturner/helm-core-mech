// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c558: u32;
    static mut DAT_1009c6a8: u32;
    static mut DAT_100bdf94: [u8; 1];
    static mut DAT_100bed74: [u8; 1];
    static mut DAT_100bfd98: [u32; 16];
    static mut DAT_1009fe94: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100576a0(param_1: i32) -> i32;
    fn FUN_10053fc0(param_1: i32, param_2: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10053cf0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are walked exactly as the original does.
    unsafe {
        let slots = (&raw mut DAT_100bfd98).cast::<u32>();
        let mut index: i32 = 0;
        loop {
            let i_var2 = FUN_100576a0(index);
            if i_var2 == 0 {
                slots.add(index as usize).write(0);
            } else {
                let mut a = (i_var2.wrapping_add(4) as u32 as usize) as *const u8;
                let mut b = (&raw const DAT_1009fe94).cast::<u8>();
                let mut equal = true;
                loop {
                    let ca = a.read();
                    let cb = b.read();
                    if ca != cb {
                        equal = false;
                        break;
                    }
                    if ca == 0 {
                        break;
                    }
                    a = a.add(1);
                    b = b.add(1);
                }
                if equal {
                    slots.add(index as usize).write(1);
                } else {
                    slots.add(index as usize).write(0);
                }
            }
            index = index.wrapping_add(1);
            if 0xf < index {
                index = 0;
                let mut record_cursor = (&raw mut DAT_100bdf94).cast::<i32>();
                loop {
                    let v = record_cursor.read_unaligned();
                    if -1 < v {
                        DAT_1009c558 = v as u32;
                        let r = FUN_100576a0(v);
                        if r == 0 {
                            FUN_10053fc0(index, record_cursor.wrapping_sub(1).cast::<u32>());
                        } else {
                            slots.add(DAT_1009c558 as usize).write(1);
                        }
                    }
                    record_cursor = record_cursor.wrapping_add(6);
                    index = index.wrapping_add(1);
                    if !(record_cursor < (&raw mut DAT_100bed74).cast::<i32>()) {
                        break;
                    }
                }
                index = 0;
                DAT_1009c6a8 = 0;
                loop {
                    let r = FUN_100576a0(index);
                    if r == 0 || slots.add(index as usize).read() == 0 || 3 < DAT_1009c6a8 as i32 {
                        slots.add(index as usize).write(0);
                    } else {
                        DAT_1009c6a8 = DAT_1009c6a8.wrapping_add(1);
                        DAT_1009c558 = index as u32;
                    }
                    index = index.wrapping_add(1);
                    if !(index < 0x10) {
                        break;
                    }
                }
                return;
            }
        }
    }
}
