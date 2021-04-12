mod data_sources;
mod graphql_types;
mod schema;
mod server;

use dotenv::dotenv;
use std::env;

use async_graphql::{EmptyMutation, EmptySubscription};
use async_std;

use schema::Query;
type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

/// Builds a GraphQL schema by combining a Query, Mutation, and Subscription.
///
/// TODO: If non-empty mutations and subscriptions are to be used in the future,
/// that functionality needs to be added to this macro first
macro_rules! build_schema {
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

    // load settings from the .env file
	dotenv().ok();

    let schema = build_schema!(Query);

    // read bind address and port settings for the http server, falling back
    // to sensible defaults if they are not set
    let address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("2000".to_string());
    let url = format!("{}:{}", address, port);

    // start the server and wait for requests
	let app = server::init(schema);
    println!("listening on {}", &url); 
	let _ = app.listen(url).await;
}




// ----------
// UNIT TESTS
// ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn lookup_song(){
        let schema = build_schema!(Query);
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
        let schema = build_schema!(Query);
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
