use config::*;
use database::db_pool;
use std::collections::HashMap;

/// on récupère notre configuration depuis le fichier
/// `src/config.toml`. Si on
pub fn get_config() -> HashMap<String, String> {
  let mut settings = Config::default();
  settings.merge(File::with_name("src/config.toml")).unwrap();
  let config = settings.try_into::<HashMap<String, String>>().unwrap();
  config
}

#[derive(Debug)]
pub struct Ctx {
  pub db_pool: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
  pub config: HashMap<String, String>,
}
// permettre à notre structure Ctx d'être un context valide
// pour Juniper ( la librairie GraphQL).
// Notre contexte sera alors accessible depuis
// 'nimporte quel resolver avec  "executor.context()"
impl juniper::Context for Ctx {}

impl Ctx {
  // helper to get a connection to dabatabase from our pool
  pub fn db_connect(&self) -> r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager> {
    self.db_pool.get().unwrap()
  }
}

// Initialiser le contexte de notre application
pub fn ctx_init() -> Ctx {
  let config = get_config();
  Ctx {
    db_pool: db_pool(&config),
    config: config,
  }
}
