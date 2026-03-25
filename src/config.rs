use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub plugs: Option<Vec<ConfigPlug>>,
  pub hubs: Option<Vec<ConfigHub>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigPlug {
  pub name: String,
  pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigHub {
  pub name: String,
  pub host: String,
}
