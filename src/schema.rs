use juniper::{graphql_object, GraphQLObject};
use sqlx::postgres::PgPoolOptions;
use serde::Deserialize;
use serde_json;
use redis;
use core::ops::Bound::*;
use chrono::Utc;
use chrono::DateTime;

use std::collections::HashMap;

use crate::teal::*;
use crate::lastfm::*;


#[graphql_object]
impl Song {
	async fn title(&self) -> String {self.title.clone()}
}

// use type aliases to rename the type for graphql
type Program = TealProgram;
type Episode = TealEpisode;

#[graphql_object]
impl Program {
	pub fn name(&self) -> &str { safe_get(&self.scalars, "name") }

	/// A shortened, url safe version of the name used to refererence the Program
	pub fn shortname(&self) -> &str { safe_get(&self.scalars, "shortname") }
	pub fn description(&self) -> &str { safe_get(&self.scalars, "description") }
	pub fn cover_image(&self) -> &str { safe_get(&self.scalars, "cover_image") }
	pub fn id(&self) -> &str { safe_get(&self.scalars, "id") }
	pub fn author(&self) -> &str { safe_get(&self.scalars, "author") }
	pub fn stream(&self) -> &str { safe_get(&self.scalars, "stream") }

	/// A list of episodes that this program has recorded
	pub fn episodes(&self) -> Vec<Episode> {
		let client = redis::Client::open("redis://127.0.0.1/").unwrap();
		let mut connection = client.get_connection().unwrap();
		let mut output: Vec<TealEpisode> = vec![];
		for id in &self.episode_ids {
			let episode = TealEpisode::get_from_redis(id, &mut connection).unwrap();
			output.push(episode);
		}

		output
	}
}

#[graphql_object]
impl Episode {
	pub fn id(&self) -> &str  { safe_get(&self.scalars, "id") }

	pub async fn tracks(&self) -> Vec<TealTrack> { 
		vec![]
		// if let Some(id) = self.scalars.get("id") {
		//	let result = TealTrack::get_from_teal(id).await;
		//	match result {
		//		Vec<>(tracks) => tracks,
		//		None => vec![],
		//	}
		// } else {
		//	return vec![]
		// }

	}
}


#[graphql_object]
impl TealTrack {
	pub fn title(&self) -> &str {"title"}
}

use super::Query;

#[graphql_object]
impl Query {
	async fn lookup_song(title: String, artist: String) -> Song {
		Song{title: "test".to_string(), artist: "test".to_string(), details: None}
	}

	async fn programs() -> Vec<TealProgram> {
		TealProgram::get_all()
	}

	// async fn program(shortname: String) -> Program {

	// }

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

use crate::schedule::TimeSlot;
use crate::schedule::MountPoint;

#[graphql_object]
impl MountPoint {
	fn mountpoint(&self) -> &str { "mountpoint" }
}

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

