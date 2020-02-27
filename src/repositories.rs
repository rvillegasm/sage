extern crate reqwest;

use std::error::Error;

/// A valid repository representation for **sage**
pub struct Repo {
    name: String,
    base_url: String,
    host_os: String,
}

impl Repo {
    /// Creates a new `Repo` from two string slices
    pub fn new(name: &str, base_url: &str, host_os: &str) -> Repo {
        Repo {
            name: String::from(name),
            base_url: String::from(base_url),
            host_os: String::from(host_os),
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

    /// Returns the name of the host os in which sage is running
    pub fn get_host_os(&self) -> &str {
        &self.host_os
    }

    /// Returns a Yaml-formatted string holding the metadata of a program.
    ///
    /// # Errors
    /// Check out the documentation for reqwest::get the see the conditions in which
    /// this method could return an error.
    /// It also fails when the repo is not valid.
    pub fn get_program_metadata(&self, program_name: &str) -> Result<String, Box<dyn Error>> {
        let target_url = match self.get_name() {
            "Arcanum" => format!(
                "{}/{}/{}/metadata.yml",
                self.get_base_url(),
                self.get_host_os(),
                program_name
            ),

            _ => panic!("The specified repository is not supported!"),
        };

        let mut response = reqwest::get(&target_url)?;

        Ok(response.text()?)
    }

    /// Returns a Yaml-formatted string holding the package info of a program.
    ///
    /// # Errors
    /// Check out the documentation for reqwest::get the see the conditions in which
    /// this method could return an error.
    /// It also fails when the repo is not valid.
    pub fn get_program_package(
        &self,
        program_name: &str,
        program_version: &str,
    ) -> Result<String, Box<dyn Error>> {
        let target_url = match self.get_name() {
            "Arcanum" => format!(
                "{}/{}/{}/{}_{}.yml",
                self.get_base_url(),
                self.get_host_os(),
                program_name,
                program_name,
                program_version
            ),

            _ => panic!("The specified repository is not supported!"),
        };

        let mut respone = reqwest::get(&target_url)?;

        Ok(respone.text()?)
    }
}
