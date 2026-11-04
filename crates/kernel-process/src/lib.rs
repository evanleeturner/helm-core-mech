// SPDX-License-Identifier: MIT
//! The Kernel's process (step 5, rulings 1389 and 1407): the KERNEL32
//! names a program asks of its own process, written once. The start-up's
//! answers (the command line, the module's path, the version) are the
//! game's row, never the host's; a module and its entry names are the
//! stand-in's own name table, [`table::Module`], never a host library
//! load; the last error and the end are the back's, behind [`Process`].
//!
//! Each file the seam serves is a [`table::Row`], kept in its game's rows
//! crate; the back is the page's, [`page::Page`], a host's, or the
//! original's calls under Wine in `mech-layer`. [`Front`] answers in the
//! file's numbers and writes one `imp.call` line a call: `c` the row's
//! name, `a` the call's numbers, `m` the module name it was given, `q`
//! the entry name it was asked, `v` its answer, `e` the error left.
//!
//! The crate names no game and nothing of any one target, so it builds
//! for `wasm32-unknown-unknown`; a game's facts live in its rows crate.
#![cfg_attr(not(test), no_std)]

pub mod page;
pub mod table;
#[cfg(test)]
mod tests;

use core::fmt;
use mech_trace::Args;
use table::{Module, Row};

/// Where one file's process state lives: the seam between the calls and
/// a target.
pub trait Process {
    /// The number `name` in `module` answers on this back, 0 for none:
    /// the function's place in the page's table, or its address.
    fn entry(&self, module: &Module, name: &str) -> u32;
    /// Ends the run with `code`: a host's process exits; on the page the
    /// call returns and the frame loop stops.
    fn end(&self, code: u32);
    /// Keeps `code` as the last error the file reads.
    fn fail(&self, code: u32);
    /// The last error kept.
    fn last_error(&self) -> u32;
    /// Writes one line around whatever the target must keep across it.
    fn quiet(&self, line: impl FnOnce()) {
        line();
    }
}

/// A name as the line shows it: each byte one character.
pub struct Text<'a>(pub &'a [u8]);

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|byte| fmt::Write::write_char(f, char::from(*byte)))
    }
}

/// Whether the module name `asked` names `module`, as `LoadLibraryA`
/// matches one: its last path part, any case, `.DLL` taken when it has no
/// extension.
fn names(asked: &[u8], module: &str) -> bool {
    let last = asked
        .rsplit(|byte| matches!(byte, b'\\' | b'/'))
        .next()
        .unwrap_or(asked);
    let module = module.as_bytes();
    if last.contains(&b'.') {
        return last.eq_ignore_ascii_case(module);
    }
    module.len() == last.len() + 4
        && module[..last.len()].eq_ignore_ascii_case(last)
        && module[last.len()..].eq_ignore_ascii_case(b".DLL")
}

/// One file's process calls: its row, and its process.
pub struct Front<P> {
    file: &'static Row,
    process: P,
}

impl<P: Process> Front<P> {
    pub const fn new(file: &'static Row, process: P) -> Self {
        Self { file, process }
    }

    /// The process the calls ask.
    pub const fn process(&self) -> &P {
        &self.process
    }

    /// `GetCommandLineA`: the row's text, whole.
    #[must_use]
    pub fn command_line(&self) -> &'static str {
        let value = self.file.command_line;
        self.process.quiet(|| {
            log::debug!(c = self.file.calls.command_line, v = value; "imp.call");
        });
        value
    }

    /// `GetVersion`: the row's answer, every bit.
    #[must_use]
    pub fn version(&self) -> u32 {
        let value = self.file.version;
        self.process.quiet(|| {
            log::debug!(c = self.file.calls.version, v:% = mech_trace::__shown!(value); "imp.call");
        });
        value
    }

    /// `GetCurrentProcess`: the row's answer.
    #[must_use]
    pub fn current_process(&self) -> u32 {
        let value = self.file.current_process;
        self.process.quiet(|| {
            log::debug!(
                c = self.file.calls.current_process, v:% = mech_trace::__shown!(value); "imp.call"
            );
        });
        value
    }

    /// `GetModuleHandleA`: the file's own handle for no name, else the
    /// table's handle for a module it names; 0 and the row's error for a
    /// name it lacks.
    pub fn module_handle(&self, name: Option<&[u8]>) -> u32 {
        let value = match name {
            None => self.file.own,
            Some(name) => self.find(name).map_or(0, |module| module.handle),
        };
        let error = self.refused(value, self.file.errors.module_not_found);
        let asked = name.unwrap_or_default();
        self.process.quiet(|| {
            log::debug!(
                c = self.file.calls.module_handle, m:% = Text(asked),
                v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error); "imp.call"
            );
        });
        value
    }

    /// `LoadLibraryA`: the table's handle for a module it names; 0 and the
    /// row's error for one it lacks. Nothing is loaded: the table is the
    /// program's own.
    pub fn load_library(&self, name: &[u8]) -> u32 {
        let value = self.find(name).map_or(0, |module| module.handle);
        let error = self.refused(value, self.file.errors.module_not_found);
        self.process.quiet(|| {
            log::debug!(
                c = self.file.calls.load_library, m:% = Text(name),
                v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error); "imp.call"
            );
        });
        value
    }

    /// `GetProcAddress`: the back's number for an entry name the module's
    /// row holds; 0 and the row's error for a handle the table never gave
    /// or a name its row lacks, as Windows answers.
    pub fn proc_address(&self, handle: u32, name: &[u8]) -> u32 {
        let module = self.module(handle);
        let (value, missing) = match module {
            None => (0, self.file.errors.bad_handle),
            Some(module) => {
                let entry = module.entries.iter().find(|entry| entry.as_bytes() == name);
                let value = entry.map_or(0, |entry| self.process.entry(module, entry));
                (value, self.file.errors.name_not_found)
            }
        };
        let error = self.refused(value, missing);
        let arguments = [mech_trace::__shown!(handle)];
        self.process.quiet(|| {
            log::debug!(
                c = self.file.calls.proc_address, a:% = Args(arguments), q:% = Text(name),
                v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error); "imp.call"
            );
        });
        value
    }

    /// `FreeLibrary`: 1 for a handle the table gave, its row kept, as the
    /// program never loads one twice; 0 and the row's error for another.
    pub fn free_library(&self, handle: u32) -> i32 {
        let value = i32::from(self.module(handle).is_some());
        let error = self.refused(u32::from(value != 0), self.file.errors.bad_handle);
        let arguments = [mech_trace::__shown!(handle)];
        self.process.quiet(|| {
            log::debug!(
                c = self.file.calls.free_library, a:% = Args(arguments),
                v:% = mech_trace::__shown!(value), e:% = mech_trace::__shown!(error); "imp.call"
            );
        });
        value
    }

    /// `GetModuleFileNameA`: the row's path for the file's own handle (0
    /// or [`Row::own`]), cut to fit `into` with its NUL; the count written,
    /// the NUL left out. Another handle's path no reader reads: 0 and the
    /// row's error.
    pub fn module_file_name(&self, handle: u32, into: &mut [u8]) -> u32 {
        let (value, error) = if (handle == 0 || handle == self.file.own) && !into.is_empty() {
            let path = self.file.path.as_bytes();
            let count = path.len().min(into.len() - 1);
            into[..count].copy_from_slice(&path[..count]);
            into[count] = 0;
            (u32::try_from(count).unwrap_or(u32::MAX), 0)
        } else {
            self.process.fail(self.file.errors.bad_handle);
            (0, self.file.errors.bad_handle)
        };
        let size = into.len();
        let arguments = [mech_trace::__shown!(handle), mech_trace::__shown!(size)];
        let written = &into[..into.len().min(value as usize)];
        self.process.quiet(|| {
            log::debug!(
                c = self.file.calls.module_file_name, a:% = Args(arguments),
                m:% = Text(written), v:% = mech_trace::__shown!(value),
                e:% = mech_trace::__shown!(error); "imp.call"
            );
        });
        value
    }

    /// `GetLastError`: the back's.
    #[must_use]
    pub fn last_error(&self) -> u32 {
        let value = self.process.last_error();
        self.process.quiet(|| {
            log::debug!(c = self.file.calls.last_error, v:% = mech_trace::__shown!(value); "imp.call");
        });
        value
    }

    /// `SetLastError`: kept by the back.
    pub fn set_last_error(&self, code: u32) {
        self.process.fail(code);
        let arguments = [mech_trace::__shown!(code)];
        self.process.quiet(|| {
            log::debug!(c = self.file.calls.set_last_error, a:% = Args(arguments); "imp.call");
        });
    }

    /// `ExitProcess`: the line first, then the back ends the run.
    pub fn exit(&self, code: u32) {
        let arguments = [mech_trace::__shown!(code)];
        self.process.quiet(|| {
            log::debug!(c = self.file.calls.exit, a:% = Args(arguments); "imp.call");
        });
        self.process.end(code);
    }

    /// The table's row a module name names.
    fn find(&self, name: &[u8]) -> Option<&'static Module> {
        self.file
            .modules
            .iter()
            .find(|module| names(name, module.name))
    }

    /// The table's row a handle names.
    fn module(&self, handle: u32) -> Option<&'static Module> {
        self.file
            .modules
            .iter()
            .find(|module| handle != 0 && module.handle == handle)
    }

    /// Keeps `code` for a refused call, `value` 0; the error the line shows.
    fn refused(&self, value: u32, code: u32) -> u32 {
        if value == 0 {
            self.process.fail(code);
            code
        } else {
            0
        }
    }
}
