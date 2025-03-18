// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
use core::mem::transmute;
pub const MAJOR: u8 = 1;
pub const MINOR: u8 = 0;
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
fn bytes_to_u16(it: &[u8]) -> u16 {
    assert_eq!(it.len(), 2);
    unsafe { transmute([it[0], it[1]]) }
}
fn u16_to_bytes(it: u16) -> [u8; 2] {
    unsafe { transmute(it) }
}
fn bytes_to_f64(it: &[u8]) -> f64 {
    assert_eq!(it.len(), 8);
    unsafe { transmute([it[0], it[1], it[2], it[3], it[4], it[5], it[6], it[7]]) }
}
fn f64_to_bytes(it: f64) -> [u8; 8] {
    unsafe { transmute(it) }
}
mod categorizer {
    use super::*;
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum BigSkipState {
        NotBigSkip,
        NeedU8,
        NeedU16Byte0,
        NeedU16Byte1(u8), //byte 0 is stored
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CategorizedByte {
        Tag(u8),
        Number(u8), //This can be used for parts of numbers as well, such as 1 byte from a u16.
        Skip,
    }
    #[derive(Clone, Debug)]
    pub struct Categorizer {
        skip_next: u32,
        big_skip_state: BigSkipState,
    }
    impl Categorizer {
        pub fn new() -> Self {
            Self {
                skip_next: 0,
                big_skip_state: BigSkipState::NotBigSkip,
            }
        }
        pub fn feed(&mut self, byte: u8) -> CategorizedByte {
            if self.skip_next >= 1 {
                self.skip_next -= 1;
                return CategorizedByte::Number(byte);
            }
            match self.big_skip_state {
                BigSkipState::NotBigSkip => {}
                BigSkipState::NeedU8 => {
                    self.skip_next = byte as u32 + 1;
                    self.big_skip_state = BigSkipState::NotBigSkip;
                    return CategorizedByte::Skip;
                }
                BigSkipState::NeedU16Byte0 => {
                    self.big_skip_state = BigSkipState::NeedU16Byte1(byte);
                    return CategorizedByte::Skip;
                }
                BigSkipState::NeedU16Byte1(byte0) => {
                    self.skip_next = bytes_to_u16(&[byte0, byte]) as u32 + 1;
                    self.big_skip_state = BigSkipState::NotBigSkip;
                    return CategorizedByte::Skip;
                }
            }
            match byte {
                tags_u8::SKIP_1 => {
                    self.skip_next = 1;
                    return CategorizedByte::Skip;
                }
                tags_u8::SKIP_2 => {
                    self.skip_next = 2;
                    return CategorizedByte::Skip;
                }
                tags_u8::SKIP_4 => {
                    self.skip_next = 4;
                    return CategorizedByte::Skip;
                }
                tags_u8::SKIP_8 => {
                    self.skip_next = 8;
                    return CategorizedByte::Skip;
                }
                tags_u8::SKIP_16 => {
                    self.skip_next = 16;
                    return CategorizedByte::Skip;
                }
                tags_u8::SKIP_U8 => {
                    self.big_skip_state = BigSkipState::NeedU8;
                    return CategorizedByte::Skip;
                }
                tags_u8::SKIP_U16 => {
                    self.big_skip_state = BigSkipState::NeedU16Byte0;
                    return CategorizedByte::Skip;
                }
                _ => {}
            }
            CategorizedByte::Tag(byte)
        }
    }
}
use categorizer::*;
fn hunt_tag(data: &[u8], tag: u8) -> Option<&[u8]> {
    let mut categorizer = Categorizer::new();
    for i in 0..data.len() {
        let byte = categorizer.feed(data[i]);
        if byte == CategorizedByte::Tag(tag) {
            return Some(&data[i..data.len()]);
        }
    }
    return None;
}
fn hunt_tags(data: &[u8], start: u8, end: u8) -> Vec<&[u8]> {
    let mut inside = 0u8;
    let mut sections = Vec::<&[u8]>::new();
    let mut current_section_start: Option<usize> = None;
    let mut categorizer = Categorizer::new();
    for i in 0..data.len() {
        let byte = categorizer.feed(data[i]);
        if byte == CategorizedByte::Tag(start) {
            if inside == 0 {
                current_section_start = Some(i + 1);
            }
            inside += 1;
        }
        if byte == CategorizedByte::Tag(end) && inside > 0 {
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
fn hunt_numbers(data: &[u8], max_count: Option<usize>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut categorizer = Categorizer::new();
    for i in 0..data.len() {
        let byte = categorizer.feed(data[i]);
        match byte {
            CategorizedByte::Number(x) => output.push(x),
            _ => {}
        }
        if let Some(some_max_count) = max_count {
            if output.len() >= some_max_count {
                return output;
            }
        }
    }
    output
}
pub mod error {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ParseFile {
        LayoutBroken,
        MagicNumbers,
        Version,
        MultipleNodeSections,
        ParseNode(parse_file::ParseNode),
    }
    impl From<parse_file::ParseNode> for ParseFile {
        fn from(was: parse_file::ParseNode) -> Self {
            Self::ParseNode(was)
        }
    }
    pub mod parse_file {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum ParseNode {
            ParseNodeID(parse_node::ParseNodeID),
            ParseCoordinates(parse_node::ParseCoordinates),
            ParseInputs(parse_node::ParseInputs),
        }
        impl From<parse_node::ParseNodeID> for ParseNode {
            fn from(was: parse_node::ParseNodeID) -> Self {
                Self::ParseNodeID(was)
            }
        }
        impl From<parse_node::ParseCoordinates> for ParseNode {
            fn from(was: parse_node::ParseCoordinates) -> Self {
                Self::ParseCoordinates(was)
            }
        }
        impl From<parse_node::ParseInputs> for ParseNode {
            fn from(was: parse_node::ParseInputs) -> Self {
                Self::ParseInputs(was)
            }
        }
        pub mod parse_node {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum ParseNodeID {
                NotFound,
                IncorrectLength,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum ParseCoordinates {
                NotFound,
                IncorrectLength,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum ParseInputs {
                MultipleInputSections,
                LayoutBroken,
            }
        }
    }
}
fn find_and_parse_node_id(data: &[u8]) -> Result<u16, error::parse_file::parse_node::ParseNodeID> {
    let found = hunt_tag(data, tags_u8::NODE_ID);
    let found = match found {
        Some(x) => x,
        None => return Err(error::parse_file::parse_node::ParseNodeID::NotFound),
    };
    let found_numbers = hunt_numbers(found, Some(2));
    if found_numbers.len() != 2 {
        return Err(error::parse_file::parse_node::ParseNodeID::IncorrectLength);
    }
    Ok(unsafe { transmute([found_numbers[0], found_numbers[1]]) })
}
fn find_and_parse_coordinates(
    data: &[u8],
) -> Result<(f64, f64), error::parse_file::parse_node::ParseCoordinates> {
    let found = hunt_tag(data, tags_u8::COORDINATES);
    let found = match found {
        Some(x) => x,
        None => return Err(error::parse_file::parse_node::ParseCoordinates::NotFound),
    };
    let found_numbers = hunt_numbers(found, Some(16));
    if found_numbers.len() != 16 {
        return Err(error::parse_file::parse_node::ParseCoordinates::IncorrectLength);
    }
    Ok((
        bytes_to_f64(&found_numbers[0..=7]),
        bytes_to_f64(&found_numbers[8..=15]),
    ))
}
fn find_and_parse_inputs(
    data: &[u8],
) -> Result<Vec<u16>, error::parse_file::parse_node::ParseInputs> {
    let input_section = hunt_tags(
        data,
        tags_u8::NODE_INPUT_LIST_START,
        tags_u8::NODE_INPUT_LIST_END,
    );
    if input_section.len() == 0 {
        return Ok(Vec::new());
    }
    if input_section.len() != 1 {
        return Err(error::parse_file::parse_node::ParseInputs::MultipleInputSections);
    }
    let input_section = input_section[0];
    let found_numbers = hunt_numbers(input_section, None);
    if found_numbers.len() % 2 != 0 {
        return Err(error::parse_file::parse_node::ParseInputs::LayoutBroken);
    }
    let mut inputs = Vec::<u16>::new();
    for i in 0..(found_numbers.len() / 2) {
        inputs.push(bytes_to_u16(&[
            found_numbers[i * 2],
            found_numbers[i * 2 + 1],
        ]));
    }
    Ok(inputs)
}
fn parse_node(data: &[u8]) -> Result<Node, error::parse_file::ParseNode> {
    let id = find_and_parse_node_id(data)?;
    let (x, y) = find_and_parse_coordinates(data)?;
    let inputs = find_and_parse_inputs(data)?;
    Ok(Node {
        id: id,
        x: x,
        y: y,
        inputs: inputs,
    })
}
fn parse_nodes(data: &[u8]) -> Result<Vec<Node>, error::parse_file::ParseNode> {
    let mut output = Vec::<Node>::new();
    for block in hunt_tags(data, tags_u8::NODE_START, tags_u8::NODE_END) {
        output.push(parse_node(block)?);
    }
    Ok(output)
}
pub fn read_file(data: &Vec<u8>) -> Result<Vec<Node>, error::ParseFile> {
    if data.len() < 16 {
        return Err(error::ParseFile::LayoutBroken);
    }
    if data[0..12] != *b"rrtkstrmbldr" {
        return Err(error::ParseFile::MagicNumbers);
    }
    let major = data[12];
    if major > MAJOR {
        return Err(error::ParseFile::Version);
    }
    let minor = data[13];
    if minor > MINOR {
        return Err(error::ParseFile::Version);
    }
    let patch = data[14];
    if patch > PATCH {
        return Err(error::ParseFile::Version);
    }
    let pre = data[15];
    if pre > PRE {
        return Err(error::ParseFile::Version);
    }
    let node_sections = hunt_tags(
        &data[16..],
        tags_u8::NODE_SECTION_START,
        tags_u8::NODE_SECTION_END,
    );
    if node_sections.len() > 1 {
        return Err(error::ParseFile::MultipleNodeSections);
    }
    if node_sections.len() == 0 {
        return Ok(Vec::new());
    }
    let node_section = node_sections[0];
    match parse_nodes(node_section) {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(error::ParseFile::ParseNode(error)),
    }
}
mod file_start {
    use super::*;
    #[allow(unused)]
    #[repr(packed)]
    struct FileStart([u8; 12], u8, u8, u8, u8);
    pub const FILE_START: [u8; 16] =
        unsafe { transmute(FileStart(*b"rrtkstrmbldr", MAJOR, MINOR, PATCH, PRE)) };
}
pub fn build_file(nodes: &Vec<Node>) -> Vec<u8> {
    //18 bytes for the magic numbers, version, and NODE_SECTION tags;
    //26 bytes for each node assuming no inputs
    //It may make sense to assume some number of inputs for each node, but the current way ensures that no
    //more memory will ever be allocated than necessary. It would also be possible to count each node's
    //inputs and calculate the correct size, but that would be probably more compute time than it's
    //worth.
    let mut output = Vec::with_capacity(18 + 26 * nodes.len());
    output.extend(FILE_START);
    output.push(tags_u8::NODE_SECTION_START);
    for node in nodes {
        output.push(tags_u8::NODE_START);
        output.push(tags_u8::NODE_ID);
        output.push(tags_u8::SKIP_2);
        output.extend(u16_to_bytes(node.id));
        output.push(tags_u8::COORDINATES);
        output.push(tags_u8::SKIP_16);
        output.extend(f64_to_bytes(node.x));
        output.extend(f64_to_bytes(node.y));
        output.push(tags_u8::NODE_INPUT_LIST_START);
        for input in &node.inputs {
            output.push(tags_u8::SKIP_2);
            output.extend(u16_to_bytes(*input));
        }
        output.push(tags_u8::NODE_INPUT_LIST_END);
        output.push(tags_u8::NODE_END);
    }
    output.push(tags_u8::NODE_SECTION_END);
    output
}
pub use file_start::FILE_START;
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
