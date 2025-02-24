use rrtk_rsb::*;
#[test]
fn read_file_layout_broken() {
    let file = b"rrtkstrmbldrvvv";
    assert_eq!(read_file(&file.into()), Err(ErrorDecode::LayoutBroken));
}
#[test]
fn read_file_magic_number_error() {
    let file = b"badmagicnumsvvvv";
    assert_eq!(read_file(&file.into()), Err(ErrorDecode::MagicNumbers));
}
#[test]
fn read_file_version_error() {
    let file: [u8; 16] = unsafe { core::mem::transmute((*b"rrtkstrmbldr", [u8::MAX; 4])) };
    assert_eq!(read_file(&file.into()), Err(ErrorDecode::Version));
}
#[test]
fn read_file_no_node_section() {
    let file: [u8; 16] = unsafe { core::mem::transmute((*b"rrtkstrmbldr", [0u8, 1, 0, 0])) };
    assert_eq!(read_file(&file.into()).unwrap(), vec![]);
}
#[test]
fn read_file_two_empty_node_sections() {
    let file: [u8; 20] = unsafe {
        core::mem::transmute((
            *b"rrtkstrmbldr",
            [0u8, 1, 0, 0],
            tags::NODES_START,
            tags::NODES_END,
            tags::NODES_START,
            tags::NODES_END,
        ))
    };
    assert_eq!(read_file(&file.into()).unwrap(), vec![]);
}
#[test]
fn read_file_empty() {
    let file: [u8; 18] = unsafe {
        core::mem::transmute((
            *b"rrtkstrmbldr",
            [0u8, 1, 0, 0],
            tags::NODES_START,
            tags::NODES_END,
        ))
    };
    assert_eq!(read_file(&file.into()).unwrap(), vec![]);
}
#[test]
fn read_file_one_node() {
    #[allow(unused)]
    #[repr(packed)]
    struct TestFile([u8; 12], [u8; 4], u8, u8, u8, f64, u8, f64, u8, u8);
    let file: [u8; 38] = unsafe {
        core::mem::transmute(TestFile(
            *b"rrtkstrmbldr",
            [0u8, 1, 0, 0],
            tags::NODES_START,
            tags::NODE_START,
            tags::F64,
            0.0f64,
            tags::F64,
            0.0f64,
            tags::NODE_END,
            tags::NODES_END,
        ))
    };
    assert_eq!(
        read_file(&file.into()).unwrap(),
        vec![Node {
            x: 0.0,
            y: 0.0,
            inputs: vec![],
        }]
    );
}
