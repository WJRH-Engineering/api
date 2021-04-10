// // use juniper::{graphql_object, GraphQLObject};

// use sqlx::postgres::PgPoolOptions;
// use serde::Deserialize;
// use serde_json;
// use redis;
// use core::ops::Bound::*;
// use chrono::Utc;
// use chrono::DateTime;

// use std::collections::HashMap;

// use crate::teal::*;
// use crate::teal;
// use crate::lastfm::*;
// use crate::schedule::TimeSlot;
// use crate::schedule::MountPoint;
// use crate::schedule;

use async_graphql::MergedObject;

use crate::graphql_types::teal;
use crate::graphql_types::song;
use crate::graphql_types::schedule;
// use crate::schedule;

//#[derive(MergedObject, Default)]
// pub struct Query(lastfm::Query, schedule::Query);

#[derive(MergedObject, Default)]
pub struct Query(teal::TealQuery, song::SongQuery, schedule::ScheduleQuery);


// #[Object]
// impl Query {
// 	/// Look up a song on Last FM
// 	pub async fn lookup_song(&self, title: String, artist: String) -> Song {
// 		lastfm::lookup_song(&title, &artist).await
// 	}

// 	/// Get all teal programs associated with WJRH
// 	pub async fn programs(&self, mut limit: Option<i32>) -> Vec<TealProgram> {
// 		let mut connection = teal::get_connection().unwrap();
// 		let mut programs = TealProgram::get_all(&mut connection);

// 		// if limit has a value less than 0, set it to None
// 		if let Some(value) = limit { limit = if value >= 0 { limit } else { None }};

// 		match limit {
// 			Some(limit) => programs[0..limit as usize].to_vec(),
// 			None => programs[..].to_vec(),
// 		}
// 	}

// 	/// Get a specific teal program
// 	async fn program(&self, shortname: String) -> Program {
// 		let mut connection = teal::get_connection().unwrap();
// 		TealProgram::get_from_redis(&shortname, &mut connection)
// 			.expect("Cannot find program shortname")
// 	}

// 	/// Get the current schedule
// 	async fn schedule(&self) -> Vec<TimeSlot> {
// 		schedule::TimeSlot::get_schedule().await
// 	}
// }


// // use type aliases to rename the type for graphql
// type Program = TealProgram;
// type Episode = TealEpisode;
// type Track = TealTrack;

// #[Object]
// impl Program {
// 	pub async fn name(&self) -> &str { safe_get(&self.scalars, "name") }

// 	/// A shortened, url safe version of the name used to refererence the Program
// 	pub async fn shortname(&self) -> &str { safe_get(&self.scalars, "shortname") }
// 	pub async fn description(&self) -> &str { safe_get(&self.scalars, "description") }
// 	pub async fn cover_image(&self) -> &str { safe_get(&self.scalars, "cover_image") }
// 	pub async fn id(&self) -> &str { safe_get(&self.scalars, "id") }
// 	pub async fn author(&self) -> &str { safe_get(&self.scalars, "author") }
// 	pub async fn stream(&self) -> &str { safe_get(&self.scalars, "stream") }

// 	/// A list of episodes that this program has recorded
// 	pub async fn episodes(&self) -> Vec<Episode> {
// 		let mut connection = teal::get_connection().unwrap();
// 		let mut output: Vec<TealEpisode> = vec![];
// 		for id in &self.episode_ids {
// 			let episode = TealEpisode::get_from_redis(id, &mut connection).unwrap();
// 			output.push(episode);
// 		}

// 		return output
// 	}
// }

// #[Object]
// impl Episode {
// 	pub async fn id(&self) -> &str  { safe_get(&self.scalars, "id") }
// 	pub async fn audio_url(&self) -> &str  { safe_get(&self.scalars, "audio_url") }
// 	pub async fn delay(&self) -> i32 { safe_get(&self.scalars, "delay").parse().unwrap() }
// 	pub async fn description(&self) -> &str  { safe_get(&self.scalars, "description") }
// 	pub async fn start_time(&self) -> DateTime<Utc>  { str::parse(safe_get(&self.scalars, "start_time")).unwrap() }
// 	pub async fn end_time(&self) -> DateTime<Utc>  { str::parse(safe_get(&self.scalars, "end_time")).unwrap() }
// 	pub async fn explicit(&self) -> &str  { safe_get(&self.scalars, "explicit") }

// 	/// A list of tracks logged during this episode
// 	pub async fn tracks(&self) -> Vec<TealTrack> { 
// 		self.get_tracks().await.unwrap()
// 	}
// }

// #[Object]
// impl Track {

// 	/// Metadata about the track pulled from the LastFM database. If the song cannot be found in
// 	/// the database, only the title and artist fields will exist, and they will match the title
// 	/// and artist fields in the track object.
// 	/// It is recommended to use the title and artist fields in song whenever possible, because
// 	/// LastFM will attempt to do some formatting and auto-correction to the result.
// 	pub async fn song(&self) -> Option<Song> {
// 		let title = self.title.clone()?;
// 		let artist = self.artist.clone()?;
// 		Some(lastfm::lookup_song(&title, &artist).await)
// 	}

// 	/// Equivalent to the title field in the song object, except that it will not trigger a call
// 	/// to the LastFM api. Instead, it will always return the value stored in the Teal database.
// 	/// Using this field can drastically improve performance for large queries
// 	pub async fn title(&self) -> Option<String> {self.clone().title}

// 	/// Equivalent to the artist field in the song object, except that it will not trigger a call
// 	/// to the LastFM api. Instead, it will always return the value stored in the Teal database.
// 	/// Using this field can drastically improve performance for large queries
// 	pub async fn artist(&self) -> Option<String> {self.clone().artist}
	

// 	pub async fn log_time(&self) -> Option<String> {self.clone().log_time}

// 	/// musicbrainz id, used to identify the song in the music brainz database
// 	/// https://musicbrainz.org/
// 	pub async fn mbid(&self) -> Option<String> {self.clone().mbid}

// 	/// teal specific id
// 	pub async fn id(&self) -> Option<String> {self.clone().id}
// }


// #[Object]
// impl Song {
// 	pub async fn title(&self) -> String { 

// 		// clone self.details so we can do operations on it
// 		let details = self.details.clone();
		
// 		// use this line to find out if there is a value at self.details.name and assign it to
// 		// title. title is assigned None if either details or name is None
// 		// use a clojure that returns an option in order to allow use of the ? operator
// 		let title = (|details: Option<LastFMTrack>| Some(details?.name?))(details);

// 		// match against the result of the previous expression. Return its value if there is one,
// 		// or the default title if not.
// 		match title {
// 			Some(title) => title,
// 			None => self.title.clone(),
// 		}
// 	}

// 	pub async fn artist(&self) -> String { 
// 		let details = self.details.clone();
// 		// same logic as title, but one layer deeper
// 		let artist = (|details: Option<LastFMTrack>| Some(details?.artist?.name?))(details);
// 		match artist {
// 			Some(artist) => artist,
// 			None => self.artist.clone(),
// 		}
// 	}

// 	pub async fn url(&self) -> Option<String> {
// 		let details = self.details.clone();
// 		Some(details?.url?)
// 	}

// 	pub async fn wiki(&self) -> Option<String> {
// 		let details = self.details.clone();
// 		Some(details?.wiki?.content?)
		
// 	}

// }

// #[Object]
// impl TimeSlot {
// 	/// the shortname of the show
// 	async fn shortname(&self) -> &str { &self.shortname }

// 	/// program info for the show
// 	async fn program(&self) -> Program {
// 		let mut connection = teal::get_connection().unwrap();	
// 		TealProgram::get_from_redis(&self.shortname, &mut connection)
// 			.expect("Cannot find program")
// 	}

// 	/// the show's start time
// 	async fn start(&self) -> DateTime<Utc> {
// 		match self.time_range.start {
// 			Included(time) => time,
// 			Excluded(time) => time,
// 			Unbounded => panic!(),
// 		}
// 	}
	
// 	/// the show's end time
// 	async fn end(&self) -> DateTime<Utc> {
// 		match self.time_range.end {
// 			Included(time) => time,
// 			Excluded(time) => time,
// 			Unbounded => panic!(),
// 		}
// 	}
// }

