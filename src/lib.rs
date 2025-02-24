// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
pub const MAJOR: u8 = 0;
pub const MINOR: u8 = 1;
pub const PATCH: u8 = 0;
pub const PRE: u8 = 0;
pub mod tags {
    pub const IGNORE_NEXT: u8 = unsafe { core::mem::transmute(i8::MIN) };
    pub const F64: u8 = unsafe { core::mem::transmute(i8::MIN + 1) };
    pub const NODES_START: u8 = unsafe { core::mem::transmute(1i8) };
    pub const NODES_END: u8 = unsafe { core::mem::transmute(-1i8) };
    pub const NODE_START: u8 = unsafe { core::mem::transmute(2i8) };
    pub const NODE_END: u8 = unsafe { core::mem::transmute(-2i8) };
}
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub inputs: Vec<u16>,
}
fn hunt_tags(data: &[u8], start: u8, end: u8) -> Vec<&[u8]> {
    let mut skip_next = 0u8;
    let mut inside = 0u8;
    let mut sections = Vec::<&[u8]>::new();
    let mut current_section_start: Option<usize> = None;
    for i in 0..data.len() {
        let byte = data[i];
        if skip_next >= 1 {
            skip_next -= 1;
            continue;
        }
        if byte == tags::IGNORE_NEXT {
            skip_next = 1;
            continue;
        }
        if byte == tags::F64 {
            skip_next = 8;
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
fn parse_node(data: &[u8]) -> Result<Node, ErrorParseNode> {
    if data.len() < 18 || data.len() % 2 != 0 || data[0] != tags::F64 || data[9] != tags::F64 {
        return Err(ErrorParseNode);
    }
    let x: [u8; 8] = [
        data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
    ];
    let x: f64 = unsafe { core::mem::transmute(x) };
    let y: [u8; 8] = [
        data[10], data[11], data[12], data[13], data[14], data[15], data[16], data[17],
    ];
    let y: f64 = unsafe { core::mem::transmute(y) };
    let mut inputs = Vec::<u16>::new();
    for i in 0..(data.len() - 18) / 2 {
        inputs.push(unsafe { *(core::ptr::addr_of!(inputs[i * 2 + 16]) as *const u16) });
    }
    Ok(Node {
        x: x,
        y: y,
        inputs: inputs,
    })
}
fn parse_nodes(data: &[u8]) -> Result<Vec<Node>, ErrorParseNode> {
    let mut output = Vec::<Node>::new();
    for block in hunt_tags(data, tags::NODE_START, tags::NODE_END) {
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
    let node_sections = hunt_tags(&data[16..], tags::NODES_START, tags::NODES_END);
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
            1,
            100,
            2,
            3,
            101,
            tags::IGNORE_NEXT,
            100,
            tags::F64,
            100,
            1,
            2,
            3,
            4,
            5,
            6,
            101,
            //F64 tag ends here
            4,
            100,
            5,
            6,
            7,
            101,
            8,
        ];
        let found = hunt_tags(&data, 100, 101);
        assert_eq!(found, vec![&[2, 3][..], &[5, 6, 7][..]]);
    }
}
