// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
pub const MAJOR: u8 = 0;
pub const MINOR: u8 = 1;
pub const PATCH: u8 = 0;
pub const PRE: u8 = 0;
pub mod tags {
    pub const IGNORE_NEXT: u8 = unsafe { core::mem::transmute(i8::MIN) };
    pub const NODES_START: u8 = unsafe { core::mem::transmute(1i8) };
    pub const NODES_END: u8 = unsafe { core::mem::transmute(-1i8) };
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorDecode {
    LayoutBroken,
    MagicNumbers,
    Version,
}
fn hunt_tags(data: &[u8], start: u8, end: u8) -> Vec<&[u8]> {
    let mut skip_next = false;
    let mut inside = 0u8;
    let mut sections = Vec::<&[u8]>::new();
    let mut current_section_start: Option<usize> = None;
    for i in 0..data.len() {
        let byte = data[i];
        if skip_next {
            skip_next = false;
            continue;
        }
        if byte == tags::IGNORE_NEXT {
            skip_next = true;
            continue;
        }
        if byte == start {
            if inside == 0 {
                current_section_start = Some(i + 1);
            }
            inside += 1;
        }
        if byte == end && inside > 0 {
            inside -= 1;
            if inside == 0 {
                let real_current_section_start = current_section_start.unwrap();
                if real_current_section_start < i {
                    sections.push(&data[real_current_section_start..i]);
                }
                current_section_start = None;
            }
        }
    }
    sections
}
pub fn read_file(data: &Vec<u8>) -> Result<(), ErrorDecode> {
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
    hunt_tags(&data[16..], tags::NODES_START, tags::NODES_END);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hunt_tags_() {
        let data = vec![1, 100, 2, 3, 101, 4, 100, 5, 6, 7, 101, 8];
        let found = hunt_tags(&data, 100, 101);
        assert_eq!(found, vec![&[2, 3][..], &[5, 6, 7][..]]);
    }
}
