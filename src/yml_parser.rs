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
}

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
}
