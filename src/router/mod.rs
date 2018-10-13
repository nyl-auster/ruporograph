pub mod routes;
use rocket;

pub fn routes() -> Vec<rocket::Route> {
  routes![
    routes::index,
    routes::graphiql,
    routes::get_graphql_handler,
    routes::post_graphql_handler
  ]
}

pub fn catchers() -> Vec<rocket::Catcher> {
  catchers![routes::not_found, routes::internal_error]
}
