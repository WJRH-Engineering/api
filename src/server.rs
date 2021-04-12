/// The HTTP server used to serve the API

use serde_json::from_str;
use std::fs::File;
use tide::Request;
use tide::Response;

use super::Schema;

use serde::Deserialize;

#[derive(Clone)]
pub struct State {
	schema: Schema,
}

#[derive(Deserialize)]
struct RequestBody {
	_operation_name: Option<String>,
	_variables: Option<std::collections::HashMap<String, String>>,
	query: String,
}

fn get_query(body_string: &str) -> String {
	// println!("{}", body_string);	
	let body: RequestBody = from_str(body_string).unwrap();
	format!("{}", &body.query)
}

mod routes {
	use super::*;

	fn error_page(code: u16, message: &str) -> Response {
		Response::builder(code)
			.body(message.to_owned())
			.header("content-type", "text/plain")
			.build()
	}

	pub async fn playground(request: Request<State>) -> tide::Result {
		let path = "graphql-playground.html";
		let playground_file = std::fs::read_to_string(path);
		if let Ok(file) = playground_file {
			let response = Response::builder(200)
				.body(file.to_string())
				.header("content-type", "text/html")
				.build();

			Ok(response)
		} else {
			Ok(error_page(404, "couldn't find playground.html"))
		}
	}

	pub async fn graphql(mut request: Request<State>) -> tide::Result {
		let schema = &request.state().schema.to_owned();

		let body = &request.body_string().await?;
		let query = get_query(&body);
		// println!("{:#?}", &query);

		let res = schema.execute(&query).await;
		let json = serde_json::to_string(&res).unwrap();

		Ok(Response::builder(200).body(json).header("content-type", "text/json").build())
	}
}

pub fn init(schema: Schema) -> tide::Server<State> {
	let state = State { schema };
	let mut server = tide::with_state(state);

	// assign each route to a function
	server.at("/").get(routes::playground);
	server.at("/").post(routes::graphql);

	return server
}
