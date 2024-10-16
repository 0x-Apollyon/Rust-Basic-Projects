fn main() {
    let fib_numbers:u32 = 50;
    let mut first:u32 = 1;
    let mut second:u32 = 1;

    if (fib_numbers == 0) || (fib_numbers == 1) {
        println!("{}" , first);
    }
    else {
        println!("{}" , first);
        println!("{}" , second);
    }

    for i in 1..fib_numbers-1 {
        let third:u32 = first + second;
        first = second;
        second = third;
        println!("{} " , second);
    }
}