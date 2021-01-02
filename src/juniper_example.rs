use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
    GraphQLEnum, Variables, GraphQLObject,
};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
struct Song {
	title: String,
	artist: String,
}

// Arbitrary context data.
struct Ctx(Episode);

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
	fn songs(&self) -> Vec<Song> {
		return vec![Song{ title: "The Distance".to_string(), artist: "Cake".to_string() }]
	}
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn main() {
    // Create a context object.
    let ctx = Ctx(Episode::Empire);

    // Run the executor.
    let (res, _errors) = juniper::execute_sync(
        "query { songs{title} }",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    ).unwrap();
	
	println!("{}",res)

}



// use juniper::{
// 	graphql_object, EmptySubscription, FieldResult, GraphQLEnum, 
// 	GraphQLInputObject, GraphQLObject, ScalarValue,
// };

// #[derive(GraphQLEnum)]
// enum Episode {
// 	NewHope,
// 	Empire,
// 	Jedi
// }

// #[derive(GraphQLObject)]
// #[graphql(description = "A humanoid creature")]
// struct Human {
// 	id: String,
// 	name: String,
// 	appears_in: Vec<Episode>,
// 	home_planet: String,
// }

// struct Context{
// 	pool: DatabasePool,
// }

// impl juniper::Context for Context {}

// struct Query;

// #[graphql_object(context=Context)]
// impl Query {
// 	fn apiVersion() -> &str {
// 		"1.0"
// 	}

// 	fn human(context: &Context, id: String) -> FieldResult<Human>{
// 		let connection = context.pool.get_connection()?;
// 		let human = connection.find_human(&id);
// 		Ok(human)
// 	}
// }


// fn main() {
// 	let context = Ctx(Episode::NewHope);

// 	let (res, errors) = juniper::execute_sync(
// 		"query{ favoriteEpisode }",
// 		None,
// 		&Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
// 		&Variables::new(),
// 		&ctx,
// 	).unwrap()

// 	println!(res)
// }
