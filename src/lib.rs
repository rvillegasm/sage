extern crate dirs;

mod packages;
mod repositories;

use packages::Package;
use repositories::Repo;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let repo = Repo::new("Arcanum", "https://raw.githubusercontent.com/rvillegasm/Arcanum/master/");

    let metadata = repo.get_program_metadata("Python")?;

    println!("{}", metadata);
    
    let mut pkg = Package::new(
        "Python",
        "3.8",
        "https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz",
    );

    pkg.download(&config.download_dir)?;

    Ok(())
}

/// Configuration data structure that holds
/// every system-wide variable regarding `sage`.
pub struct Config {
    download_dir: PathBuf,
    install_dir: PathBuf,
}

impl Config {
    /// Wraps the environment arguments as a `Config`
    /// data structure.
    pub fn new(_args: env::Args) -> Config {
        const DEFAULT_SAGE_HOME: &str = ".sage";
        let home: PathBuf = dirs::home_dir().unwrap();

        let sage_home = match env::var("SAGE_HOME") {
            Ok(p) => PathBuf::from(p),
            Err(_) => home.join(DEFAULT_SAGE_HOME),
        };

        let download_dir: PathBuf = sage_home.join("downloads");
        let install_dir: PathBuf = sage_home.join("bin");

        Config {
            download_dir,
            install_dir,
        }
    }

    /// Checks if the necessary directory strucutures for `sage`
    /// to work properly exist or not. If they don't, then it creates them.
    ///
    /// # Errors
    /// Check out the documentation for `std::fs::create_dir_all`
    /// to see the situations in which this function will return an error.
    /// The conditions in which an error will be thrown are the same as in said function.
    pub fn prepare_env(&self) -> Result<(), Box<dyn Error>> {
        // Create the download dir
        fs::create_dir_all(&self.download_dir)?;
        // Create the install dir
        fs::create_dir_all(&self.install_dir)?;

        Ok(())
    }
}
