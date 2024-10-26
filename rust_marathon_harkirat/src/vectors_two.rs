// remove all vowels from a vector

fn main() {
    let mut main_vector = Vec::new();

    main_vector.push("a");
    main_vector.push("p");
    main_vector.push("l");

    remove_vowels(&mut main_vector);
    println!("Vector with removed vowels --> {:?}" , main_vector)
}

fn remove_vowels(vector : &mut Vec<&str>) {
    let mut i = 0;
    while i < vector.len(){
        if vector[i] == "a" || vector[i] == "e" || vector[i] == "i" || vector[i] == "o" || vector[i] == "u" {
            vector.remove(i);
        }
        else {
            i = i + 1;
        }
    }

}