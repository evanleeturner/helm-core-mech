// SPDX-License-Identifier: MIT
//! A line's numbers, as the window rows, the slots and the pass-through show them. A file of
//! its own, so the Wine test compiles it beside `through.rs` and never links the layer's
//! library.
use core::fmt;

/// Numbers as a line shows them, comma-joined: `a`'s, or `o`'s record.
pub(crate) struct Numbers<'a>(pub(crate) &'a [u32]);

impl fmt::Display for Numbers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().enumerate().try_for_each(|(index, number)| {
            if index > 0 {
                f.write_str(",")?;
            }
            mech_trace::__shown!(*number).fmt(f)
        })
    }
}
