// SPDX-License-Identifier: MIT
//! The swap: the shell library's 580 ported functions in one library the
//! live game loads beside its own shell (HC5, `JOB.md` I1). Each port is
//! `mech-shell`'s module, copied by `build.rs` with its definition renamed
//! `HC5_<address>`; its globals, `DAT_<address>`, and its callees,
//! `FUN_<address>`, stay undefined here and are linked to the shell's own
//! addresses by the swap's build, so the library is valid only while the
//! shell sits at 0x10000000.
//!
//! The ports' DEBUG trace goes to a file: `DllMain` opens the path the
//! environment's `HC5_LOG` names and installs the sink, which writes each
//! record as one line of the workspace's grammar, `HH:MM:SS.mmm L event
//! k=v`, by one `WriteFile`; a line past [`LINE_BYTES`] is cut there, its
//! newline kept, and a line that cannot be written is dropped. No
//! `HC5_LOG`, no trace.
//!
//! 32-bit x86 Windows only: on any other target the crate is [`body`]
//! alone, for the host test that holds it equal to `mech_logline::body`.
#![no_std]

#[cfg(test)]
extern crate std;

use core::fmt::{self, Write as _};

use log::Record;
use log::kv::{Error, Key, Value, VisitSource};

#[cfg(all(target_arch = "x86", target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/ports.rs"));

pub mod bind;
pub mod listen;

#[cfg(test)]
mod test_log;

#[cfg(all(target_arch = "x86", target_os = "windows", not(test)))]
mod listen_live;

#[cfg(all(target_arch = "x86", target_os = "windows", not(test)))]
mod live;

#[cfg(all(target_arch = "x86", target_os = "windows", not(test)))]
mod windows;

// The layer (HC6): linked in so the staticlib carries its exports.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
use mech_layer as _;

/// The bytes one trace line may take, its newline included: room for the
/// longest row the swap writes, `bind.swap` with every port's address in its
/// setting, about 5,300 bytes. Two lines sit on the writing thread's stack
/// while a record is written, the sink's and [`body`]'s for a field's value.
pub const LINE_BYTES: usize = 8192;

/// A fixed buffer that takes what fits and drops the rest, its last byte
/// kept for the newline [`Line::end`] writes.
pub struct Line {
    bytes: [u8; LINE_BYTES],
    len: usize,
}

impl Line {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: [0; LINE_BYTES],
            len: 0,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    /// Ends the line with its newline, in the byte a write never takes, so a
    /// line cut at [`LINE_BYTES`] still ends and the next is never glued to
    /// it. Once a line: a full line takes no second.
    pub fn end(&mut self) {
        if self.len < LINE_BYTES {
            self.bytes[self.len] = b'\n';
            self.len += 1;
        }
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Line {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let take = text.len().min((LINE_BYTES - 1).saturating_sub(self.len));
        self.bytes[self.len..self.len + take].copy_from_slice(&text.as_bytes()[..take]);
        self.len += take;
        Ok(())
    }
}

/// Writes the line's text after its time and level into `line`: the event
/// id, then each field as `key=value`, a value with a space, a quote or `=`
/// double-quoted, as `mech_logline::body` renders it.
pub fn body(record: &Record<'_>, line: &mut Line) {
    struct Fields<'a>(&'a mut Line);
    impl<'kvs> VisitSource<'kvs> for Fields<'_> {
        fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), Error> {
            let mut text = Line::new();
            let _ = write!(text, "{value}");
            // A value cut at LINE_BYTES may end inside a character: keep
            // the whole ones.
            let shown = match core::str::from_utf8(text.as_bytes()) {
                Ok(shown) => shown,
                Err(cut) => {
                    core::str::from_utf8(&text.as_bytes()[..cut.valid_up_to()]).unwrap_or_default()
                }
            };
            if shown.is_empty() || shown.contains([' ', '"', '=']) {
                let _ = write!(self.0, " {key}=\"");
                for c in shown.chars() {
                    let _ = match c {
                        '\\' => self.0.write_str("\\\\"),
                        '"' => self.0.write_str("\\\""),
                        _ => self.0.write_char(c),
                    };
                }
                let _ = self.0.write_char('"');
            } else {
                let _ = write!(self.0, " {key}={value}");
            }
            Ok(())
        }
    }
    let _ = write!(line, "{}", record.args());
    let _ = record.key_values().visit(&mut Fields(line));
}

#[cfg(test)]
mod tests {
    use log::kv::Value;
    use log::{Level, Record};

    use super::{LINE_BYTES, Line, body};

    /// The 580 ports' addresses as `build.rs` takes them from `mech-shell`'s
    /// `lib.rs`: each `mod fun_<address>;` or `mod swap_<address>;` under
    /// `#[cfg(target_arch = "x86")]`.
    fn port_addresses() -> std::vec::Vec<&'static str> {
        let lib = include_str!("../../mech-shell/src/lib.rs");
        let mut addresses = std::vec::Vec::new();
        let mut gated = false;
        for line in lib.lines().map(str::trim) {
            let module = line.strip_prefix("mod ").and_then(|m| m.strip_suffix(';'));
            if let (true, Some(module)) = (gated, module) {
                let address = module
                    .strip_prefix("fun_")
                    .or_else(|| module.strip_prefix("swap_"))
                    .expect("a gated module is fun_<address> or swap_<address>");
                addresses.push(address);
            }
            gated = line == "#[cfg(target_arch = \"x86\")]";
        }
        assert_eq!(addresses.len(), 580, "mech-shell gates 580 ports");
        addresses
    }

    /// The sink's line for `record` past its time and level: `body`, then
    /// the sink's ending, as `windows.rs` writes them.
    fn ended(record: &Record<'_>) -> Line {
        let mut line = Line::new();
        body(record, &mut line);
        line.end();
        line
    }

    /// A `bind.swap` row whose setting lists every port, past 1,024 bytes:
    /// the swap's line is `mech_logline::body`'s whole, its counts in it,
    /// then its newline.
    #[test]
    fn a_full_swap_row_is_written_whole() {
        let addresses = port_addresses();
        let setting = addresses.join(",");
        assert!(setting.len() > 1024, "the setting passes 1,024 bytes");
        let count = addresses.len();
        let fields = [
            ("file", Value::from("MW2SHELL.DLL")),
            ("setting", Value::from(setting.as_str())),
            ("chosen", Value::from(count)),
            ("written", Value::from(count)),
        ];
        let record = Record::builder()
            .level(Level::Info)
            .args(format_args!("bind.swap"))
            .key_values(&fields)
            .build();
        let whole = mech_logline::body(&record);
        let line = ended(&record);
        let shown = std::str::from_utf8(line.as_bytes()).expect("the line is UTF-8");
        assert_eq!(shown, std::format!("{whole}\n"));
        assert!(shown.ends_with(&std::format!(" chosen={count} written={count}\n")));
    }

    /// A row past `LINE_BYTES` is cut there and still ends its line: its
    /// first `LINE_BYTES - 1` bytes are `mech_logline::body`'s, then the
    /// newline, so the next row is never glued to it.
    #[test]
    fn a_row_past_the_cap_keeps_its_newline() {
        let ports = port_addresses().join(",");
        let setting = [ports.as_str(), ports.as_str()].join(",");
        let fields = [("file", "MW2SHELL.DLL"), ("setting", setting.as_str())];
        let record = Record::builder()
            .level(Level::Info)
            .args(format_args!("bind.swap"))
            .key_values(&fields)
            .build();
        let whole = mech_logline::body(&record);
        assert!(whole.len() > LINE_BYTES, "the row passes the cap");
        let mut cut = std::vec::Vec::from(&whole.as_bytes()[..LINE_BYTES - 1]);
        cut.push(b'\n');
        assert_eq!(ended(&record).as_bytes(), cut.as_slice());
    }

    /// The swap's line holds `mech_logline::body`'s for one record whose
    /// fields take every rule: plain, a space, a quote, `=`, and empty.
    #[test]
    fn body_equals_the_logline_adapters() {
        let fields: [(&str, &str); 5] = [
            ("f", "10004330"),
            ("v", "a b"),
            ("q", "say \"hi\""),
            ("e", "k=v"),
            ("z", ""),
        ];
        let record = Record::builder()
            .level(Level::Debug)
            .args(format_args!("fn.in"))
            .key_values(&fields)
            .build();
        let mut line = Line::new();
        body(&record, &mut line);
        assert_eq!(
            std::str::from_utf8(line.as_bytes()).expect("the line is UTF-8"),
            mech_logline::body(&record)
        );
    }
}
