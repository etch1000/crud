use crud::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_db_connection();

    let mut title = String::new();

    let mut body = String::new();

    println!("Title of your post : ");

    stdin().read_line(&mut title).unwrap();

    let title = title.trim_end();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );

    stdin().read_line(&mut body).unwrap();

    let post = create_post(connection, title, &body);

    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
