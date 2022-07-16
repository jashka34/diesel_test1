extern crate diesel_test1;
extern crate diesel;

use diesel_test1::*;
use self::models::*;
use diesel::prelude::*;


fn main() {
    use diesel_test1::schema::usrs::dsl::*;

    let connection = establish_connection();
    let results = usrs.filter(id.ge(0))
                     .limit(5)
                     .load::<Usr>(&connection)
                     .expect("Error loading usr");

    println!("Displaying {} usr", results.len());
    println!("---------------------\n");
    for u in results {
        println!("{} {} {}", u.id, u.name, u.active);
        println!("---------------------\n");
    }
}


