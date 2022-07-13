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

    println!("Displaying {} posts", results.len());
    for u in results {
        println!("{}", u.id);
        println!("----------\n");
        println!("{}", u.name);
    }
}


