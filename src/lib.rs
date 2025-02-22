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
    if data.len() < 16 {
        return Err(ErrorDecode::LayoutBroken);
    }
    if data[0..12] != *b"rrtkstrmbldr" {
        return Err(ErrorDecode::MagicNumbers);
    }
    let major = data[12];
    if major > MAJOR {
        return Err(ErrorDecode::Version);
    }
    let minor = data[13];
    if minor > MINOR {
        return Err(ErrorDecode::Version);
    }
    let patch = data[14];
    if patch > PATCH {
        return Err(ErrorDecode::Version);
    }
    let pre = data[15];
    if pre > PRE {
        return Err(ErrorDecode::Version);
    }
    Ok(())
}
