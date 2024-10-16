
struct Student {
    age: u8,
    name: String,
    class: u8,
    section: String,
    admission_number: u32,
}

impl Student {
    fn name_starts_with_a(&self) -> bool {
        let char_to_check : char = 'a';
        let char_to_check_upper : char = 'A';
        if (self.name.chars().next().unwrap() == char_to_check) || (self.name.chars().next().unwrap() == char_to_check_upper) {
            return true;
        }
        else {
            return false;
        }
    }

    fn static_func() -> u32 {
        let something : u32 = 12345;
        return something;
    }
}

fn main() {
    let apollyon = Student{
        age: 16,
        name: String::from("Apollyon"),
        class: 12,
        section: String::from("D"),
        admission_number: 12345
    };

    println!("Name of student starts with A: {}" , apollyon.name_starts_with_a());
    println!("Static function {}" , Student::static_func());
}