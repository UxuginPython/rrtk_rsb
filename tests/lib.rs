// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
use rrtk_rsb::*;
#[test]
fn read_file_layout_broken() {
    let file = b"rrtkstrmbldrvvv";
    assert_eq!(read_file(&file.into()), Err(error::ParseFile::LayoutBroken));
}
#[test]
fn read_file_magic_number_error() {
    let file = b"badmagicnumsvvvv";
    assert_eq!(read_file(&file.into()), Err(error::ParseFile::MagicNumbers));
}
#[test]
fn read_file_version_error() {
    let file: [u8; 16] = unsafe { core::mem::transmute((*b"rrtkstrmbldr", [u8::MAX; 4])) };
    assert_eq!(read_file(&file.into()), Err(error::ParseFile::Version));
}
#[test]
fn read_file_no_node_section() {
    let file: [u8; 16] = unsafe { core::mem::transmute((*b"rrtkstrmbldr", [1u8, 0, 0, 0])) };
    assert_eq!(read_file(&file.into()).unwrap(), vec![]);
}
#[test]
fn read_file_two_empty_node_sections() {
    #[allow(unused)]
    #[repr(packed)]
    struct TestFile([u8; 12], [u8; 4], i8, i8, i8, i8);
    let file: [u8; 20] = unsafe {
        core::mem::transmute(TestFile(
            *b"rrtkstrmbldr",
            [1u8, 0, 0, 0],
            tags::NODE_SECTION_START,
            tags::NODE_SECTION_END,
            tags::NODE_SECTION_START,
            tags::NODE_SECTION_END,
        ))
    };
    assert_eq!(read_file(&file.into()).unwrap(), vec![]);
}
#[test]
fn read_file_empty() {
    #[allow(unused)]
    #[repr(packed)]
    struct TestFile([u8; 12], [u8; 4], i8, i8);
    let file: [u8; 18] = unsafe {
        core::mem::transmute((
            *b"rrtkstrmbldr",
            [1u8, 0, 0, 0],
            tags::NODE_SECTION_START,
            tags::NODE_SECTION_END,
        ))
    };
    assert_eq!(read_file(&file.into()).unwrap(), vec![]);
}
#[test]
fn read_file_one_node() {
    #[allow(unused)]
    #[repr(packed)]
    struct TestFile(
        [u8; 12],
        [u8; 4],
        i8,
        i8,
        i8,
        i8,
        u16,
        i8,
        i8,
        f64,
        f64,
        i8,
        i8,
        i8,
        i8,
    );
    let file: [u8; 44] = unsafe {
        core::mem::transmute(TestFile(
            *b"rrtkstrmbldr",
            [1u8, 0, 0, 0],
            tags::NODE_SECTION_START,
            tags::NODE_START,
            tags::NODE_ID,
            tags::SKIP_2,
            4u16,
            tags::COORDINATES,
            tags::SKIP_16,
            0.0f64,
            0.0f64,
            tags::NODE_INPUT_LIST_START,
            tags::NODE_INPUT_LIST_END,
            tags::NODE_END,
            tags::NODE_SECTION_END,
        ))
    };
    assert_eq!(
        read_file(&file.into()).unwrap(),
        vec![Node {
            id: 4,
            x: 0.0,
            y: 0.0,
            inputs: vec![],
        }]
    );
}
#[test]
fn read_file_two_nodes() {
    #[allow(unused)]
    #[repr(packed)]
    struct TestFile(
        [u8; 12],
        [u8; 4],
        i8,
        i8,
        i8,
        i8,
        u16,
        i8,
        i8,
        f64,
        f64,
        i8,
        i8,
        i8,
        i8,
        i8,
        i8,
        u16,
        i8,
        i8,
        f64,
        f64,
        i8,
        i8,
        u8,
        u16,
        i8,
        i8,
        i8,
    );
    let file: [u8; 74] = unsafe {
        core::mem::transmute(TestFile(
            *b"rrtkstrmbldr",
            [1u8, 0, 0, 0],
            tags::NODE_SECTION_START,
            tags::NODE_START,
            tags::NODE_ID,
            tags::SKIP_2,
            5u16,
            tags::COORDINATES,
            tags::SKIP_16,
            0.0f64,
            0.0f64,
            tags::NODE_INPUT_LIST_START,
            tags::NODE_INPUT_LIST_END,
            tags::NODE_END,
            tags::NODE_START,
            tags::NODE_ID,
            tags::SKIP_2,
            4u16,
            tags::COORDINATES,
            tags::SKIP_16,
            0.0f64,
            0.0f64,
            tags::NODE_INPUT_LIST_START,
            tags::SKIP_U8,
            1u8,
            0u16,
            tags::NODE_INPUT_LIST_END,
            tags::NODE_END,
            tags::NODE_SECTION_END,
        ))
    };
    assert_eq!(
        read_file(&file.into()).unwrap(),
        vec![
            Node {
                id: 5,
                x: 0.0,
                y: 0.0,
                inputs: vec![],
            },
            Node {
                id: 4,
                x: 0.0,
                y: 0.0,
                inputs: vec![0],
            }
        ]
    );
}
