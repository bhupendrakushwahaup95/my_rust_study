fn main(){
    // --- Phase 1: Comparison using Variables -----
    let x: i32 = 15;
    let y: i32 = 10;

    // 1. Equal to (==)
    let is_equal = x == y;
    println!("Is {} equal to {}? : {} ", x, y, is_equal);

    // 2. Not Equal to (!=)
    let not_equal = x != y;
    println!("Is {} not equal to {}? : {}", x, y, not_equal);

    // 3. Greater than (>)
    let is_greater = x > y;
    println!("Is {} not equal to {}? : {}", x, y, is_greater);

    // 4. Less than (<)
    let is_less = x < y;
    println!("Is {} less than {}? : {}", x, y, is_less);
    
    // 5. Greater than or Equal to (>=)
    let greater_equal = x >= 15;
    println!("Is {} greater than or equal to 15? : {}", x, greater_equal);
    // 6. Less than or Equal to (<=)
    let less_equal = y <= 5;
    println!("Is {} less than or equal to 5? : {}", y, less_equal);

    println!("---------------------------------------------------------------");
    
}