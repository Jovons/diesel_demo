// use diesel_demo::models::*; 方法1，绝对路径，通过依赖名称引入
// use self::models::*; 方法2，相对路径同名依赖子模块
// use models::*; 方法3，相对路径中的self可以省略
use crate::models::*; // 方法4， 绝对路径，通过crate关键字引入
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    // use diesel_demo::schema::posts::dsl::*; // 方法1
    // use self::schema::posts::dsl::*; // 方法2
    // use schema::posts::dsl::*; // 方法3
    use crate::schema::posts::dsl::*; // 方法4
    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
