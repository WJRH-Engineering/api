use sqlx::postgres::types::*;
use chrono::DateTime;
use chrono::offset::Utc;

use sqlx::postgres::*;
use sqlx::*;

pub async fn get_connection() -> PgPool {
	let database_url = "postgres://dev:hackme@api.wjrh.org:5432/graphqltest";
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(database_url)
		.await
		.unwrap();

	return pool
}


// type alias for postgresql timerange
type TimeRange = PgRange<DateTime<Utc>>;

#[derive(sqlx::FromRow)]
pub struct MountPoint{
	pub id: i32,
	pub shortname: String,
	pub password: String,
	pub mountpoint: String,
}


// #[derive(GraphQLObject)]
#[derive(sqlx::FromRow)]
pub struct TimeSlot{
	pub id: i32,
	pub shortname: String,
	pub time_range: TimeRange,
}

impl TimeSlot {
	pub async fn get_schedule() -> Vec<TimeSlot> {
		let connection = get_connection().await;
		let result = sqlx::query_as::<_, TimeSlot>("select * from schedule")
			.fetch_all(&connection)
			.await
			.unwrap();

		result
	}
}


// #[cfg(test)]
// mod tests {
//	use super::*;

//	#[test]	
//	fn test_1() {
//		let (res, errors) = juniper::execute(
//			"query {example {shortname}}",
//			None,
//			&Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
//			&Variables::new(),
//			&()
//		).unwrap();
//	}
// }
