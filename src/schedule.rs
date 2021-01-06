use sqlx::postgres::types::*;
use chrono::DateTime;
use chrono::offset::Utc;


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
