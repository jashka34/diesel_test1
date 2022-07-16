extern crate diesel;
extern crate diesel_test1;

use self::diesel::prelude::*;
use self::diesel_test1::*;
use std::env::args;

fn main() {
    use diesel_test1::schema::usrs::dsl::*;

    let id_for_delete = args().nth(1).expect("Need id as arg for delete")
        .parse::<i32>().expect("Invalid id");
    let connection = establish_connection();

    let del_count = diesel::delete(usrs.find(id_for_delete))
        .execute(&connection)
        .expect("Error deleting user");
    println!("Deleted {} usrs", del_count);

}
