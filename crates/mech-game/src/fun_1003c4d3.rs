// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb0: u32;
    static mut DAT_100b3c54: u32;
    static mut DAT_10160e80: u8;
    static mut DAT_10160e90: u8;
    static mut DAT_100af0c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_sequence_status_4(a: u32) -> u32;
    fn _AIL_release_sequence_handle_4(a: u32) -> u32;
    fn _AIL_allocate_sequence_handle_4(a: u32) -> u32;
    fn _AIL_init_sequence_12(a: u32, b: u32, c: u32) -> u32;
    fn _AIL_set_sequence_volume_12(a: u32, b: u32, c: u32) -> u32;
    fn _AIL_start_sequence_4(a: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c4d3(param_1: i32, param_2: i16, param_3: i16) -> u32 {
    let mut local_10: i32 = -1;
    if param_2 < 1 {
        return 0;
    }
    // SAFETY: the globals, arrays and callees are the game's own memory and
    // functions; slots are indexed 0..8 as in the original.
    unsafe {
        let seqs = (&raw mut DAT_10160e90) as *mut u8;
        let ids = (&raw mut DAT_10160e80) as *mut u8;
        let mut local_c: i32 = 0;
        while local_c < 8 {
            let seq_slot = seqs
                .byte_add(local_c.wrapping_mul(4) as usize)
                .cast::<u32>();
            let id_slot = ids.byte_add(local_c.wrapping_mul(2) as usize).cast::<u16>();
            let mut local_14: u32 = seq_slot.read_unaligned();
            if local_14 != 0xffff_ffff {
                let i_var2 = _AIL_sequence_status_4(local_14);
                if i_var2 == 2 {
                    _AIL_release_sequence_handle_4(local_14);
                    seq_slot.write_unaligned(0xffff_ffff);
                    local_14 = seq_slot.read_unaligned();
                    FUN_1008664f(
                        (id_slot.read_unaligned() as i16 as i32) as u32,
                        DAT_100af0c4 as *mut i32,
                    );
                    id_slot.write_unaligned(0);
                }
            }
            if local_14 == 0xffff_ffff {
                local_10 = local_c;
            }
            local_c = local_c.wrapping_add(1);
        }
        if local_10 == -1 {
            return 0xffff_fff8;
        }
        let puVar3 = FUN_1008668b(param_1, param_2 as i32 as u32, DAT_100af0c4 as *mut i32);
        if puVar3.is_null() || DAT_100aefb0 == 0 {
            return 0;
        }
        let i_var2 = _AIL_allocate_sequence_handle_4(DAT_100aefb0);
        if i_var2 == 0 {
            FUN_1008664f(param_2 as i32 as u32, DAT_100af0c4 as *mut i32);
            return 0xffff_fff6;
        }
        seqs.byte_add(local_10.wrapping_mul(4) as usize)
            .cast::<u32>()
            .write_unaligned(i_var2);
        _AIL_init_sequence_12(i_var2, puVar3 as u32, param_3 as i32 as u32);
        let i_var4 = FUN_10003d60(DAT_100b3c54 as i32, 0x7f);
        _AIL_set_sequence_volume_12(i_var2, i_var4 as u32, 0);
        _AIL_start_sequence_4(i_var2);
        1
    }
}
