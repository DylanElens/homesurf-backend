use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::files;

#[derive(Queryable, Debug)]
pub struct File {
    pub id: i32,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub file_name: &'a str,
    pub file_path: &'a str,
    pub file_type: &'a str,
    pub created_at: NaiveDateTime
}
