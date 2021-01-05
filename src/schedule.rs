use juniper::{graphql_object, GraphQLObject};
use sqlx::postgres::PgPoolOptions;
use serde::Deserialize;
use serde_json;
use redis;

use std::collections::HashMap;

use crate::teal::*;


#[graphql_object]
impl TealProgram {
	pub fn name(&self) -> &str { safe_get(&self.scalars, "name") }
	pub fn shortname(&self) -> &str { safe_get(&self.scalars, "shortname") }
	pub fn description(&self) -> &str { safe_get(&self.scalars, "description") }
	pub fn cover_image(&self) -> &str { safe_get(&self.scalars, "cover_image") }
	pub fn id(&self) -> &str { safe_get(&self.scalars, "id") }
	pub fn author(&self) -> &str { safe_get(&self.scalars, "author") }
	pub fn stream(&self) -> &str { safe_get(&self.scalars, "stream") }
}

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

use reqwest as fetch;
use url::{Url, ParseError};

struct Song {
	title: String,
	artist: String,
	details: Option<LastFMTrack>,
}

#[graphql_object]
impl Song {
	async fn title(&self) -> String {self.title.clone()}
}

#[derive(Deserialize)]
struct LastFMArtist {
	name: String,
	mbid: String,
	url: String,
}

#[derive(Deserialize)]
struct LastFMTrack {
	name: String,
	mbid: String,
	url: String,
	duration: String,
	artist: LastFMArtist,
}

#[derive(Deserialize)]
struct LastFMResponse {
	track: Option<LastFMTrack>,
}

use super::Query;

pub async fn lookup_song(title: String, artist: String) {
	let url_base = "ws.audioscrobbler.com/2.0/";	
	let url = format!("http://{base}?{method}&api_key={lastfmkey}&artist={artist}&track={track}&autocorrect&format=json",
		base = url_base,
		lastfmkey = "14cacc2d28210dcd318ffa2085778844",
		method = "method=track.getInfo",
		artist = "cake",
		track = "the+distance",
	);

	println!("{}", url);

	let response_string = fetch::get(&url).await.unwrap().text().await.unwrap();
	let response: LastFMResponse = serde_json::from_str(&response_string).unwrap();
	let track: LastFMTrack = match response.track {
		Some(track) => track,
		None => panic!(),
	};

	println!("{}", track.artist.name);
	
}

#[graphql_object]
impl Query {

	async fn lookup_song(title: String, artist: String) -> Song {
		Song{title: "test".to_string(), artist: "test".to_string(), details: None}
	}

	async fn programs() -> Vec<TealProgram> {
		TealProgram::get_all()
	}

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
