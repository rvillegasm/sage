extern crate yaml_rust;

use yaml_rust::scanner::ScanError;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

// TODO: Look for a different solution other than .clone() for creating a Parser

/// A parser for reading metadata information about *Arcanum* packages
pub struct MetadataParser {
    doc: Yaml,
}

/// A parser for reading package information about *Arcanum* packages
pub struct PackageParser {
    doc: Yaml,
}

/// Enum containing every available installation detail that a package
/// can have in *Arcanum*
pub enum InstallInfo {
    Type, // for now, the installation type is the only one available
}

// --------- IMPLEMENTATIONS ---------

impl MetadataParser {
    /// Creates a new `MetadataParser`, instantly parsing the contents of a
    /// yaml-formatted string
    pub fn new(str: &str) -> Result<MetadataParser, ScanError> {
        let docs = YamlLoader::load_from_str(str)?;
        let doc = docs[0].clone();

        Ok(MetadataParser { doc })
    }

    /// Returns a vector containing the versions of the package
    pub fn get_versions(&self) -> Option<&Vec<Yaml>> {
        let versions = &self.doc["versions"];
        versions.as_vec()
    }

    /// Returns the long term support version of the package
    pub fn get_lts_version(&self) -> Option<&str> {
        let lts_version = &self.doc["lts"];
        lts_version.as_str()
    }

    /// Returns the latest version of the package
    pub fn get_latest_version(&self) -> Option<&str> {
        let latest_version = &self.doc["latest"];
        latest_version.as_str()
    }
}

impl PackageParser {
    /// Creates a new `PackageParser`, instantly parsing the contents of a
    /// yaml-formatted string
    pub fn new(str: &str) -> Result<PackageParser, ScanError> {
        let docs = YamlLoader::load_from_str(str)?;
        let doc = docs[0].clone();

        Ok(PackageParser { doc })
    }

    /// Returns the name of the package
    pub fn get_name(&self) -> Option<&str> {
        let name = &self.doc["name"];
        name.as_str()
    }

    /// Returns the version of the package
    pub fn get_version(&self) -> Option<&str> {
        let version = &self.doc["version"];
        version.as_str()
    }

    /// Returns the url of the package
    pub fn get_ulr(&self) -> Option<&str> {
        let url = &self.doc["url"];
        url.as_str()
    }

    /// Returns the file type of the package
    pub fn get_file_type(&self) -> Option<&str> {
        let file_type = &self.doc["type"];
        file_type.as_str()
    }

    /// Returns the name of the file to be downloaded
    pub fn get_file_name(&self) -> Option<&str> {
        let file = &self.doc["file"];
        file.as_str()
    }

    /// Analyzes the install info and returns the desired specific info
    pub fn get_installation_info(&self, desired_info: InstallInfo) -> Option<&str> {
        let install_details = &self.doc["installation"];
        // decide waht to do based on what the desireed info is
        let info = match desired_info {
            // the install type has been requested
            InstallInfo::Type => install_details["type"].as_str(),
        };
        // return the info
        info
    }
}

// -----------------------
//       UNIT TESTS
// -----------------------

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_YML: &str = "
    versions:
        - 3.8.0
        - 3.7.0
    lts: 3.8.0
    latest: 3.8.0
    ";

    static TEST_PACKAGE: &str = "
    name: Python
    version: 3.8.0
    url: https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz
    type: tar.xz
    file: Python-3.8.0.tar.xz
    installation:
        type: make
    ";

    #[test]
    fn meta_parser_get_versions() {
        let meta_parser = match MetadataParser::new(TEST_YML) {
            Ok(parser) => parser,
            Err(_) => panic!("Could not create a metadata parser with the given string"),
        };
        let versions = meta_parser.get_versions().unwrap();
        // Check out what versions actually holds by running cargo test -- --nocapture
        println!(":: versions: {:?}", versions);
        println!(":: first version:  {}", versions[0].as_str().unwrap());

        assert_eq!(
            versions,
            &vec![
                yaml_rust::Yaml::String(String::from("3.8.0")),
                yaml_rust::Yaml::String(String::from("3.7.0"))
            ]
        );
        assert_eq!(versions[0].as_str().unwrap(), "3.8.0");
    }

    #[test]
    fn meta_parser_get_lts() {
        let meta_parser = MetadataParser::new(TEST_YML)
            .expect("Could not create a metadata parser with the given string");
        let lts_version = meta_parser.get_lts_version().unwrap();
        println!(":: LTS Version: {}", lts_version);

        assert_eq!(lts_version, "3.8.0");
    }

    #[test]
    fn meta_parser_get_latest() {
        let meta_parser = MetadataParser::new(TEST_YML)
            .expect("Could not create a metadata parser with the given string");
        let latest_version = meta_parser.get_latest_version().unwrap();
        println!(":: Latest Version: {}", latest_version);

        assert_eq!(latest_version, "3.8.0");
    }

    #[test]
    fn pack_parser_get_everything() {
        let pack_parser = PackageParser::new(TEST_PACKAGE)
            .expect("Could not create a package parser with the given string");
        let name = pack_parser.get_name().unwrap();
        let version = pack_parser.get_version().unwrap();
        let url = pack_parser.get_ulr().unwrap();
        let f_type = pack_parser.get_file_type().unwrap();
        let file = pack_parser.get_file_name().unwrap();
        let install_type = pack_parser
            .get_installation_info(InstallInfo::Type)
            .unwrap();

        assert_eq!(name, "Python");
        assert_eq!(version, "3.8.0");
        assert_eq!(
            url,
            "https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz"
        );
        assert_eq!(f_type, "tar.xz");
        assert_eq!(file, "Python-3.8.0.tar.xz");
        assert_eq!(install_type, "make");
    }
}
