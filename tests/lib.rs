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
fn read_file_correct() {
    let file: [u8; 16] = unsafe { core::mem::transmute((*b"rrtkstrmbldr", [0u8, 1, 0, 0])) };
    read_file(&file.into()).unwrap();
}
