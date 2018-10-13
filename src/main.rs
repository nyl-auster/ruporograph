#![feature(plugin)]
#![feature(trace_macros)]
#![plugin(rocket_codegen)]
extern crate config as rs_config; // alias, sinon conflit avec notre "mod config;"
extern crate postgres;
extern crate toml;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
mod config;
mod ctx;
mod database;
mod graphql;
mod routes;
mod user;

fn main() {
  let config = config::get();
  let ctx = ctx::Ctx {
    db_pool: database::db_pool(&config),
    config: config,
  };

  // créer une table "users" de démo.
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
