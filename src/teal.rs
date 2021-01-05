use juniper::{graphql_object, GraphQLObject};
use sqlx::postgres::PgPoolOptions;
use serde::Deserialize;
use serde_json;
use redis;
use std::collections::HashMap;

pub struct TealProgram {
	pub scalars: HashMap<String, String>,
	pub episode_ids: Vec<String>,
}

impl TealProgram {
	pub fn get_from_redis(key: &str, connection: &mut redis::Connection) -> redis::RedisResult<Self> {

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
			output.push(TealProgram::get_from_redis(&program, &mut connection).unwrap());	
		}

		output
	}
}

pub fn safe_get<'a>(hashmap: &'a HashMap<String, String>, key: &str) -> &'a str {
	if let Some(value) = hashmap.get(key) {
		value
	} else {
		""
	}
}
