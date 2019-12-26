extern crate reqwest;

use std::error::Error;

// TODO: Implement conection to remote repo and change the package download code

pub fn get_program_yml_config(repo_url: &str, program_name: &str) -> Result<(), Box<dyn Error>> {
    let target_url = repo_url + program_name;
    
    let response = reqwest::get()?;
}