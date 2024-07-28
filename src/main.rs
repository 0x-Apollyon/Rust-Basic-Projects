use std::io;


fn floating_point_division(number1: i32, number2: i32) -> (u128, u128) {
    let number1 = (number1 as u128) * 1000;
    let number2 = number2 as u128;
    let div = number1 / number2;
    
    let floating = div % 1000;
    let intreg = div / 1000;
    
    (intreg , floating)
}

fn main() {
    println!("Calculator \n");
    println!("1:Addition ");
    println!("2:Subtraction ");
    println!("3:Multiplication ");
    println!("4:Division ");
    println!("5:Exponentiation\n ");

    println!("Select operation: ");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Error while reading operation name");

    operation = operation.trim().to_lowercase();

    let array_addition = vec!["addition", "add" , "sum" , "1" , "one"];
    let array_subtraction = vec!["subtraction", "subtract" , "difference" , "2" , "two"];
    let array_multiplication = vec!["multiplication", "multiply" , "product" , "3" , "three"];
    let array_division = vec!["division", "divide" , "4" , "four"];
    let array_exponenet = vec!["exponent", "exponential" , "power" , "5" , "five"];

    if array_addition.iter().any(|e| operation.contains(e))

    {
        let mut number_1 = String::new();
        let mut number_2 = String::new();

        println!("Number 1 to add: ");
        io::stdin().read_line(&mut number_1).expect("Error while reading first number");
        let number1: i32 = number_1.trim().parse().expect("Input not an integer");
        drop(number_1);


        println!("Number 2 to add: ");
        io::stdin().read_line(&mut number_2).expect("Error while reading second number");
        let number2: i32 = number_2.trim().parse().expect("Input not an integer");
        drop(number_2);

        println!("The sum of {} and {} is {}" , number1 , number2 , number1+number2);

    }

    else if array_subtraction.iter().any(|e| operation.contains(e))

    {
        let mut number_1 = String::new();
        let mut number_2 = String::new();

        println!("Number 1 (To be subtracted from): ");
        io::stdin().read_line(&mut number_1).expect("Error while reading first number");
        let number1: i32 = number_1.trim().parse().expect("Input not an integer");
        drop(number_1);


        println!("Number 2 (To subtract): ");
        io::stdin().read_line(&mut number_2).expect("Error while reading second number");
        let number2: i32 = number_2.trim().parse().expect("Input not an integer");
        drop(number_2);

        println!("The difference of {} and {} is {}" , number1 , number2 , number1-number2);

    }

    else if array_multiplication.iter().any(|e| operation.contains(e))

    {
        let mut number_1 = String::new();
        let mut number_2 = String::new();

        println!("Number 1 to multiply: ");
        io::stdin().read_line(&mut number_1).expect("Error while reading first number");
        let number1: i32 = number_1.trim().parse().expect("Input not an integer");
        drop(number_1);


        println!("Number 2 to multiply: ");
        io::stdin().read_line(&mut number_2).expect("Error while reading second number");
        let number2: i32 = number_2.trim().parse().expect("Input not an integer");
        drop(number_2);

        println!("The product of {} and {} is {}" , number1 , number2 , number1*number2);
   
    }

    else if array_division.iter().any(|e| operation.contains(e))

    {
        let mut number_1 = String::new();
        let mut number_2 = String::new();

        println!("Number 1 (Dividend): ");
        io::stdin().read_line(&mut number_1).expect("Error while reading first number");
        let number1: i32 = number_1.trim().parse().expect("Input not an integer");
        drop(number_1);


        println!("Number 2 (Divisor): ");
        io::stdin().read_line(&mut number_2).expect("Error while reading second number");
        let number2: i32 = number_2.trim().parse().expect("Input not an integer");
        drop(number_2);

        let decimal_output = floating_point_division(number1 , number2);

        println!("The quotient is {} and the remainder is {}. The decimal expression is {}.{}" , number1/number2 , number1%number2 , decimal_output.0 , decimal_output.1);

    }

    else if array_exponenet.iter().any(|e| operation.contains(e))

    {
        let mut number_1 = String::new();
        let mut number_2 = String::new();

        println!("Number 1 (Base): ");
        io::stdin().read_line(&mut number_1).expect("Error while reading first number");
        let number1: i32 = number_1.trim().parse().expect("Input not an integer");
        drop(number_1);


        println!("Number 2 (Exponent): ");
        io::stdin().read_line(&mut number_2).expect("Error while reading second number");
        let number2: i32 = number_2.trim().parse().expect("Input not an integer");
        drop(number_2);

        let powered = (number1 as u32).pow(number2 as u32);

        println!("{} to the power {} is {:?}" , number1 , number2 , powered);
    }
}
