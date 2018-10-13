use rs_config::*;
use std::collections::HashMap;
use std::path::Path;

/// Récupère la configuration du fichier `Config.toml`
/// Si un fichier `Config.local.toml` est trouvé, ses valeurs
/// écrasent celle de `Config.toml`
pub fn get() -> HashMap<String, String> {
  let mut settings = Config::default();
  settings.merge(File::with_name("Config.toml")).unwrap();
  if Path::new("Config.local.toml").exists() {
    settings
      .merge(File::with_name("Config.local.toml"))
      .unwrap();
  }
  let config = settings.try_into::<HashMap<String, String>>().unwrap();
  config
}
