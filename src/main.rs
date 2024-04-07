use sqlx::{migrate::MigrateDatabase, Sqlite};

mod tui;
mod dbrind;

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    tui::termui().unwrap();
}
