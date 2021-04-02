use diesel::{Queryable,Insertable};
// use super::schema::posts; // 方法1，相对路径
use crate::schema::posts; // 方法2， 绝对路径

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
