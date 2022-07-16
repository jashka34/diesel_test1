extern crate diesel;
extern crate diesel_test1;

use self::diesel::prelude::*;
use self::diesel_test1::*;
use self::models::Usr;
use std::env::args;

fn main() {
    use diesel_test1::schema::usrs::dsl::{usrs, active};

    let id = args().nth(1).expect("Need id as arg")
        .parse::<i32>().expect("Invalid id");
    let connection = establish_connection();

    let usr = diesel::update(usrs.find(id))
        .set(active.eq(true))
        .get_result::<Usr>(&connection)
        .expect(&format!("Unable to find usr {}", id));
    println!("User {} is active now", usr.name);

}
