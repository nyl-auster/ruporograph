use rs_config::*;
use std::collections::HashMap;
use std::path::Path;

/// Récupèrer la configuration depuis le fichier `App.toml`
/// pour la configuration par défaut. Si un fichier `App.local.toml`
/// est trouvé, ses valeurs écraserons celles par défault
pub fn get() -> HashMap<String, String> {
  let mut settings = Config::default();
  settings.merge(File::with_name("App.toml")).unwrap();
  if Path::new("App.local.toml").exists() {
    settings.merge(File::with_name("App.local.toml")).unwrap();
  }
  let config = settings.try_into::<HashMap<String, String>>().unwrap();
  config
}
