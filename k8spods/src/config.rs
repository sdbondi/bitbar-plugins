extern crate toml;

use std::error::Error as BaseError;
use std::fs;
use std::fmt;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Config {
  pub namespaces: Vec<String>,
  #[serde(rename = "cluster-name")]
  pub cluster_name: String,
  #[serde(rename = "kubectl-path")]
  pub kubectl_path: String,
  #[serde(rename = "aws-iam-authenticator-path")]
  pub iam_authenticator_path: String,
}

impl Config {
  pub fn from_file(filename: String) -> Result<Config, Error> { // Teachable: can use Box<dyn Error> as the error type to return "any error"
    let contents = fs::read_to_string(filename)
        .map_err(|_e| Error("Unable to load config file"))?;

    toml::from_str(contents.as_str())
        .map_err(|_e| Error( "Unable to parse toml file"))
  }
}

#[derive(Debug)]
pub struct Error(&'static str);

impl BaseError for Error {
  fn description(&self) -> &str {
    self.0
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

