pub mod models;
pub mod schema;
use crate::schema::files;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use models::{File, NewFile};
use std::env;
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn list_files(conn: &mut SqliteConnection) -> Vec<File> {
    use self::schema::files::dsl::*;

    files
        .limit(5)
        .load::<File>(conn)
        .expect("Error loading posts")
}

pub fn create_file(
    conn: &mut SqliteConnection,
    file_name: &str,
    file_path: &str,
    file_type: &str,
) -> Result<usize, diesel::result::Error> {
    let created_at = Utc::now().naive_utc();
    let new_file = NewFile {
        file_name,
        file_path,
        file_type,
        created_at,
    };

    diesel::insert_into(files::table)
        .values(&new_file)
        .execute(conn)
}
