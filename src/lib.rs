extern crate dirs;

mod packages;
mod repositories;
mod yml_parser;

use packages::Package;
use repositories::Repo;
use yml_parser::{MetadataParser, PackageParser};

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let repo = Repo::new(
        "Arcanum",
        "https://raw.githubusercontent.com/rvillegasm/Arcanum/master/",
        "linux",
    );

    // Decide which command to use
    let command: &str = config.command.as_ref();
    match command {
        "info" => generic_info(&repo, &config.desired_pkg),
        "details" => {
            // TODO: do a better job with the error handling, maybe create a custom error type
            let version = &config
                .desired_pkg_version
                .expect("A version of the package is needed to give details about it");
            specific_info(&repo, &config.desired_pkg, version)
        }
        _ => panic!("Could not match on the specified command"),
    }

    // generic info for when the user doesn't sepcifies the version
    // generic_info(repo, "Python")

    // Info for when the user specifies a version of the package
    // specific_info(&repo, "Python", "3.8.0")

    // let mut pkg = Package::new(
    //     "Python",
    //     "3.8",
    //     "https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz",
    // );

    // pkg.download(&config.download_dir)?;

    // Ok(())
}

fn generic_info(repo: &Repo, program_name: &str) -> Result<(), Box<dyn Error>> {
    // Get the metadata parser and parse it
    let metadata = repo.get_program_metadata(program_name)?;
    let parser = MetadataParser::new(&metadata)?;
    println!("Package: {}", program_name);
    println!("Available versions: ");
    // Calculates the versions
    let versions = match parser.get_versions() {
        Some(vec) => vec,
        None => panic!("No versions could be found!"), // TODO: do a better job with the error handling, maybe create a custom error type
    };
    // And print them
    for i in 0..versions.len() {
        println!(
            "  - {}",
            versions[i]
                .as_str()
                .expect("Error trying to access the versions of the package")
        );
    }
    println!("LTS Version: {}", parser.get_lts_version().unwrap()); // TODO: do a better job with the error handling, maybe create a custom error type
    println!("Latest Version: {}", parser.get_latest_version().unwrap()); // TODO: do a better job with the error handling, maybe create a custom error type

    Ok(())
}

fn specific_info(
    repo: &Repo,
    program_name: &str,
    program_version: &str,
) -> Result<(), Box<dyn Error>> {
    // get the package data and parse it
    let pkg_data = repo.get_program_package(program_name, program_version)?;
    let parser = PackageParser::new(&pkg_data)?;

    println!("Package: {}", parser.get_name().unwrap()); // TODO: do a better job with the error handling, maybe create a custom error type
    println!("Version: {}", parser.get_version().unwrap());
    println!("Download Url: {}", parser.get_ulr().unwrap());
    println!("File Type: {}", parser.get_file_type().unwrap());

    Ok(())
}

/// Configuration data structure that holds
/// every system-wide variable regarding `sage`.
pub struct Config {
    download_dir: PathBuf,
    install_dir: PathBuf,
    command: String,
    desired_pkg: String,
    desired_pkg_version: Option<String>,
}

// Config helper functions

/// Checks if the specified command is supported by sage or not
fn parse_commands(command: String) -> Result<String, &'static str> {
    // A set containing all the valid commands
    const COMMANDS_QUANTITY: usize = 2;
    let mut command_set: HashSet<&str> = HashSet::with_capacity(COMMANDS_QUANTITY);
    command_set.insert("info");
    command_set.insert("details");

    // Check if the specified command is valid
    let command_str: &str = command.as_ref();
    if !command_set.contains(command_str) {
        return Err("The specified command is not valid");
    } else {
        Ok(command)
    }
}

/// Checks if the user specified a version with the package or not
fn parse_desired_pkg(pkg: String) -> (String, Option<String>) {
    // Check if the package contains an '@'
    // to separate the name from the version
    let (name, version) = match pkg.find('@') {
        Some(index) => {
            let name = String::from(&pkg[..index]);
            let version = String::from(&pkg[index + 1..]);
            (Some(name), Some(version))
        }
        None => (Some(pkg), None),
    };

    if version == None {
        return (name.unwrap(), None);
    } else {
        (name.unwrap(), version)
    }
}

impl Config {
    /// Wraps the environment arguments as a `Config`
    /// data structure.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // ignore the name with which sage was called by the user
        args.next();

        // Command parsing
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("No command was specified"),
        };
        let command = parse_commands(command)?;

        // Package parsing (with optional version)
        let desired_pkg = match args.next() {
            Some(arg) => arg,
            None => return Err("No package was specified"),
        };
        let (desired_pkg, desired_pkg_version) = parse_desired_pkg(desired_pkg);

        // Default directory in which the packages will be
        // downloaded and installed
        const DEFAULT_SAGE_HOME: &str = ".sage";
        let home: PathBuf = dirs::home_dir().unwrap();

        let sage_home = match env::var("SAGE_HOME") {
            Ok(p) => PathBuf::from(p),
            Err(_) => home.join(DEFAULT_SAGE_HOME),
        };

        let download_dir: PathBuf = sage_home.join("downloads");
        let install_dir: PathBuf = sage_home.join("bin");

        Ok(Config {
            download_dir,
            install_dir,
            command,
            desired_pkg,
            desired_pkg_version,
        })
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
