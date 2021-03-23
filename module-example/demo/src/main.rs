extern crate phrases;

use phrases::greetings::french;

fn main () {
    // To show both options to use the module
    println!("English = {}", phrases::greetings::english::hello());
    println!("French = {}", french::hello());
}