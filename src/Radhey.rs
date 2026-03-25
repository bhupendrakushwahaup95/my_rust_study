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
    print!("Division using variables ({} % {} ) :  {}", a , b, remainder);


    print!("\n -----------------------------------------------------------");
    

}