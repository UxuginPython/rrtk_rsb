// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
use core::mem::transmute;
pub const MAJOR: u8 = 0;
pub const MINOR: u8 = 1;
pub const PATCH: u8 = 0;
pub const PRE: u8 = 0;
pub mod tags {
    pub const SKIP_1: i8 = -128;
    pub const SKIP_2: i8 = -127;
    pub const SKIP_4: i8 = -126;
    pub const SKIP_8: i8 = -125;
    pub const SKIP_16: i8 = -124;
    pub const SKIP_U8: i8 = -123;
    pub const SKIP_U16: i8 = -122;
    pub const NODE_ID: i8 = 0;
    pub const COORDINATES: i8 = 3;
    pub const NODE_SECTION_START: i8 = 1;
    pub const NODE_SECTION_END: i8 = -1;
    pub const NODE_START: i8 = 2;
    pub const NODE_END: i8 = -2;
    pub const NODE_INPUT_LIST_START: i8 = 4;
    pub const NODE_INPUT_LIST_END: i8 = -4;
}
mod tags_u8 {
    use super::*;
    pub const SKIP_1: u8 = unsafe { transmute(tags::SKIP_1) };
    pub const SKIP_2: u8 = unsafe { transmute(tags::SKIP_2) };
    pub const SKIP_4: u8 = unsafe { transmute(tags::SKIP_4) };
    pub const SKIP_8: u8 = unsafe { transmute(tags::SKIP_8) };
    pub const SKIP_16: u8 = unsafe { transmute(tags::SKIP_16) };
    pub const SKIP_U8: u8 = unsafe { transmute(tags::SKIP_U8) };
    pub const SKIP_U16: u8 = unsafe { transmute(tags::SKIP_U16) };
    pub const NODE_ID: u8 = unsafe { transmute(tags::NODE_ID) };
    pub const COORDINATES: u8 = unsafe { transmute(tags::COORDINATES) };
    pub const NODE_SECTION_START: u8 = unsafe { transmute(tags::NODE_SECTION_START) };
    pub const NODE_SECTION_END: u8 = unsafe { transmute(tags::NODE_SECTION_END) };
    pub const NODE_START: u8 = unsafe { transmute(tags::NODE_START) };
    pub const NODE_END: u8 = unsafe { transmute(tags::NODE_END) };
    pub const NODE_INPUT_LIST_START: u8 = unsafe { transmute(tags::NODE_INPUT_LIST_START) };
    pub const NODE_INPUT_LIST_END: u8 = unsafe { transmute(tags::NODE_INPUT_LIST_END) };
}
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: u16,
    pub x: f64,
    pub y: f64,
    pub inputs: Vec<u16>,
}
fn hunt_tag(data: &[u8], tag: u8) -> Option<&[u8]> {
    let mut skip_next = 0u32;
    for i in 0..data.len() {
        if skip_next >= 1 {
            skip_next -= 1;
            continue;
        }
        let byte = data[i];
        let mut continuing = true;
        skip_next = match byte {
            tags_u8::SKIP_1 => 1,
            tags_u8::SKIP_2 => 2,
            tags_u8::SKIP_4 => 4,
            tags_u8::SKIP_8 => 8,
            tags_u8::SKIP_16 => 16,
            tags_u8::SKIP_U8 => {
                //We want to skip the next byte, the one telling us how far to skip, itself, as well as
                //adding a 'bias value' of 1 since skipping 0 bytes is worthless and it lets us skip up
                //to 256 instead of 255.
                data[i + 1] as u32 + 2
            }
            tags_u8::SKIP_U16 => {
                //Similarly to SKIP_U8, we skip the next two bytes (the ones telling us how far to
                //skip) and a bias of 1.
                (unsafe { transmute::<[u8; 2], u16>([data[i + 1], data[i + 2]]) }) as u32 + 3
            }
            _ => {
                continuing = false;
                0
            }
        };
        if continuing {
            continue;
        }
        //Everything above here is copied from hunt_tags.
        if byte == tag {
            return Some(&data[i..data.len()]);
        }
    }
    return None;
}
fn hunt_tags(data: &[u8], start: u8, end: u8) -> Vec<&[u8]> {
    let mut skip_next = 0u32;
    let mut inside = 0u8;
    let mut sections = Vec::<&[u8]>::new();
    let mut current_section_start: Option<usize> = None;
    for i in 0..data.len() {
        if skip_next >= 1 {
            skip_next -= 1;
            continue;
        }
        let byte = data[i];
        let mut continuing = true;
        skip_next = match byte {
            tags_u8::SKIP_1 => 1,
            tags_u8::SKIP_2 => 2,
            tags_u8::SKIP_4 => 4,
            tags_u8::SKIP_8 => 8,
            tags_u8::SKIP_16 => 16,
            tags_u8::SKIP_U8 => {
                //We want to skip the next byte, the one telling us how far to skip, itself, as well as
                //adding a 'bias value' of 1 since skipping 0 bytes is worthless and it lets us skip up
                //to 256 instead of 255.
                data[i + 1] as u32 + 2
            }
            tags_u8::SKIP_U16 => {
                //Similarly to SKIP_U8, we skip the next two bytes (the ones telling us how far to
                //skip) and a bias of 1.
                (unsafe { transmute::<[u8; 2], u16>([data[i + 1], data[i + 2]]) }) as u32 + 3
            }
            _ => {
                continuing = false;
                0
            }
        };
        if continuing {
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ErrorParseNode;
fn find_and_parse_node_id(data: &[u8]) -> Result<u16, ErrorParseNode> {
    let found = hunt_tag(data, tags_u8::NODE_ID);
    let found = match found {
        Some(x) => x,
        None => return Err(ErrorParseNode),
    };
    //TODO: make this follow the rule that you can use [SKIP_U8 2u8] and [SKIP_U16 2u16] too
    if found[1] != tags_u8::SKIP_2 {
        return Err(ErrorParseNode);
    }
    return Ok(unsafe { transmute([found[2], found[3]]) });
}
fn find_and_parse_coordinates(data: &[u8]) -> Result<(f64, f64), ErrorParseNode> {
    let found = hunt_tag(data, tags_u8::COORDINATES);
    let found = match found {
        Some(x) => x,
        None => return Err(ErrorParseNode),
    };
    //TODO: make this allow more options for how the numbers are escaped
    if found[1] != tags_u8::SKIP_16 {
        return Err(ErrorParseNode);
    }
    Ok((
        unsafe {
            transmute([
                found[2], found[3], found[4], found[5], found[6], found[7], found[8], found[9],
            ])
        },
        unsafe {
            transmute([
                found[10], found[11], found[12], found[13], found[14], found[15], found[16],
                found[17],
            ])
        },
    ))
}
//TODO: make this able to error in case a bad thing happens
fn find_and_parse_inputs(data: &[u8]) -> Vec<u16> {
    let input_section = hunt_tags(
        data,
        tags_u8::NODE_INPUT_LIST_START,
        tags_u8::NODE_INPUT_LIST_END,
    );
    if input_section.len() == 0 {
        return Vec::new();
    }
    if input_section.len() != 1 {
        panic!();
    }
    let input_section = input_section[0];
    let mut inputs = Vec::<u16>::new();
    for i in 0..(input_section.len() / 2) {
        inputs.push(unsafe { transmute([input_section[i * 2], input_section[i * 2 + 1]]) });
    }
    inputs
}
fn parse_node(data: &[u8]) -> Result<Node, ErrorParseNode> {
    let id = find_and_parse_node_id(data)?;
    let (x, y) = find_and_parse_coordinates(data)?;
    let inputs = find_and_parse_inputs(data);
    Ok(Node {
        id: id,
        x: x,
        y: y,
        inputs: inputs,
    })
}
fn parse_nodes(data: &[u8]) -> Result<Vec<Node>, ErrorParseNode> {
    let mut output = Vec::<Node>::new();
    for block in hunt_tags(data, tags_u8::NODE_START, tags_u8::NODE_END) {
        output.push(parse_node(block)?);
    }
    Ok(output)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorDecode {
    LayoutBroken,
    MagicNumbers,
    Version,
    MultipleNodeSections,
    NodeBroken,
}
pub fn read_file(data: &Vec<u8>) -> Result<Vec<Node>, ErrorDecode> {
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
    let node_sections = hunt_tags(
        &data[16..],
        tags_u8::NODE_SECTION_START,
        tags_u8::NODE_SECTION_END,
    );
    if node_sections.len() > 1 {
        return Err(ErrorDecode::MultipleNodeSections);
    }
    if node_sections.len() == 0 {
        return Ok(Vec::new());
    }
    let node_section = node_sections[0];
    match parse_nodes(node_section) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(ErrorDecode::NodeBroken),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hunt_tags_() {
        let data = vec![
            1u8,
            100,
            2,
            3,
            101,
            tags_u8::SKIP_1,
            100,
            tags_u8::SKIP_8,
            100,
            1,
            2,
            3,
            4,
            5,
            6,
            101,
            //SKIP_8 tag ends here
            4,
            100,
            5,
            6,
            7,
            101,
            8,
            tags_u8::SKIP_U8,
            2,
            100,
            4,
            101,
            //SKIP_U8 tag ends here
            100,
            8,
            9,
            101,
            5,
            tags_u8::SKIP_U16,
            //Just a really hacky way of inserting a u16 into an array of u8s.
            unsafe { transmute::<u16, [u8; 2]>(3u16)[0] },
            unsafe { transmute::<u16, [u8; 2]>(3u16)[1] },
            100,
            2,
            101,
            3,
            //SKIP_U16 tag ends here
            100,
            10,
            11,
            101,
            6,
        ];
        let found = hunt_tags(&data, 100, 101);
        assert_eq!(
            found,
            vec![&[2, 3][..], &[5, 6, 7][..], &[8, 9][..], &[10, 11][..]]
        );
    }
}
