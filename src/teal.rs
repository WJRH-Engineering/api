use std::collections::HashMap;
use redis;
use reqwest::get as fetch;
use serde::Deserialize;
use serde_json::from_str;

pub struct TealProgram {
	pub scalars: HashMap<String, String>,
	pub episode_ids: Vec<String>,
}

fn open_redis_connection(url: &str) -> redis::RedisResult<redis::Connection> {
	let client = redis::Client::open("redis://127.0.0.1/").unwrap();
	let mut connection = client.get_connection().unwrap();
	Ok(connection)
}

impl TealProgram {
	pub fn get_from_redis(key: &str, connection_param: Option<&mut redis::Connection>) -> redis::RedisResult<Self> {

		let mut connection = match connection_param {
			Some(connection) => connection,
			None => &mut open_redis_connection("").unwrap(),
		};

		let scalars : HashMap<String, String> = redis::cmd("hgetall").arg(format!("programs:{}:scalars", key)).query(connection)?;
		let episode_ids : Vec<String> = redis::cmd("zrange").arg(format!("programs:{}:episodes", key)).arg("0").arg("-1").query(connection)?;
		let output = TealProgram { scalars, episode_ids };

		return Ok(output)
	}

	pub fn get_all() -> Vec<Self> {
		let client = redis::Client::open("redis://127.0.0.1/").unwrap();
		let mut connection = client.get_connection().unwrap();

		let programs: Vec<String> = redis::cmd("smembers").arg("programs").query(&mut connection).unwrap();
		
		let mut output = vec![];
		for program in programs {
			output.push(TealProgram::get_from_redis(&program, Some(&mut connection)).unwrap());	
		}

		output
	}
}

#[derive(Deserialize)]
pub struct TealEpisode {
	pub scalars: HashMap<String, String>,
	pub tracks: Option<Vec<TealTrack>>,
}

impl TealEpisode {
	pub fn get_from_redis(key: &str, connection: &mut redis::Connection) -> redis::RedisResult<Self> {
		let scalars : HashMap<String, String> = redis::cmd("hgetall").arg(format!("episodes:{}:scalars", key)).query(connection)?;
		let output = Self { scalars, tracks: None, };

		return Ok(output)
	}

	pub async fn get_tracks(&self) -> Result<Vec<TealTrack>, reqwest::Error> {
		
		if let Some(id) = self.scalars.get("id") {
			let url = format!("https://api.teal.cool/episodes/{}", id);
			let response_string = fetch(&url).await?.text().await?;
			let response: TealEpisode = serde_json::from_str(&response_string).unwrap();
			return Ok(response.tracks.unwrap());

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


#[derive(Deserialize)]
pub struct TealTrack {
	pub title: String,
	pub artist: String,
	pub log_time: String,
	pub mbid: String,
	pub id: String,
}

impl TealTrack {
	// pub async fn get_from_teal(id: &str) -> Result<Self, reqwest::Error> {
	//	let url = format!("https://api.teal.cool/episodes/{}", id);
	//	let response_string = fetch(&url).await?.text().await?;
	//	let response: TealTrack = from_str(&response_string).unwrap();

	//	Ok(response)
	// }
}

pub fn safe_get<'a>(hashmap: &'a HashMap<String, String>, key: &str) -> &'a str {
	if let Some(value) = hashmap.get(key) {
		value
	} else {
		""
	}
}
