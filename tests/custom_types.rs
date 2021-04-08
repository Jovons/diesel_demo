use diesel::connection::SimpleConnection;
use diesel::deserialize::{self, FromSql};
use diesel::mysql::Mysql;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;
use testschema::*;
use std::io::Write;
use diesel::sql_types::Varchar;

#[macro_use]
extern crate diesel;

mod testschema;

table! {
    use diesel::sql_types::*;
    custom_types {
        id -> Integer,
        custom_enum -> crate::MyEnum,
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type  = "Varchar"]
pub enum MyEnum {
    Foo,
    Bar,
}

impl ToSql<Varchar, Mysql> for MyEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Mysql>) -> serialize::Result {
        match *self {
            MyEnum::Foo => out.write_all(b"foo")?,
            MyEnum::Bar => out.write_all(b"bar")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Mysql> for MyEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"foo" => Ok(MyEnum::Foo),
            b"bar" => Ok(MyEnum::Bar),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
#[table_name = "custom_types"]
struct HasCustomTypes {
    id: i32,
    custom_enum: self::MyEnum,
}

#[test]
fn custom_types_round_trip() {
    let data = vec![
        HasCustomTypes {
            id: 1,
            custom_enum: MyEnum::Foo,
        },
        HasCustomTypes {
            id: 2,
            custom_enum: MyEnum::Bar,
        },
    ];
    let connection = connection();
    connection
        .batch_execute(
            r#"
        CREATE TABLE `custom_types` (
          `id` int(11) NOT NULL AUTO_INCREMENT,
          `custom_enum` varchar(10) NOT NULL,
          PRIMARY KEY (`id`)
        ) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
    "#,
        )
        .unwrap();

    let inserted = insert_into(custom_types::table)
        .values(&data)
        .execute(&connection)
        .unwrap();
    assert_eq!(data, inserted);
}
