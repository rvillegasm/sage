extern crate dirs;

mod errors;
mod packages;
mod repositories;
mod yml_parser;

use errors::{NoVersionFoundError, NoVersionSpecifiedError, PackageNotFoundError};
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
        // get information about every version of said program
        "info" => generic_info(&repo, &config.desired_pkg),
        // get info about a specific version of the program
        "details" => {
            // Check if the user specified a version or not
            let version = match &config.desired_pkg_version {
                Some(string) => string,
                None => return Err(Box::new(NoVersionSpecifiedError)),
            };
            specific_info(&repo, &config.desired_pkg, version, true)?;
            Ok(())
        }
        // just download a version of the program
        "download" => {
            let version = match &config.desired_pkg_version {
                Some(string) => string,
                None => return Err(Box::new(NoVersionSpecifiedError)),
            };
            let name = &config.desired_pkg;
            // Create the package
            let mut pkg = specific_info(&repo, name, version, false)?;
            pkg.download(&config.download_dir)
        }
        // This will never happen, it's just here to exaust the match options
        _ => panic!("Could not match on the specified command"),
    }
}

/// Gets the genral information about a package, like name, available versions
/// lts version and latest version
///
/// # Errors
/// The function will return an error if the remote package doesn't exists.
/// Check out the documentation for `Repo::get_program_metadata` and `MetadataParser::new`
/// to find out other reasons for this function to fail.
fn generic_info(repo: &Repo, program_name: &str) -> Result<(), Box<dyn Error>> {
    // Get the metadata parser and parse it
    let metadata = repo.get_program_metadata(program_name)?;
    let parser = MetadataParser::new(&metadata)?;

    // Calculates the versions
    let versions = match parser.get_versions() {
        Some(vec) => vec,
        None => return Err(Box::new(NoVersionFoundError)),
    };

    println!("Package: {}", program_name);
    println!("Available versions: ");
    // And print them
    for i in 0..versions.len() {
        println!(
            "  - {}",
            versions[i]
                .as_str()
                .expect("Error trying to access the versions of the package")
        );
    }
    // The unwraps here are because, for the moment,
    // a package will always have an LTS and a Latest value
    println!("LTS Version: {}", parser.get_lts_version().unwrap());
    println!("Latest Version: {}", parser.get_latest_version().unwrap());

    Ok(())
}

/// Gets the specific information about a package, constructing and returning one
///
/// # Errors
/// The function will return an error if the package doesen't exists.
/// Check out the documentation for `Repo::get_program_package` and `PackageParser::new`
/// to find out other reasons for this function to fail.
fn specific_info(
    repo: &Repo,
    program_name: &str,
    program_version: &str,
    print_out_info: bool,
) -> Result<Package, Box<dyn Error>> {
    // get the package data and parse it
    let pkg_data = repo.get_program_package(program_name, program_version)?;
    let parser = PackageParser::new(&pkg_data)?;

    let pkg_name = match parser.get_name() {
        Some(name) => name,
        None => return Err(Box::new(PackageNotFoundError)),
    };

    // With how the repo (Arcanum) is designed, these unwraps will never fail
    let pkg_version = parser.get_version().unwrap();
    let pkg_url = parser.get_ulr().unwrap();
    let pkg_type = parser.get_file_type().unwrap();
    let pkg_file = parser.get_file_name().unwrap();

    if print_out_info {
        println!("Package: {}", pkg_name);
        println!("Version: {}", pkg_version);
        println!("Download Url: {}", pkg_url);
        println!("File Type: {}", pkg_type);
        println!("File Name: {}", pkg_file);
    }

    Ok(Package::new(pkg_name, pkg_version, pkg_url))
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
    const COMMANDS_QUANTITY: usize = 3; // NUMBER OF COMMANDS
    let mut command_set: HashSet<&str> = HashSet::with_capacity(COMMANDS_QUANTITY);
    command_set.insert("info");
    command_set.insert("details");
    command_set.insert("download");

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
