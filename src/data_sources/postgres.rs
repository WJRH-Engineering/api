/// --------------------
/// Postgres Data Source
/// --------------------

use sqlx::postgres::types::*;
use chrono::DateTime;
use chrono::offset::Utc;
use sqlx::postgres::*;
use std::env;

// NOTE: currently, this url is not set correctly in order to protect the
// databas password. Please set it manually with the DATABASE_URL environment
// variable.
const DEFAULT_URL: &'static str = "postgres://wjrh:password@api.wjrh.org/testdb";

use dotenv::dotenv;

/// Create a connection to the Postgres Database and return it
async fn get_connection() -> PgPool {

    dotenv().ok();
    
	// Get the database url from the environment variables, use a default if none is set
	let url = match env::var("DATABASE_URL") {
		Ok(url) => url,
		Err(_) => {
			println!("The environment variable DATABASE_URL is not set.");
			println!("Please set it to the correct database and re-run the program");
			println!("falling back to a default");
			DEFAULT_URL.to_string()
		}
	};

	PgPoolOptions::new()
		.max_connections(5)
		.connect(&url)
		.await
		.unwrap()
}

#[derive(sqlx::FromRow)]
pub struct MountPoint{
	pub id: i32,
	pub shortname: String,
	pub password: String,
	pub mountpoint: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct TimeSlot{
	pub id: i32,
	pub shortname: String,
	pub time_range: PgRange<DateTime<Utc>>,
	pub year: i32, 
	pub season: String,
}

pub async fn query_schedule(year: i32, season: &str) -> Vec<TimeSlot> {
	let connection = get_connection().await;
	sqlx::query_as!(
		TimeSlot,
		"SELECT * FROM SCHEDULE WHERE year = $1 AND season = $2",
		year, season.to_string(),
	).fetch_all(&connection).await.unwrap()
}


 #[cfg(test)]
 mod tests {
	use super::*;

	#[test]
	fn database_url_is_set(){
        dotenv().ok();
	   &env::var("DATABASE_URL")
		   .expect("enironment variable: \"DATABASE_URL\" is not set"); 
	}

	#[async_std::test]	
	async fn test_database_connection() {
		get_connection().await;
	}

	#[async_std::test]	
	async fn schedule() {
		let schedule = query_schedule(2020, "FALL").await;	
		// panic!("{:#?}", schedule);
	}
 }
