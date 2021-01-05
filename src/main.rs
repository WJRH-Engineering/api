 use juniper::{
	graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
	GraphQLEnum, Variables, GraphQLObject,
 };

use actix_web::{web, App, HttpResponse, get, post, HttpRequest, HttpServer, Responder};

use juniper_actix::graphql_handler;
use juniper_actix::playground_handler;

pub struct Query;

mod teal;
mod schedule;

type Schema = juniper::RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

fn schema() -> Schema {
	Schema::new(
		Query,
		EmptyMutation::new(),
		EmptySubscription::new(),
	)
}

#[get("/")]
async fn playground() -> impl Responder {
	playground_handler("/", None).await
}

#[post("/")]
async fn graphql(
	request: HttpRequest,
	payload: actix_web::web::Payload,
	schema: web::Data<Schema>,
) -> impl Responder {
	graphql_handler(&schema, &(), request, payload).await
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

	// just for testing
	// schedule::test_redis().await;

	// Ok(())
	

	 // start the actix web server
	 HttpServer::new(|| {
		App::new()
			.data(schema())
			.service(playground)
			.service(graphql)
	 }).bind("0.0.0.0:8000")?.run().await
}
