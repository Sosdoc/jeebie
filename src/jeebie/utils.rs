/// Returns true if the bit at index is 1, false otherwise 
#[inline(always)]
pub fn is_set(data: u8, index: usize) -> bool {    
    ((data >> index) & 1) == 1
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
    let registers = 0u8;
    is_set(registers, 8);
}