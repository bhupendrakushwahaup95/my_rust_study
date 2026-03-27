fn main(){
    // -- Phase 1 : Logical Operations using Variables ----
    let is_student: bool = true;
    let has_passed: bool = true;
    let is_lazy: bool = false;

    // 1. Logical AND (&&) - Returns true only if both conditions are ture
    let can_graduate = is_student && has_passed;
    println!("Can graduate (Student AND Passed)? : {}", can_graduate);

    // 2. Logical OR (||) - Returns true if at least one condition is true
    let needs_help = is_lazy || !has_passed;
    println!("Needs help (Lazy OR Not Passed)? : {}", needs_help);

    // 3. Logical Not (!) - Reverses the boolean value (true becomes false)
    let is_active = !is_lazy;
    println!("Is active (NOT Lazy)? : {}", is_active);

    println!("-------------------------------------------------------------------");

    // --- Phase 2. Direct Logical Calculation inside println! Macro -------------
 
    // Direct AND Check (Both must be true)
    println!("Direct AND (10 > 5 && 5 < 8 ): {}", (10 < 5 ) && (5 < 8));

    // Direct OR Check (One must be true)
    println!("Direct OR (10 < 2 || 5 == 5): {}", (10 < 2) || (5 == 5));

    // Direct NOT Check (Inverts the result)
    println!("Direct NOT !(10 == 10): {}", !(10 == 10));

    // Comlex Logical Expression 
    // This combines Arithemtic, Comparison, and Logical operators
    // Process: (15 == 15) AND (10 > 5) OR (false) -> true AND true OR false --> true
    let result = (10 + 5 == 15 ) && (20 / 2 > 5) || (false);
    println!("Combined Logical Result: {}", result);
}