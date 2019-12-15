extern crate dirs;

mod packages;
use packages::Package;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut pkg = Package::new(
        "Python",
        "3.8",
        "https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz",
    );

    pkg.download(&config.download_dir);

    Ok(())
}

pub struct Config {
    download_dir: PathBuf,
    install_dir: PathBuf,
}

impl Config {
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

    pub fn prepare_env(&self) -> Result<(), Box<dyn Error>> {
        // Create the download dir
        println!(
            "Creating directory structure: {} ...",
            self.download_dir
                .to_str()
                .expect("Download dir is not a valid utf-8 encoded value")
        );
        fs::create_dir_all(&self.download_dir)?;
        println!("Structure succesfully created");
        // Create the install dir
        println!(
            "Creating directory structure: {} ...",
            self.install_dir
                .to_str()
                .expect("Install dir is not a valid utf-8 encoded value")
        );
        fs::create_dir_all(&self.install_dir)?;
        println!("Structure succesfully created");

        Ok(())
    }
}
