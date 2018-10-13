use helpers::Ctx;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::collections::HashMap;

pub fn db_pool(
  config: &HashMap<String, String>,
) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
  let dabatabase_uri = config.get("database_uri").unwrap().as_str();
  let manager = PostgresConnectionManager::new(dabatabase_uri, TlsMode::None).unwrap();
  let pool = r2d2::Pool::new(manager).unwrap();
  pool
}

pub fn install(ctx: &Ctx) {
  println!("création de la table users");
  let connect = ctx.db_pool.get().unwrap();
  connect
    .execute(
      "CREATE TABLE users (
    id  SERIAL PRIMARY KEY,
    name  VARCHAR NOT NULL,
    mail  VARCHAR NOT NULL
    )",
      &[],
    )
    .expect("erreur en créant la table users");

  println!("création de la table test");
  let connect = ctx.db_pool.get().unwrap();
  connect
    .execute(
      "CREATE TABLE test (
    id  SERIAL PRIMARY KEY,
    name  VARCHAR NOT NULL
    )",
      &[],
    )
    .expect("erreur en créant la table test");
}

pub fn uninstall(ctx: &Ctx) {
  let connect = ctx.db_pool.get().unwrap();
  println!("suppression de la table users");
  connect
    .execute("DROP TABLE IF EXISTS users", &[])
    .expect("erreur en supprimant la table users");

  println!("suppression de la table test");
  connect
    .execute("DROP TABLE IF EXISTS test", &[])
    .expect("erreur en supprimant la table test");
}
