fn main(){
        // --- Phase 1: Assignment using Variables ---

    // Note: 'mut' is used because we are changing the value
    let mut x: i32 = 10;
    let y: i32 = 5;
    println!("Value of x is : {}", x);
    println!("Value of y is : {}", y);

    // 1. Simple Assignment (=)
    x = 20;
    println!("Simple Assignment (x = 20 ): {}", x);

    // 2. Addition Assignment (x += ) -> x = x + y
    x += y;
    println!("Addition Assignment (x += {} ): {}", y, x);

    // 3. Subtraction Assignment (-=) -> x = x-y
    x -= 2;
    println!("Subtraction Assignment (x -= 2): {}", x);

    // 4. Multiplication Assignment (*=) -> x = x * Y
    x *= 3;
    println!("Multiplication Assignment (x *= 3): {}", x);

    // 5. Division Assignment (/=) -> x = x / y
    x /= 5;
    println!("Division Assignment (x /= 5) : {}", x);

    // 6. Remainder Assignment (%=) -> x = x % y
    x %= 4;
    println!("Remainder Assignment (x %= 4): {}", x);

    println!("--------------------------------------------------------------------");

    // ---- Phase 2: Direct Assignment & Calculation inside println! ---------

    // Note: Rust does not allow direct assignment inside println! like some other languages.
    // However, we can perform and print the updated value of a mutable variable.

    let mut score = 100;
    println!("Value of score is : {}",score);
    score += 50; // Updating score
    println!("Updated Score after += 50 : {}", score);

    score *= 2; // Doubling the score
    println!("Updated Score after *= 2: {}", score);

    // Compound Assignment with Logic 
    let mut status = true;
    status &= false; // Bitwise/Logical AND assignment
    println!("Status after &= false: {}", status);
    
}