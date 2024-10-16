fn main() {
    println!("{}" , is_even(19));
}

fn is_even(numb: i32) -> bool {
    if numb % 2 == 0 {
        return true;
    }
    else {
        return false;
    }
}