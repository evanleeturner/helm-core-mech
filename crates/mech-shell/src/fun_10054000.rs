// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bdf90: [u8; 0];
    static mut _DAT_1009c68c: u32;
    static mut DAT_1009c690: *mut u8;
    static mut DAT_1009c698: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10053fc0(param_1: u32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10054000() {
    mech_trace::enter!("10054000");
    let mut record_offset: i32 = 0;
    let mut groups_left: i32 = 4;
    // SAFETY: the globals and the callee are the game's own; the record table
    // is walked in 0x18-byte steps exactly as the original does.
    unsafe {
        loop {
            let mut index_in_group: u32 = 0;
            loop {
                let record = (&raw mut DAT_100bdf90)
                    .cast::<u8>()
                    .byte_add(record_offset as usize)
                    .cast::<u32>();
                record_offset = (record_offset as u32).wrapping_add(0x18) as i32;
                let next_index = index_in_group.wrapping_add(1);
                FUN_10053fc0(index_in_group, record);
                index_in_group = next_index;
                if next_index >= 0x25 {
                    break;
                }
            }
            groups_left = groups_left.wrapping_sub(1);
            if groups_left == 0 {
                break;
            }
        }
        mech_trace::gw!("10054000", "1009c68c", _DAT_1009c68c, 0);
        mech_trace::gw!("10054000", "1009c698", DAT_1009c698, 0);
        mech_trace::gw!(
            "10054000",
            "1009c690",
            DAT_1009c690,
            (&raw mut DAT_100bdf90).cast::<u8>()
        );
    }
}
