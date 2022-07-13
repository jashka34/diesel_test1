// extern crate diesel;
extern crate diesel_test1;

use self::diesel_test1::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Enter имя usr:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    let usr = create_usr(&connection, name);

    println!("Usr create with id: {} and name: {} ({})", usr.id, usr.name, EOF);

}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
