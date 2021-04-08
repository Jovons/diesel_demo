use diesel::{Queryable,Insertable};
use chrono;
use bigdecimal;
use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub title: String,
    pub published: MyEnum,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub create_time_timestamp: chrono::NaiveDateTime,
    pub spent: bigdecimal::BigDecimal,
    pub spent_num: bigdecimal::BigDecimal,
    pub array_data: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub my_title: &'a str,
    pub body: &'a str,
}

use diesel::backend::Backend;
use diesel::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use std::io::Write;

#[repr(i32)]
#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression)]
pub enum MyEnum {
    A = 0,
    B = 1,
}

impl<DB> FromSql<Bool, DB> for MyEnum
    where
        DB: Backend,
        bool: FromSql<Bool, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match bool::from_sql(bytes)? {
            false => Ok(MyEnum::A),
            true => Ok(MyEnum::B),
        }
    }
}

impl<DB> ToSql<Bool, DB> for MyEnum
    where
        DB: Backend,
        i32: ToSql<Bool, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i32).to_sql(out)
    }
}