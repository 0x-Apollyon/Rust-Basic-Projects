fn main() {
    let string_main = String::from("hello");
    // println!("The length of the string is {}" , string_main.chars().count())    
    println!("The length of the string is {}" , length_function(&string_main))
}

fn length_function(s : &str) -> usize {
    return s.chars().count()
}