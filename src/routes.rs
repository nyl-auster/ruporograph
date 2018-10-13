// HttpRequest -> Graphql Type
use ctx::Ctx;
use graphql::Schema;
use rocket::response::content;
use rocket::State;

#[get("/")]
pub fn graphiql() -> content::Html<String> {
  juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
  context: State<Ctx>,
  request: juniper_rocket::GraphQLRequest,
  schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
  context: State<Ctx>,
  request: juniper_rocket::GraphQLRequest,
  schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
  request.execute(&schema, &context)
}
