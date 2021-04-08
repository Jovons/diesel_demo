use diesel_demo::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use diesel_demo::schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(false))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {

        println!("{}", post.title);
        match post.published {
            MyEnum::A => println!("published is false"),
            MyEnum::B => println!("published is true"),
        }
    }
}


