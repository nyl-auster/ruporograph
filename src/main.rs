#![feature(plugin)]
#![feature(trace_macros)]
#![plugin(rocket_codegen)]
extern crate config;
extern crate postgres;
extern crate toml;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
mod database;
mod graphql;
mod helpers;
mod routes;
mod user;

fn main() {
  let ctx = helpers::ctx_init();

  // créer une table "users".
  database::uninstall(&ctx);
  database::install(&ctx);

  // lancer le serveur web
  rocket::ignite()
    .manage(ctx)
    .manage(graphql::build_schema())
    .mount(
      "/",
      routes![
        routes::post_graphql_handler,
        routes::get_graphql_handler,
        routes::graphiql,
      ],
    )
    .launch();
}
