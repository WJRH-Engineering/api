 use juniper::{
	graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
	GraphQLEnum, Variables, GraphQLObject,
 };

 pub struct Query;
 mod schedule;

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
// pub type Schema = juniper::RootNode<'static, Query, EmptyMutation, EmptySubscription>;

type Schema = juniper::RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

async fn run() {
	let (res, errors) = juniper::execute(
		"query {schedule{ start }}",
		None,
		&Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
		&Variables::new(),
		&()
	).await.unwrap();

	println!("----");
	println!("{}", res);
	println!("----");
}

use reqwest::get;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {

	let result = get("");

	run().await;
	Ok(())
}
