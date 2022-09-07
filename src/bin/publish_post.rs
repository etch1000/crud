use self::models::Post;
use crud::*;
use diesel::prelude::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("piblish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_db_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .unwrap();

    println!("Published post {}", post.title);
}
