#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

table! {
	mounts {
		id -> Integer,
		shortname -> VarChar,
		password -> VarChar,
		mountpoint -> VarChar,
	}
}

#[derive(Insertable)]
#[table_name = "mounts"]
pub struct MountInsert {
	pub shortname: String,
	pub password: String,
	pub mountpoint: String,
}

#[derive(Queryable)]
pub struct Mount {
	pub id: i32,
	pub shortname: String,
	pub password: String,
	pub mountpoint: String,
}

pub fn establish_connection() -> PgConnection {
	let database_url = "postgres://dev:hackme@api.wjrh.org:5432/graphqltest";
	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

fn main() {

	use crate::mounts::dsl::*;

	let connection = establish_connection();
	// let  = PostInsert {
	// 	title: "Post1".to_string(),
	// 	message: "A small post".to_string(),
	// };

	let results = mounts
		// .select(shortname)
		// .filter()
		.load::<Mount>(&connection)
		.expect("Error");

	// let result: Post = diesel::insert_into(posts::table).values(&post)
	// 	.get_result(&connection).expect("Error creating post");

	println!("{}", results[10].shortname)
}

