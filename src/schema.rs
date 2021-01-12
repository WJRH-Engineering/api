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
use crate::teal;
use crate::lastfm::*;
use crate::schedule::TimeSlot;
use crate::schedule::MountPoint;
use crate::schedule;

use crate::lastfm;

use super::Query;


#[graphql_object]
impl Query {
	/// Look up a song on Last FM
	async fn lookup_song(title: String, artist: String) -> Song {
		lastfm::lookup_song(&title, &artist).await
	}

	/// Get all teal programs associated with WJRH
	async fn programs(mut limit: Option<i32>) -> Vec<TealProgram> {
		let mut connection = teal::get_connection().unwrap();
		let mut programs = TealProgram::get_all(&mut connection);

		// if limit has a value less than 0, set it to None
		if let Some(value) = limit { limit = if value >= 0 { limit } else { None }};

		match limit {
			Some(limit) => programs[0..limit as usize].to_vec(),
			None => programs[..].to_vec(),
		}
	}

	/// Get a specific teal program
	async fn program(shortname: String) -> Program {
		let mut connection = teal::get_connection().unwrap();
		TealProgram::get_from_redis(&shortname, &mut connection)
			.expect("Cannot find program shortname")
	}

	/// Get the current schedule
	async fn schedule() -> Vec<TimeSlot> {
		schedule::TimeSlot::get_schedule().await
	}
}


// use type aliases to rename the type for graphql
type Program = TealProgram;
type Episode = TealEpisode;
type Track = TealTrack;

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
		let mut connection = teal::get_connection().unwrap();
		let mut output: Vec<TealEpisode> = vec![];
		for id in &self.episode_ids {
			let episode = TealEpisode::get_from_redis(id, &mut connection).unwrap();
			output.push(episode);
		}

		return output
	}
}

#[graphql_object]
impl Episode {
	pub fn id(&self) -> &str  { safe_get(&self.scalars, "id") }
	pub fn audio_url(&self) -> &str  { safe_get(&self.scalars, "audio_url") }
	pub fn delay(&self) -> i32 { safe_get(&self.scalars, "delay").parse().unwrap() }
	pub fn description(&self) -> &str  { safe_get(&self.scalars, "description") }
	pub fn start_time(&self) -> DateTime<Utc>  { str::parse(safe_get(&self.scalars, "start_time")).unwrap() }
	pub fn end_time(&self) -> DateTime<Utc>  { str::parse(safe_get(&self.scalars, "end_time")).unwrap() }
	pub fn explicit(&self) -> &str  { safe_get(&self.scalars, "explicit") }

	/// A list of tracks logged during this episode
	pub async fn tracks(&self) -> Vec<TealTrack> { 
		self.get_tracks().await.unwrap()
	}
}

#[graphql_object]
impl Track {

	/// Metadata about the track pulled from the LastFM database. If the song cannot be found in
	/// the database, only the title and artist fields will exist, and they will match the title
	/// and artist fields in the track object.
	/// It is recommended to use the title and artist fields in song whenever possible, because
	/// LastFM will attempt to do some formatting and auto-correction to the result.
	pub async fn song(&self) -> Option<Song> {
		let title = self.title.clone()?;
		let artist = self.artist.clone()?;
		Some(lastfm::lookup_song(&title, &artist).await)
	}

	/// Equivalent to the title field in the song object, except that it will not trigger a call
	/// to the LastFM api. Instead, it will always return the value stored in the Teal database.
	/// Using this field can drastically improve performance for large queries
	pub fn title(&self) -> Option<String> {self.clone().title}

	/// Equivalent to the artist field in the song object, except that it will not trigger a call
	/// to the LastFM api. Instead, it will always return the value stored in the Teal database.
	/// Using this field can drastically improve performance for large queries
	pub fn artist(&self) -> Option<String> {self.clone().artist}
	

	pub fn log_time(&self) -> Option<String> {self.clone().log_time}

	/// musicbrainz id, used to identify the song in the music brainz database
	/// https://musicbrainz.org/
	pub fn mbid(&self) -> Option<String> {self.clone().mbid}

	/// teal specific id
	pub fn id(&self) -> Option<String> {self.clone().id}
}


#[graphql_object]
impl Song {
	pub fn title(&self) -> String { 

		// clone self.details so we can do operations on it
		let details = self.details.clone();
		
		// use this line to find out if there is a value at self.details.name and assign it to
		// title. title is assigned None if either details or name is None
		// use a clojure that returns an option in order to allow use of the ? operator
		let title = (|details: Option<LastFMTrack>| Some(details?.name?))(details);

		// match against the result of the previous expression. Return its value if there is one,
		// or the default title if not.
		match title {
			Some(title) => title,
			None => self.title.clone(),
		}
	}

	pub fn artist(&self) -> String { 
		let details = self.details.clone();
		// same logic as title, but one layer deeper
		let artist = (|details: Option<LastFMTrack>| Some(details?.artist?.name?))(details);
		match artist {
			Some(artist) => artist,
			None => self.artist.clone(),
		}
	}

	pub fn url(&self) -> Option<String> {
		let details = self.details.clone();
		Some(details?.url?)
	}

	pub fn wiki(&self) -> Option<String> {
		let details = self.details.clone();
		Some(details?.wiki?.content?)
		
	}

}

#[graphql_object]
impl TimeSlot {
	/// the shortname of the show
	fn shortname(&self) -> &str { &self.shortname }

	/// program info for the show
	fn program(&self) -> Program {
		let mut connection = teal::get_connection().unwrap();	
		TealProgram::get_from_redis(&self.shortname, &mut connection)
			.expect("Cannot find program")
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

