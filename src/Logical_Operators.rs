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

    println!("-------------------------------------------------------------------")
}