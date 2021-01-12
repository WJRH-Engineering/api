use std::collections::HashMap;
use redis;
use reqwest::get as fetch;
use serde::Deserialize;
use serde_json::from_str;

use chrono::DateTime;
use chrono::offset::Utc;

pub fn get_connection() -> redis::RedisResult<redis::Connection> {
	let client = redis::Client::open("redis://127.0.0.1/")?;
	let connection = client.get_connection()?;
	Ok(connection)
}

pub fn safe_get<'a>(hashmap: &'a HashMap<String, String>, key: &str) -> &'a str {
	if let Some(value) = hashmap.get(key) {
		value
	} else {
		""
	}
}

#[derive(Clone)]
pub struct TealProgram {
	pub scalars: HashMap<String, String>,
	pub episode_ids: Vec<String>,
}

impl TealProgram {
	pub fn get_from_redis(key: &str, connection: &mut redis::Connection) -> redis::RedisResult<Self> {

		let scalars: HashMap<String, String> = redis::cmd("hgetall")
			.arg(format!("programs:{}:scalars", key))
			.query(connection)?;

		let episode_ids: Vec<String> = redis::cmd("zrange")
			.arg(format!("programs:{}:episodes", key))
			.arg("0").arg("-1")
			.query(connection)?;

		let output = TealProgram { scalars, episode_ids };
		return Ok(output)
	}

	pub fn get_all(connection: &mut redis::Connection) -> Vec<Self> {
		let programs: Vec<String> = redis::cmd("smembers")
			.arg("programs")
			.query(connection)
			.unwrap();
		
		let mut output = vec![];
		for program in programs {
			output.push(TealProgram::get_from_redis(&program, connection).unwrap());	
		}

		output
	}

	pub fn get_episodes(&self, connection: &mut redis::Connection) -> Vec<TealEpisode> {
		let mut output: Vec<TealEpisode> = vec![];
		for id in &self.episode_ids {
			let episode = TealEpisode::get_from_redis(id, connection);
			if let Some(episode) = episode { output.push(episode); }
		}

		return output
	}

	//  TODO: sorting programs by date of last published episode takes too long because of repeated
	//  calls to the redis server. Features is shelved until it can be done more efficiently
	//  
	// /// Returns the timestamp of the last event experienced by this program. This will either be
	// /// the last episode published by the program, or, if no episodes exist, the date the program
	// /// was created. This will be used to sort the list of programs before returning them to the
	// /// user, with the goal of presenting more recently active shows higher in applications
	// pub fn last_event(&self, connection: &mut redis::Connection) -> Option<DateTime<Utc>> {
	// 	let episodes = self.get_episodes(connection);

	// 	if episodes.len() == 0 { return None }; 

	// 	let date = episodes[0].scalars.get("pubdate")?;

	// 	let dates = episodes.into_iter()
	// 		.map(|episode| -> DateTime<Utc> {episode.scalars.get("pubdate").unwrap().parse().unwrap()})
	// 		.max().unwrap();
	// 	Some(dates)
	// }
}

#[derive(Deserialize, Clone)]
pub struct TealEpisode {
	pub scalars: HashMap<String, String>,
	pub tracks: Option<Vec<TealTrack>>,
}

#[derive(Deserialize)]
struct EpisodeResult {
	pub tracks: Option<Vec<TealTrack>>,
}

impl TealEpisode {
	pub fn get_from_redis(key: &str, connection: &mut redis::Connection) -> Option<Self> {

		let result = redis::cmd("hgetall")
			.arg(format!("episodes:{}:scalars", key))
			.query(connection);

		let scalars: HashMap<String, String> = match result {
			Ok(scalars) => scalars,
			Err(_err) => return None,
		};

		let output = Self { scalars, tracks: None, };
		return Some(output)
	}

	pub async fn get_tracks(&self) -> Result<Vec<TealTrack>, reqwest::Error> {
		
		if let Some(id) = self.scalars.get("id") {
			let url = format!("https://api.teal.cool/episodes/{}", id);

			println!("{}", url);
			let response_string = fetch(&url).await?.text().await?;


			let response: EpisodeResult = serde_json::from_str(&response_string).unwrap();

			// println!("{}", id);
			// println!("{}", response.get("tracks").unwrap());


			match response.tracks {
				Some(tracks) => Ok(tracks),
				None => Ok(vec![]),
			}

			// let response: serde_json::Value = serde_json::from_str(&response_string).unwrap();

			// let tracks = response.as_object().unwrap()
			//	.get("tracks").unwrap()
			//	.as_array().unwrap();

			// let mut output = vec![];
			// for track in tracks {
				
			// }

			// Ok(tracks.clone())


			// let response: TealEpisode = from_str(&response_string)?.unwrap();

			// Ok(response.tracks)

		} else {
			Ok(vec![])
		}

	}
}


#[derive(Deserialize, Clone)]
pub struct TealTrack {
	pub title: Option<String>,
	pub artist: Option<String>,
	pub log_time: Option<String>,
	pub mbid: Option<String>,
	pub id: Option<String>,
}


