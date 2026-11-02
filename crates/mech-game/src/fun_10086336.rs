// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ba7bc: i32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100c9054: *mut u16;
    static mut DAT_100b0a08: i32;
    static mut DAT_100c9050: i32;
    static mut s_Freeing_bad_item__type___s_id____100ba7c4: [i8; 1];
    static mut s_Freeing_bad_item__type___s_id____100ba7ec: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10085fb3(param_1: i32);
    fn FUN_100108a0(param_1: u32, param_2: *mut i8);
    fn FUN_1008607d();
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

unsafe extern "stdcall" {
    fn HeapFree(heap: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086336(mem: *mut u16) {
    mech_trace::enter!("10086336", mem);
    if mem.is_null() {
        return;
    }
    // SAFETY: mem is the game's own heap item; the globals, hash table and
    // chain nodes are the game's memory, accessed at the original's offsets.
    unsafe {
        let mut local_78: [i8; 100] = [0; 100];
        let mut local_10: [i8; 5] = [0; 5];
        let mut local_8: i32;

        mech_trace::wr!("10086336", mem.add(1), write, 0);
        FUN_10085fb3(mem as i32);
        local_10[4] = 0;
        let u_var1: u32 = mech_trace::rd!("10086336", mem.add(2).cast::<u32>(), read_unaligned);
        local_10[2] = (u_var1 >> 0x10) as i8;
        local_10[3] = (u_var1 >> 0x18) as i8;
        local_10[0] = u_var1 as i8;
        local_10[1] = (u_var1 >> 8) as i8;
        let sum: u32 = (local_10[2] as i32 as u32)
            .wrapping_add(local_10[3] as i32 as u32)
            .wrapping_add(local_10[0] as i32 as u32)
            .wrapping_add(local_10[1] as i32 as u32)
            .wrapping_add(mech_trace::rd!("10086336", mem, read) as u32);
        let local_14: i32 = (sum as i32) % 0x3f1;
        let bytes = u_var1.to_le_bytes();
        local_10[0] = bytes[0] as i8;
        local_10[1] = bytes[1] as i8;
        local_10[2] = bytes[2] as i8;
        local_10[3] = bytes[3] as i8;

        let slot = mech_trace::gr!("10086336", "100ba7bc", DAT_100ba7bc)
            .wrapping_add(local_14.wrapping_mul(4)) as *mut u32;
        if (mech_trace::rd!("10086336", slot, read_unaligned) as usize as *mut u16) == mem {
            mech_trace::wr!(
                "10086336",
                slot,
                write_unaligned,
                mech_trace::rd!("10086336", mem.add(4).cast::<u32>(), read_unaligned)
            );
        } else {
            local_8 = mech_trace::rd!("10086336", slot, read_unaligned) as i32;
            loop {
                if local_8 == 0 {
                    break;
                }
                let next = mech_trace::rd!(
                    "10086336",
                    ((local_8 as u32).wrapping_add(8) as *const i32),
                    read_unaligned
                );
                if next == 0 || (next as u32 as usize as *mut u16) == mem {
                    break;
                }
                local_8 = next;
            }
            let next = if local_8 != 0 {
                mech_trace::rd!(
                    "10086336",
                    ((local_8 as u32).wrapping_add(8) as *const i32),
                    read_unaligned
                )
            } else {
                0
            };
            if local_8 == 0 || next == 0 {
                FUN_100108a0(
                    0x20,
                    (&raw mut s_Freeing_bad_item__type___s_id____100ba7c4).cast::<i8>(),
                );
                if mem == mech_trace::gr!("10086336", "100c9054", DAT_100c9054) {
                    FUN_1008607d();
                }
                if mech_trace::gr!("10086336", "100b0a08", DAT_100b0a08) == 0 {
                    return;
                }
                mech_trace::cr!(
                    "10086336",
                    "_sprintf",
                    _sprintf(
                        local_78.as_mut_ptr(),
                        (&raw const s_Freeing_bad_item__type___s_id____100ba7ec).cast::<i8>(),
                        local_10.as_ptr(),
                        mech_trace::rd!("10086336", mem, read) as u32,
                    )
                );
                mech_trace::cr!(
                    "10086336",
                    "1005be24",
                    FUN_1005be24(local_78.as_mut_ptr(), 1, 0x712, 100)
                );
                return;
            }
            let node_next = (next as u32).wrapping_add(8) as *const u32;
            mech_trace::wr!(
                "10086336",
                ((local_8 as u32).wrapping_add(8) as *mut u32),
                write_unaligned,
                mech_trace::rd!("10086336", node_next, read_unaligned)
            );
        }
        mech_trace::cr!(
            "10086336",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("10086336", "100adf74", DAT_100adf74),
                1,
                mem.cast()
            )
        );
        mech_trace::gw!(
            "10086336",
            "100c9050",
            DAT_100c9050,
            mech_trace::gr!("10086336", "100c9050", DAT_100c9050).wrapping_sub(1)
        );
    }
}
