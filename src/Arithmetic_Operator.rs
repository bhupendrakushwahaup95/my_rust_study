fn main(){
    // phase 1 : Calculation using veriables 

    // Declaring two i32 (integer) variables
    let a: i32 = 20;
    let b: i32 = 6;

    // 1. Addition
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
    
    //2. Subtraction
    let difference = a - b;
    println!("The difference between {} and {} is {}", a, b, difference);
    
    // 3. Multiplication
    let product = a * b ;
    println!("Multiplication using variables ({} * {} ): {} ", a , b, product);

    // 4. Division (Note: Provides quotient in integer division )
    let quotient = a / b;
    println!("Division using variables ({} / {} ): {} ", a , b , quotient);

    //5. Remainder (Modulo)
    let remainder = a % b;
    println!("Division using variables ({} % {} ) :  {}", a , b, remainder);


    println!("\n -----------------------------------------------------------");
    
    //--- Phase 2: Direct Calculation inside println! macro ----------

    // Direct Addition
    println!("Direct Addition (50 + 10: {} ", 50+10);

    // Direct Subtraction 
    println!("Direct Subtraction (100-45): {} ", 100-45);

    // Direct Multiplication
    println!("Direct Multiplication (12*5): {} ", 12*5);

    // Direct Division (Using floats to get decimal result)
    println!("Direct Division (25.0/4.0): {}", 25.0/4.0);

    // Direct Remainder
    println!("Direct Remainder (17%5): {}", 17%5);

    // Complex Expression : (10*2)+(20/4)-5
    println!("Combined Calculation (10*2+20/4-5) : {}", 10*2+20/4-5);
}