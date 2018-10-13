use rs_config::*;
use std::collections::HashMap;
use std::path::Path;

/// Récupère la configuration du fichier `App.toml`
/// Si un fichier `App.local.toml` est trouvé, ses valeurs
/// écrasent celle de `App.toml`
pub fn get() -> HashMap<String, String> {
  let mut settings = Config::default();
  settings.merge(File::with_name("App.toml")).unwrap();
  if Path::new("App.local.toml").exists() {
    settings.merge(File::with_name("App.local.toml")).unwrap();
  }
  let config = settings.try_into::<HashMap<String, String>>().unwrap();
  config
}
