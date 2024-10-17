use std::fs;


fn main() {
    let file_path = String::from("main.rs");
    let file_content = read_from_file(file_path);
    
    match file_content {
        Ok(file_content) => {
            println!("File read success");
            println!("{}" , file_content);
        },
        Err(error) => {
            println!("Error while reading file --> {}" , error);
        }
    }

}

fn read_from_file(filepath: String) -> Result<String,String> {
    let file_content = fs::read_to_string(filepath);

    match file_content {
        Ok(file_cc) => return Ok(file_cc),
        Err(error) => Err(String::from("File reading error"))
    }
    
}
