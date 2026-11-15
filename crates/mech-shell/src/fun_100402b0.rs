// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040300(text_object: *mut u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050d90(
        this: *mut u8,
        text_object: i32,
        param_2: i32,
        param_3: *mut u8,
        param_4: *mut u8,
    ) -> u8;
}

#[unsafe(export_name = "FUN_100402b0")]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn DrawTextItem(item: *mut u32) {
    mech_trace::enter!("100402b0", item);
    // A hot port that calls out: its `fn.in` stays at its entry, its
    // callees' lines inside the call, and its reads are counted into one
    // `fn.reads` line at each return in place of a `mem.rd` a read.
    let mut reads: u32 = 0;
    // SAFETY: item points at the game's text object; fields are read
    // and written at the offsets the original uses, and the string is scanned
    // up to its terminator.
    unsafe {
        'call: {
            let base = item.cast::<u8>();
            reads += 1;
            let text = base.byte_add(0x19).cast::<*const i8>().read_unaligned();
            if !text.is_null() {
                let mut countdown: i32 = -1;
                let mut cursor = text;
                loop {
                    if countdown == 0 {
                        break;
                    }
                    countdown = countdown.wrapping_sub(1);
                    reads += 1;
                    let c = cursor.read();
                    cursor = cursor.add(1);
                    if c == 0 {
                        break;
                    }
                }
                if countdown != -2 {
                    reads += 1;
                    if item.byte_add(0x10).read_unaligned() == 1 {
                        mech_trace::cr!("100402b0", "10040300", FUN_10040300(item));
                        break 'call;
                    }
                    // The five reads below, none of them past a branch.
                    reads += 5;
                    let text_context = item.read_unaligned() as *mut u8;
                    let x = base.byte_add(0x25).cast::<i32>().read_unaligned();
                    let y = base.byte_add(0x29).cast::<i32>().read_unaligned();
                    let s = base.byte_add(0x19).cast::<*mut u8>().read_unaligned();
                    let p4 = item.add(1).read_unaligned() as *mut u8;
                    mech_trace::cr!(
                        "100402b0",
                        "10050d90",
                        FUN_10050d90(text_context, x, y, s, p4)
                    );
                    mech_trace::wr!("100402b0", base.byte_add(0x35), write, 1);
                }
            }
        }
    }
    mech_trace::log::debug!(
        f = "100402b0", a:% = mech_trace::Args([mech_trace::__shown!(item)]),
        n:% = mech_trace::__shown!(reads); "fn.reads"
    );
}
