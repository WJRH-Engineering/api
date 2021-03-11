
use async_graphql::*;
use async_std;

mod teal;
mod lastfm;
mod schedule;

use serde_json::from_str;

mod schema;

use schema::Query;

//server

use std::fs::File;



use tide::Request;
use tide::Response;
use tide::prelude::*;

// async fn server(request: Request, schema: MySchema) {
// 	router!{ request,
// 		(GET) (/) => { playground() },
// 		(POST) (/) => { graphql(&request, schema).await },

// 		_ => { Response::empty_404() }
// 	}	
// }

async fn playground(request: Request<()>) -> tide::Result {
	match std::fs::read_to_string("graphql-playground.html") {
		Ok(file) => {
			let response = Response::builder(200)
				.body(file.to_string())
				.header("content-type", "text/html")
				.build();

			Ok(response)
		},
		Err(error) => {
			println!("error reading index.html:");
			println!("{:#?}", error);
			let response = Response::builder(404)
				.body("oh no a 404 error!")
				.header("content-type", "text/plain")
				.build();

			Ok(response)
		}
	}
}

// "{\"operationName\":null,\"variables\":{},\"query\":\"mutation {\\n  schedule {\\n    shortname\\n  }\\n}\\n\"}"

#[derive(Deserialize)]
struct RequestBody {
	operation_name: Option<String>,
	variables: Option<std::collections::HashMap<String, String>>,
	query: String,
}

fn get_query(body_string: &str) -> String {
	println!("{}", body_string);	
	let body: RequestBody = from_str(body_string).unwrap();
	format!("{}", &body.query)
}


type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;
async fn graphql(mut request: Request<()>) -> tide::Result {
	let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
	let body = request.body_string().await?;
	println!("{:?}", &body);
	let query = get_query(&body);

	let res = schema.execute(&query).await;
	// let res = schema.execute("query{ schedule {shortname} }").await;
	let json = serde_json::to_string(&res).unwrap();
	Ok(Response::builder(200).body(json).header("content-type", "text/json").build())
}


#[async_std::main]
async fn main() {
	
	// let body_string = "{\"operationName\":null,\"variables\":{},\"query\":\"mutation {\\n  schedule {\\n    shortname\\n  }\\n}\\n\"}";
	// println!("{}", get_query(&body_string));
	// let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
	// let res = schema.execute("query{ schedule {shortname} }").await;
	// let json = serde_json::to_string(&res).unwrap();
	// println!("{:#?}", json);

	let mut app = tide::new();
	app.at("/").get(playground);
	app.at("/").post(move |request| {
		graphql(request)
	});
	app.listen("0.0.0.0:2000").await;
}

// old stuff

 // use juniper::{
	// graphql_object, EmptyMutation, FieldResult, 
	// GraphQLEnum, Variables, GraphQLObject,
 // };

// use actix_web::{web, App, HttpResponse, get, post, HttpRequest, HttpServer, Responder};

// use juniper_actix::graphql_handler;
// use juniper_actix::playground_handler;

// pub struct Query;

// mod schema;
// mod teal;
// mod lastfm;
// mod schedule;

// type Schema = juniper::RootNode<'static, Query, EmptyMutation<()>>;

// fn schema() -> Schema {
	// Schema::new(
		// Query,
		// EmptyMutation::new(),
		// EmptySubscription::new(),
	// )
// }

// #[get("/")]
// async fn playground() -> impl Responder {
	// playground_handler("/", None).await
// }

// #[post("/")]
// async fn graphql(
	// request: HttpRequest,
	// payload: actix_web::web::Payload,
	// schema: web::Data<Schema>,
// ) -> impl Responder {
	// graphql_handler(&schema, &(), request, payload).await
// }


// #[actix_web::main]
// async fn main() -> std::io::Result<()>{

	// // just for testing
	// // schedule::test_redis().await;

	// // Ok(())
	

	 // // start the actix web server
	 // HttpServer::new(|| {
		// App::new()
			// .data(schema())
			// .service(playground)
			// .service(graphql)
	 // }).bind("0.0.0.0:8000")?.run().await
// }
