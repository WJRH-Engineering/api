 use juniper::{
	graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
	GraphQLEnum, Variables, GraphQLObject,
 };


use::sqlx::postgres::PgPoolOptions;

// use std::str::FromStr;

#[derive(sqlx::FromRow)]
#[derive(GraphQLObject)]
#[derive(Clone)]
struct MountPoint{
	id: i32,
	shortname: String,
	password: String,
	mountpoint: String,
}

use sqlx::postgres::types::*;
use chrono::DateTime;
use chrono::offset::Utc;

// type alias for postgresql timerange
type TimeRange = PgRange<DateTime<Utc>>;

// #[derive(GraphQLObject)]
#[derive(sqlx::FromRow)]
struct TimeSlot{
	id: i32,
	shortname: String,
	time_range: TimeRange,
}


use std::ops::Bound::*;

#[graphql_object]
impl TimeSlot {
	/// the shortname of the show
	fn show(&self) -> &str {
		&self.shortname	
	}

	/// the show's start time
	fn start(&self) -> DateTime<Utc> {
		match self.time_range.start {
			Included(time) => time,
			Excluded(time) => time,
			Unbounded => panic!(),
		}
	}
	
	/// the show's end time
	fn end(&self) -> DateTime<Utc> {
		match self.time_range.end {
			Included(time) => time,
			Excluded(time) => time,
			Unbounded => panic!(),
		}
	}
}

use super::Query;
#[graphql_object()]
impl Query {
	async fn mountpoint(shortname: String) -> MountPoint {
		let database_url = "postgres://dev:hackme@api.wjrh.org:5432/graphqltest";
	
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(database_url)
			.await
			.unwrap();
	
		let result = sqlx::query_as::<_, MountPoint>("select * from mounts where shortname=$1;")
			.bind(shortname)
			.fetch_one(&pool)
			.await
			.unwrap();

		return result
	}

	async fn schedule() -> Vec<TimeSlot> {
		let database_url = "postgres://dev:hackme@api.wjrh.org:5432/graphqltest";
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(database_url)
			.await
			.unwrap();

		let result = sqlx::query_as::<_, TimeSlot>("select * from schedule")
			.bind("")
			.fetch_all(&pool)
			.await
			.unwrap();

		return result
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
