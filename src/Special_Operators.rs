fn main(){
    // --- Phase 1: Special Operations using Variables ---

    // 1. Type Casting Operator (as)
    // Used to convert one data type to another
    let integer_val: i32 = 65;
    let float_val = integer_val as f64 + 0.5; // Converting i32 to f64
    let char_val = integer_val as u8 as char; // Converting i32 to char (ASCII)
    println!("Type Casting (as): {} -> {} and char: {}", integer_val, float_val, char_val);

    // 2. Range Operators (.. and ..=)
    // exclusive range: 1..5 (1,2,3,4)
    // inclusive range: 1..=5 (1,2,3,4,5)
    println!("Exclusive Range (1..5) is used in loops (discussed in statements)");

    // 3. Reference and Dereference Operators (& and *)
    let actual_value = 100;
    let reference = &actual_value; // & creates a reference (address)
    println!("Reference Address (&): {:p}", reference);

    let deref_value = *reference; // * accesses the value at that address
    println!("Dereferenced Value (*): {}", deref_value);

    println!("--------------------------------------------------------------------------");

    

}