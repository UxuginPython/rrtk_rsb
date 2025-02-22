// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
const MAJOR: u8 = 0;
const MINOR: u8 = 1;
const PATCH: u8 = 0;
const PRE: u8 = 0;
pub enum ErrorDecode {
    MagicNumbers,
    Version,
}
pub fn read_file<const N: usize>(data: &[u8; N]) -> Result<(), ErrorDecode> {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}
