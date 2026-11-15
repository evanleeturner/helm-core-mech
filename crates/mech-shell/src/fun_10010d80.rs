// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10010e80(this: *mut u8, sample: u32);
    fn FUN_10010e30(this: *mut u8, sample: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    #[link_name = "\u{1}_HC6_AIL_start_sample@4"]
    fn AIL_start_sample(handle: u32);
}

/// The entry the game's calls reach: pushes `ebx`, then `esi`, as the
/// original's first two instructions do (`10010d80`, `10010d81`), runs
/// [`body`] and pops them again. The words are pushed for the stack they
/// leave, not for the body: the EQUIPT tab's press calls this at
/// `10034f0e`, then `10033e80`, which, when the old list has no lit row,
/// reads its own local at `esp + 8` (`10033eac`), a word it never writes,
/// and lights the row whose key equals it. That word is the one the
/// original's `push esi` left, the caller's `esi`; a Rust prologue leaves
/// another register there, and another row is lit.
#[unsafe(naked)]
#[unsafe(export_name = "FUN_10010d80")]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn StartSample(sample: *mut u8) {
    core::arch::naked_asm!(
        "push ebx",
        "push esi",
        "call {body}",
        "pop esi",
        "pop ebx",
        "ret",
        body = sym body,
    )
}

/// The port's work, entered only through [`StartSample`]: `ecx` still
/// holds the sample.
unsafe extern "fastcall" fn body(sample: *mut u8) {
    mech_trace::enter!("10010d80", sample);
    // SAFETY: sample points at the game's sample object; fields are read and
    // written at the original's offsets, and the callees are the game's own.
    unsafe {
        if mech_trace::rd!(
            "10010d80",
            sample.byte_add(0x29).cast::<i32>(),
            read_unaligned
        ) != 0
        {
            FUN_10010e80(
                sample,
                mech_trace::rd!(
                    "10010d80",
                    sample.byte_add(0x10).cast::<u32>(),
                    read_unaligned
                ),
            );
            FUN_10010e30(
                sample,
                mech_trace::rd!(
                    "10010d80",
                    sample.byte_add(0xc).cast::<i32>(),
                    read_unaligned
                ),
            );
            mech_trace::cr!(
                "10010d80",
                "AIL_start_sample",
                AIL_start_sample(mech_trace::rd!(
                    "10010d80",
                    sample.byte_add(0x29).cast::<u32>(),
                    read_unaligned
                ))
            );
            mech_trace::wr!("10010d80", sample.byte_add(0x28), write, 1);
        }
    }
}
