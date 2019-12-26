extern crate reqwest;

use std::error::Error;

/// A valid repository representation for **sage**
pub struct Repo {
    name: String,
    base_url: String,
}

impl Repo {
    /// Creates a new `Repo` from two string slices
    pub fn new(name: &str, base_url: &str) -> Repo {
        Repo {
            name: String::from(name),
            base_url: String::from(base_url),
        }
    }

    /// Returns the internal *name* field of the `Repo`
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the internal *base_url* field of the `Repo`
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_program_metadata(&self, program_name: &str) -> Result<String, Box<dyn Error>> {
        let target_url = match self.get_name() {
            "Arcanum" => format!("{}/{}/metadata.yml", self.get_base_url(), program_name),

            _ => panic!("The specified repository is not supported!"),
        };

        let mut response = reqwest::get(&target_url)?;

        Ok(response.text()?)
    }
}
