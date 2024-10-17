fn main() {
    let name = String::from("xapollyon");
    let first_vowel_index = vowel_finder(name);

    match first_vowel_index{
        Some(value) => println!("The first vowel is at the position {}" , value),
        None => println!("No vowel")
    
    };

    let sentence = String::from("one ring to rule them all, one ring to bind them");
    let first_word_of_sentence = find_first_word(&sentence);
    let last_word_of_sentence = find_last_word(&sentence);


    match first_word_of_sentence {
        Some(value) => println!("The first word is {}" , value),
        None => println!("Unknown error")
    }

    match last_word_of_sentence {
        Some(value) => println!("The last word is {}" , value),
        None => println!("Unknown error")
    }
}

fn vowel_finder(inpt_string : String) -> Option<u32> {
    for (i , j) in inpt_string.chars().enumerate() {
        if j == 'a' || j == 'e' || j == 'i' || j == 'o' || j == 'u' {
            return Some(i as u32);
        } 
    }

    return None;

}

fn find_first_word(inpt_string : &String) -> Option<String> {
    //we dont really need to use options here
    //the return type can be string too
    let mut first_word = String::from("");
    for (i , j) in inpt_string.chars().enumerate() {
        if j == ' ' {
            return Some(first_word as String);
        }
        else {
            first_word = first_word + &j.to_string();
        }
    }
    return Some(first_word as String);
}

fn find_last_word(inpt_string : &String) -> Option<String> {
    let mut last_word = String::from("");
    for (i , j) in inpt_string.chars().enumerate() {
        if j == ' ' {
            last_word = String::from("");
        }
        else {
            last_word = last_word + &j.to_string();
        }
    }
    return Some(last_word as String);
}