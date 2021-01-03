use diesel::prelude::*;
// use diesel::sql_types;
use diesel::pg::PgConnection;

use juniper::{
	graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
	GraphQLEnum, Variables, GraphQLObject,
};

table! {
	mounts {
		id -> Integer,
		shortname -> VarChar,
		password -> VarChar,
		mountpoint -> VarChar,
	}
}

table!{
	// use diesel::sql_types::*;
	schedule {
		id -> Integer,
		shortname -> VarChar,
		time_range -> VarChar,
		year -> VarChar,
		season -> VarChar,
	}
}

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct Mount {
	pub id: i32,
	pub shortname: String,
	pub mountpoint: String,
	pub password: String,
}

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct Timeslot {
	pub id: i32,
	pub shortname: String,
	pub timerange: String,
	pub year: String,
	pub season: String,
}


// database functions
pub fn establish_connection() -> PgConnection {
	let database_url = "postgres://dev:hackme@api.wjrh.org:5432/graphqltest";
	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

pub fn get_mounts(connection: PgConnection) -> Vec<Mount>{
	use crate::schedule::mounts::dsl::*;
	let results = mounts
		.load::<Mount>(&connection)
		.expect("error");

	return results;
}

pub fn get_schedule(conn: PgConnection) -> Vec<Timeslot>{
	use crate::schedule::schedule::dsl::*;
	return schedule.load::<Timeslot>(&conn)
		.expect("error")
}

// pub struct Query;

use super::Query;
#[graphql_object()]
impl Query {
	fn example_mount(&self) -> Mount{
		Mount{
			id: 1,	
			shortname: "example".to_string(),
			mountpoint: "example".to_string(),
			password: "ABCDE".to_string(),
		}
	}

	fn mounts(&self) -> Vec<Mount> {
		let connection = establish_connection();
		let mounts = get_mounts(connection);

		return mounts;
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	/// test that a connection to Postgres can be established
	#[test]
	fn test_connect() {
		establish_connection();
	}

	#[test]
	fn test_schedule(){
		let conn = establish_connection();
		get_schedule(conn);
	}
}
