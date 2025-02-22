// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
pub const MAJOR: u8 = 0;
pub const MINOR: u8 = 1;
pub const PATCH: u8 = 0;
pub const PRE: u8 = 0;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorDecode {
    LayoutBroken,
    MagicNumbers,
    Version,
}
pub fn read_file(data: Vec<u8>) -> Result<(), ErrorDecode> {
    if data.len() < 12 {
        return Err(ErrorDecode::LayoutBroken);
    }
    if data[0..8] != *b"NotAFood" {
        return Err(ErrorDecode::MagicNumbers);
    }
    let major = data[8];
    if major > MAJOR {
        return Err(ErrorDecode::Version);
    }
    let minor = data[9];
    if minor > MINOR {
        return Err(ErrorDecode::Version);
    }
    let patch = data[10];
    if patch > PATCH {
        return Err(ErrorDecode::Version);
    }
    let pre = data[11];
    if pre > PRE {
        return Err(ErrorDecode::Version);
    }
    Ok(())
}
