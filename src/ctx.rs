use std::collections::HashMap;

#[derive(Debug)]
pub struct Ctx {
  pub db_pool: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
  pub config: HashMap<String, String>,
}

impl Ctx {
  // pour pouvoir récupérer une connexion en faisant simplement "ctx.db_connect()"
  pub fn db_connect(&self) -> r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager> {
    self.db_pool.get().unwrap()
  }
}

// permettre à notre structure Ctx d'être un context valide
// pour Juniper ( la librairie GraphQL).
// Notre contexte sera alors accessible depuis
// 'nimporte quel resolver avec  "executor.context()"
impl juniper::Context for Ctx {}
