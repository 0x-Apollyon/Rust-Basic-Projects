struct user{
    username : String,
    isActive : bool,
    email : String,
    age : u8
}

fn main(){
    let main_user = user{
        username: String::from("apollyon"),
        email: String::from("apollyon@something.something"),
        age: 16,
        isActive: true
    };

    println!("The email address of the user is {:?} and the username is {:?}" , main_user.email , main_user.username);
}