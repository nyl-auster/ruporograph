use config::*;
use std::collections::HashMap;
use std::path::Path;

pub fn get_config() -> HashMap<String, String> {
  let mut settings = Config::default();
  settings.merge(File::with_name("src/config.toml")).unwrap();
  if Path::new("src/config.local.toml").exists() {
    settings
      .merge(File::with_name("src/config.local.toml"))
      .unwrap();
  }
  let config = settings.try_into::<HashMap<String, String>>().unwrap();
  config
}

#[derive(Debug)]
pub struct Ctx {
  pub db_pool: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
  pub config: HashMap<String, String>,
}
// allow our Ctx to be a context for Juniper.
// It will be fully accessible from any field with  "executor.context()"
impl juniper::Context for Ctx {}

pub fn ctx_init() -> Ctx {
  let config = get_config();
  Ctx {
    db_pool: db_pool(&config),
    config: config,
  }
}
