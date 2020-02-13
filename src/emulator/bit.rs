pub fn split_word(word: u16) -> (u8, u8) {
    ((word >> 8) as u8, (word & 0xFF) as u8)
}

pub fn combine_bytes(high: u8, low: u8) -> u16 {
    (high as u16) << 8 | low as u16
}

#[test]
fn split_word_test() {
    let (high, low) = split_word(0x1234);
    assert_eq!(high, 0x12);
    assert_eq!(low, 0x34);
}

#[test]
fn combine_bytes_test() {
    let (high, low) = (0x34, 0x12);
    let result = combine_bytes(high, low);
    assert_eq!(result, 0x3412);
}