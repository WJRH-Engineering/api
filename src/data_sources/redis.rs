use redis;
use redis::Connection;
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

/// Opens a connection to the redis database and returns its.
/// It will attempt to read the `REDIS_URL` environment variable, either from
/// the environment or from a `.env` file. If it cannot find one, it defaults
/// to connecting to localhost `redis://127.0.0.1`
pub fn get_connection() -> redis::RedisResult<Connection> {
    dotenv().ok();
    let url = match env::var("REDIS_URL") {
        Ok(url) => url,
        Err(_error) => "redis://127.0.0.1".to_string(),
    };

	let client = redis::Client::open(url)?;
	let connection = client.get_connection()?;
	Ok(connection)
}

/// Syntactic sugar for a querying a redis database. Takes a command followed
/// by one or more arguments, and executes them against the database.
/// Returns a result that must be unwrapped
///
/// example: 
/// ```rust
/// query!("smembers", "programs")?;
/// query!("keys", "*").unwrap();
/// ```
macro_rules! query {
    ($cmd:expr, $($arg:expr),+)  => {{
        // connect to the database
        let mut connection = get_connection().unwrap();

        // execute the command with the given arguments
        redis::cmd($cmd)
            $( .arg( $arg ) )+
            .query(&mut connection)
            .unwrap() 
    }}
}

pub struct Program {
    pub shortname: String, 
	pub scalars: HashMap<String, String>,
	pub episode_ids: Vec<String>,
}

pub struct Episode {
    pub id: String, 
	pub scalars: HashMap<String, String>,
}

impl Program {

    pub fn scalar(&self, key: &str) -> String {
        match self.scalars.get(key) {
            Some(value) => value,
            None => ""
        }.to_string()
    }

    pub fn optional_scalar(&self, key: &str) -> Option<String> {
        let value = self.scalars.get(key)?.to_string();
        Some(value)
    }

    /// Queries the redis database for information about a given program and
    /// returns it 
    pub fn get(shortname: &str) -> Program {
        
        let scalars_key = format!("programs:{shortname}:scalars", shortname = shortname); 
        let episodes_key = format!("programs:{shortname}:episodes", shortname = shortname); 

        let scalars: HashMap<String, String> = query!("hgetall", scalars_key);
        let episode_ids: Vec<String> = query!("zrange", episodes_key, "0", "-1");

        Program {
            shortname: shortname.to_string(),
            scalars,
            episode_ids,
        }
    }

    /// runs Program::get() for each program in the database and returns them
    /// as a Vector
    pub fn get_all() -> Vec<Program> {
        let names: Vec<String> = query!("smembers", "programs");
        return names.iter()
            .map(|shortname| Program::get(shortname))
            .collect()
    }
}

impl Episode {
    pub fn get(id: &str) -> Episode {
        let scalars: HashMap<String, String> = query!(
            "hgetall",
            format!("episodes:{id}:scalars", id = id)
        );
        Episode { id: id.to_string(), scalars }
    }

    pub fn scalar(&self, key: &str) -> String {
        match self.scalars.get(key) {
            Some(value) => value,
            None => ""
        }.to_string()
    }

    pub fn optional_scalar(&self, key: &str) -> Option<String> {
        let value = self.scalars.get(key)?.to_string();
        Some(value)
    }

}

 #[cfg(test)]
 mod tests {
	use super::*;

	#[test]	
	fn test_macro() {
        let res: Vec<String> = query!("keys", "test");
	} 

	#[test]	
	fn program_get_all() {
        Program::get_all();
	} 

 }
