// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9e10: u32;
    static mut DAT_101ec430: [u8; 0];
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        hHeap: *mut core::ffi::c_void,
        dwFlags: u32,
        dwBytes: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100104e1(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: *mut i16,
) -> u32 {
    // SAFETY: globals and HeapAlloc are the game's own; param_4 points at a
    // caller-provided short; the allocation is 0x1c bytes (7 words).
    unsafe {
        if (DAT_100a9e10 as i32) < 7 {
            if param_3 == 1 {
                if param_4.read_unaligned() < 1 {
                    0xffff
                } else {
                    let puVar2 = HeapAlloc(DAT_100adf74, 1, 0x1c) as *mut u32;
                    if puVar2.is_null() {
                        0xffff
                    } else {
                        let mut puVar5 = puVar2;
                        let mut iVar4: i32 = 7;
                        while iVar4 != 0 {
                            puVar5.write_unaligned(0);
                            puVar5 = puVar5.add(1);
                            iVar4 -= 1;
                        }
                        let slot = (&raw mut DAT_101ec430)
                            .cast::<u8>()
                            .byte_add(DAT_100a9e10.wrapping_mul(4) as usize)
                            .cast::<*mut u32>();
                        slot.write_unaligned(puVar2);
                        puVar2.write_unaligned(0);
                        puVar2.add(3).write_unaligned(0);
                        puVar2.add(4).write_unaligned(param_1);
                        puVar2.add(2).write_unaligned(param_2);
                        puVar2
                            .add(5)
                            .write_unaligned(param_4.read_unaligned() as i32 as u32);
                        let uVar1 = DAT_100a9e10;
                        DAT_100a9e10 = DAT_100a9e10.wrapping_add(1);
                        uVar1
                    }
                }
            } else {
                0xffff
            }
        } else {
            0xffff
        }
    }
}
