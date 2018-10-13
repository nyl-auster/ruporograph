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
mod app;
mod business;
mod graphql;
mod migrations;
mod router;

fn main() {
  let ctx = app::ctx_init();
  migrations::release_0::install::run(&ctx);
  rocket::ignite()
    .manage(ctx)
    .manage(graphql::build_schema())
    .mount("/", router::routes())
    .catch(router::catchers())
    .launch();
}
