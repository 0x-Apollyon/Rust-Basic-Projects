//! ```cargo
//! [dependencies]
//! chrono = "0.4"
//! ```

extern crate chrono;
use chrono::{Local,Utc};
//make sure to install chrono before hand
//cargo add chrono
//also make sure to include the above 4 lines to include the crates in case you run this with cargo_script

fn main() {
    println!("Current UTC time is : {}" , Utc::now());

    let time_now = Utc::now().format("%Y/%m/%d %H-%M-%S");
    println!("Current time formatted is : {}" , time_now);

    println!("Current local time is : {}" , Local::now());
}