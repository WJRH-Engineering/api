mod data_sources;
mod graphql_types;
mod schema;
mod server;

use dotenv::dotenv;
use std::env;

use async_graphql::{EmptyMutation, EmptySubscription};
use async_std;
use serde_json::from_str;

use schema::Query;
type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

macro_rules! get_schema {
    ($query:ident) => {
        Schema::new(
           $query::default(),
           EmptyMutation,
           EmptySubscription,
        );
   }
}

#[async_std::main]
async fn main() {

    // load extra settings from the .env file
	println!("Loading environment variables");
	dotenv().ok();

    // build the schema
	let schema = Schema::new(
		Query::default(),
		EmptyMutation,
		EmptySubscription
	);

    // start the server
    println!("Listening on port 2000");
	let app = server::init(schema);
	app.listen("0.0.0.0:2000").await;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn lookup_song(){
        let schema = get_schema!(Query);
        let res = schema.execute("{
            lookupSong(artist: \"hilltop hoods\", title: \"leave me lonely\") {
                wiki
            }
        }").await;
        let json = serde_json::to_string_pretty(&res).unwrap();
        println!("{}", json);
        // panic!();
   } 
    
    #[async_std::test]
    async fn query(){
        let schema = get_schema!(Query);
        let res = schema.execute("{
            programs(limit: 5) {
                name
                episodes {
                    name
                }
            }
        }").await;
        let json = serde_json::to_string_pretty(&res).unwrap();
        println!("{}", json);
        // panic!();
   } 
}
