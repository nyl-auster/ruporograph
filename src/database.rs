use helpers::Ctx;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::collections::HashMap;

/**
 * Permet de se connecter à notre base de données Postgres
 *
 * On utilise un "pool de connexion" qui pourra être partagé par
 * les différents threads au lieu d'une connexion unique ou d'ouvrir
 * une nouvelle connexion systématiquement.
 */
pub fn db_pool(
  config: &HashMap<String, String>,
) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
  let dabatabase_uri = config.get("database_uri").unwrap().as_str();
  println!(
    "🔍  Tentative de connexion à la base \"{}\" en cours ...",
    &dabatabase_uri
  );
  let manager = PostgresConnectionManager::new(dabatabase_uri, TlsMode::None).unwrap();
  let pool = r2d2::Pool::new(manager).expect("Impossible de se connecter à la base de données !");
  println!("🎉  Connexion à la base réussie !");
  pool
}

// créer la table users
pub fn install(ctx: &Ctx) {
  println!("✨  Création de la table users");
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
    .expect("Erreur en créant la table users");
}

// supprimer la table users
#[allow(dead_code)]
pub fn uninstall(ctx: &Ctx) {
  let connect = ctx.db_pool.get().unwrap();
  println!("🗑️   Suppression de la base de données users");
  connect
    .execute("DROP TABLE IF EXISTS users", &[])
    .expect("erreur en supprimant la table users");
}
