use self::models::*;
use crud::*;
use diesel::prelude::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_db_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
