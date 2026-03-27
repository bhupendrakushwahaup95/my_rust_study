fn main(){
    // --- Phase 1: Bitwise Operations using Variables ---

    // Using u8 (8-bit unsigned integer) for clear bit representation
    // 5 in binary is 00000101
    // 3 in binary is 00000011
    let a: u8 = 5;
    let b: u8 =3;

    // 1. Bitwise AND (&)
    // Comparison: 0101 & 0011 = 0001 (Decimal 1)
    let and_res = a & b;
    println!("Bitwise AND ({} & {}): {}", a, b, and_res);

    // 2. Bitwise OR (|)
    // Comparison: 0101 | 0011 = 0111 (Decimal 7)
    let or_res = a | b;
    println!("Bitwise OR ({} | {}): {}", a, b, or_res);

    // 3. Bitwise XOR (^)
    // Comparison: 0101 ^ 0011 = 0110 (Decimal 6)
    let xor_res = a ^ b;
    println!("Bitwise XOR ({} ^ {}): {}", a, b, xor_res);

    // 4. Bitwise NOT (!)
    // Inverts all bits: !00000101 = 11111010 (Decimal 250 for u8)
    let not_res = !a;
    println!("Bitwise NOT (!{}): {}", a, not_res);

    // 5. Left Shift (<<)
    // Shifts bits to left: 00000101 << 1 = 0001010 (Decimal 10)
    let left_shift = a << 1;
    println!("Left Shift ({} << 1 ): {}", a, left_shift);

    // 6. Right Shift (>>)
    // Shifts bits to right: 00000101 >> 1 = 00000010 (Decimal 2)
    let right_shift = a >> 1;
    println!("Right Shift ({} >> 1): {}", a, right_shift);

    println!("-----------------------------------------------------------------")
}