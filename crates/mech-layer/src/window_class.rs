// SPDX-License-Identifier: MIT
//! A window class's record with its background brush taken out.
//!
//! `RegisterClassA` takes a `WNDCLASSA`: 10 fields of 4 bytes, the
//! background brush the 8th. When a window is erased, its inside is
//! filled with its class's brush. An erase the program never sees, one
//! the system sends while it holds the window's messages to itself, so
//! paints over the last picture, and nothing paints it back. The game's
//! pictures fill every point of its window, so its class needs no brush:
//! the window row hands the real call a copy with the brush 0, and the
//! game's own record stays as it wrote it. With no brush an erase fills
//! nothing.
//!
//! Built on every target, so a host test reads it.

/// The fields of a `WNDCLASSA`, each 4 bytes on 32-bit Windows.
pub const FIELDS: usize = 10;

/// The background brush's place among them, `hbrBackground`, from 0.
pub const BRUSH: usize = 7;

/// `class` with its background brush 0, and the brush taken out.
#[must_use]
pub const fn without_brush(class: [u32; FIELDS]) -> ([u32; FIELDS], u32) {
    let mut copy = class;
    copy[BRUSH] = 0;
    (copy, class[BRUSH])
}

#[cfg(test)]
mod tests {
    use super::{BRUSH, FIELDS, without_brush};

    /// A class whose every field holds its own place plus 1, so a field
    /// that moves or changes shows.
    const NUMBERED: [u32; FIELDS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    #[test]
    fn the_brush_comes_out_and_is_handed_back() {
        let (copy, brush) = without_brush(NUMBERED);
        assert_eq!(copy[BRUSH], 0);
        assert_eq!(brush, 8);
    }

    #[test]
    fn the_other_nine_fields_are_alike() {
        let (copy, _) = without_brush(NUMBERED);
        for (place, (field, was)) in copy.into_iter().zip(NUMBERED).enumerate() {
            if place != BRUSH {
                assert_eq!(field, was, "field {place}");
            }
        }
    }

    #[test]
    fn a_class_with_no_brush_is_unchanged() {
        let mut brushless = NUMBERED;
        brushless[BRUSH] = 0;
        assert_eq!(without_brush(brushless), (brushless, 0));
    }
}
