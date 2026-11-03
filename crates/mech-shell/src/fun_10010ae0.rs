// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10054f20(
        this: *mut c_void,
        sample_owner: i32,
        pack_database: *mut u32,
        param_3: *mut i32,
    ) -> u32;
    fn FUN_10010890(this: *mut c_void, sample_owner: u32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100107a0(sample_owner: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_allocate_sample_handle_4(param_1: u32) -> u32;
    fn HeapFree(heap: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
    fn _AIL_init_sample_4(param_1: u32) -> u32;
    fn _AIL_set_sample_file_12(param_1: u32, param_2: u32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10010ae0(
    sample: *mut c_void,
    sample_owner: *mut i8,
    pack_database: *mut c_void,
    param_3: i32,
) -> *mut i32 {
    mech_trace::enter!("10010ae0", sample, sample_owner, pack_database, param_3);
    let mut item_data: i32 = 0;
    let mut local_4: i32 = -1;
    let s = sample as *mut u8;
    // SAFETY: the callees and globals are the game's own; sample points at the
    // caller's object, accessed at the same offsets as the original.
    unsafe {
        mech_trace::cr!(
            "10010ae0",
            "10054f20",
            FUN_10054f20(
                pack_database,
                param_3,
                (&raw mut item_data).cast::<u32>(),
                &raw mut local_4,
            )
        );
        mech_trace::wr!(
            "10010ae0",
            s.cast::<*mut i8>(),
            write_unaligned,
            sample_owner
        );
        mech_trace::wr!("10010ae0", s.byte_add(0x28), write, 0);
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(4).cast::<i32>(),
            write_unaligned,
            item_data
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(8).cast::<i32>(),
            write_unaligned,
            local_4
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x29).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0xc).cast::<u32>(),
            write_unaligned,
            1
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x10).cast::<u32>(),
            write_unaligned,
            1
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x18).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x14).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x1c).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x20).cast::<u32>(),
            write_unaligned,
            1
        );
        mech_trace::wr!(
            "10010ae0",
            s.byte_add(0x24).cast::<u32>(),
            write_unaligned,
            1
        );
        if !sample_owner.is_null() && item_data != 0 && local_4 != 0 {
            let wave_out_driver =
                mech_trace::cr!("10010ae0", "100107a0", FUN_100107a0(sample_owner));
            let handle = mech_trace::cr!(
                "10010ae0",
                "_AIL_allocate_sample_handle_4",
                _AIL_allocate_sample_handle_4(wave_out_driver as u32)
            );
            mech_trace::wr!(
                "10010ae0",
                s.byte_add(0x29).cast::<u32>(),
                write_unaligned,
                handle
            );
            let owner = mech_trace::rd!("10010ae0", s.cast::<usize>(), read_unaligned);
            let flag = mech_trace::rd!(
                "10010ae0",
                ((owner as *const u8).byte_add(5)).cast::<i32>(),
                read_unaligned
            );
            if flag == 0 {
                let p = mech_trace::rd!(
                    "10010ae0",
                    s.byte_add(4).cast::<*mut c_void>(),
                    read_unaligned
                );
                if !p.is_null() {
                    mech_trace::cr!(
                        "10010ae0",
                        "HeapFree",
                        HeapFree(mech_trace::gr!("10010ae0", "10098224", DAT_10098224), 1, p)
                    );
                    mech_trace::wr!("10010ae0", s.byte_add(4).cast::<u32>(), write_unaligned, 0);
                }
            } else if mech_trace::rd!("10010ae0", s.byte_add(0x29).cast::<u32>(), read_unaligned)
                != 0
            {
                mech_trace::cr!(
                    "10010ae0",
                    "_AIL_init_sample_4",
                    _AIL_init_sample_4(mech_trace::rd!(
                        "10010ae0",
                        s.byte_add(0x29).cast::<u32>(),
                        read_unaligned
                    ))
                );
                mech_trace::cr!(
                    "10010ae0",
                    "_AIL_set_sample_file_12",
                    _AIL_set_sample_file_12(
                        mech_trace::rd!("10010ae0", s.byte_add(0x29).cast::<u32>(), read_unaligned),
                        mech_trace::rd!("10010ae0", s.byte_add(4).cast::<u32>(), read_unaligned),
                        0xffffffff,
                    )
                );
                FUN_10010890(
                    mech_trace::rd!("10010ae0", s.cast::<*mut c_void>(), read_unaligned),
                    sample as usize as u32,
                );
            }
        }
    }
    mech_trace::rt!("10010ae0", sample as *mut i32)
}
