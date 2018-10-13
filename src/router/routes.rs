// HttpRequest -> Graphql Type
use app::Ctx;
use graphql::Schema;
use rocket::response::content;
use rocket::State;

#[catch(500)]
pub fn internal_error() -> &'static str {
  "Une erreur est survenue sur le serveur."
}

#[catch(404)]
pub fn not_found(req: &rocket::Request) -> String {
  format!("La page '{}' n'a pas été trouvée", req.uri())
}

#[get("/")]
pub fn index() -> String {
  format!("Hello")
}

#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
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
