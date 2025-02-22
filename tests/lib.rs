use rrtk_rsb::*;
#[test]
fn read_file_layout_broken() {
    let file = b"NotAFoodvvv";
    assert_eq!(read_file(file), Err(ErrorDecode::LayoutBroken));
}
#[test]
fn read_file_magic_number_error() {
    let file = b"ItIsFoodvvvv";
    assert_eq!(read_file(file), Err(ErrorDecode::MagicNumbers));
}
#[test]
fn read_file_version_error() {
    let file: [u8; 12] = unsafe { core::mem::transmute((*b"NotAFood", [u8::MAX; 4])) };
    assert_eq!(read_file(&file), Err(ErrorDecode::Version));
}
#[test]
fn read_file_correct() {
    let file: [u8; 12] = unsafe { core::mem::transmute((*b"NotAFood", [0u8, 1, 0, 0])) };
    read_file(&file).unwrap();
}
