/// Returns true if the bit at index is 1, false otherwise.
/// Panics in case the index specified is greater than 7 (out of bounds of a byte)
pub fn is_set(data: u8, index: usize) -> bool {
    ((data >> index) & 1) == 1
}

/// Returns the byte with the bit at the specified index swapped.
/// Panics in case the index specified is greater than 7 (out of bounds of a byte)
pub fn swap_bit(data: u8, index: usize) -> u8 {
    data ^ (1 << index)
}

/// Returns the byte with the bit at the specified index set to 1.
/// Panics in case the index specified is greater than 7 (out of bounds of a byte)
pub fn set_bit(data: u8, index: usize) -> u8 {
    data | (1 << index)
}

/// Returns the byte with the bit at the specified index reset to 0.
/// Panics in case the index specified is greater than 7 (out of bounds of a byte)
pub fn reset_bit(data: u8, index: usize) -> u8 {
    data & ((1 << index) ^ 0xFF)
}

/// Returns a single 16 bit unsigned integer by chaining two 8 bit uints.
pub fn combine_as_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

#[test]
fn test_is_set() {
    let register = 0b1100_0010u8;

    assert!(is_set(register, 1));
    assert!(is_set(register, 6));
    assert!(is_set(register, 7));

    assert!(!is_set(register, 0));
}

#[test]
#[should_panic]
fn test_is_set_panics_overflow() {
    is_set(0, 8);
}

#[test]
fn test_swap() {
    let register = 0b1100_0010u8;

    assert_eq!(0b1100_0011u8, swap_bit(register, 0));
    assert_eq!(0b0100_0010u8, swap_bit(register, 7));
    assert_eq!(0b1000_0010u8, swap_bit(register, 6));
    assert_eq!(0b1100_1010u8, swap_bit(register, 3));
}

#[test]
#[should_panic]
fn test_swap_panics_overflow() {
    swap_bit(0, 8);
}

#[test]
fn test_set() {
    let register = 0b1100_0010u8;

    assert_eq!(0b1100_0011u8, set_bit(register, 0));
    assert_eq!(0b1100_0010u8, set_bit(register, 7));
    assert_eq!(0b1100_0010u8, set_bit(register, 6));
    assert_eq!(0b1100_1010u8, set_bit(register, 3));
}

#[test]
#[should_panic]
fn test_set_panics_overflow() {
    set_bit(0, 8);
}

#[test]
fn test_reset() {
    let register = 0b1100_0010u8;

    assert_eq!(0b1100_0010u8, reset_bit(register, 0));
    assert_eq!(0b0100_0010u8, reset_bit(register, 7));
    assert_eq!(0b1000_0010u8, reset_bit(register, 6));
    assert_eq!(0b1100_0010u8, reset_bit(register, 3));
}

#[test]
#[should_panic]
fn test_reset_panics_overflow() {
    reset_bit(0, 8);
}

#[test]
fn test_combine() {
    assert_eq!(0xFFAB, combine_as_u16(0xFF, 0xAB));
    assert_eq!(0x0, combine_as_u16(0x0, 0x0));
    assert_eq!(0x100, combine_as_u16(0x1, 0x0));
}
