extern crate indicatif;
extern crate reqwest;

use crate::errors::{
    DecoderNotFoundError, FileTypeNotSupportedError, InstallTypeNotSupportedError,
    PathNotFoundError,
};
use crate::install_utils::{CommandRunner, Decoder, FileTypes, InstallTypes};

use std::error::Error;
use std::fs::File;
use std::path::Path;

/// Representation of a valid `sage` package.
pub struct Package {
    name: String,
    version: String,
    url: String,
    file_type: FileTypes,
    file: String,
    install_type: InstallTypes,
    install_target: String,
}

// Public API for Package
impl Package {
    /// Wraps the given name, version and download url
    /// (the three of them string slices) as a  `Package`.
    pub fn new(
        name: &str,
        version: &str,
        url: &str,
        file_type: &str,
        file_name: &str,
        install_type: &str,
        install_target: &str,
    ) -> Result<Package, Box<dyn Error>> {
        // Create the FileType instance depending on the given string
        let file_type_enum = match file_type {
            "tar.xz" => FileTypes::TarXz,
            "tar.gz" => FileTypes::TarGz,
            _ => return Err(Box::new(FileTypeNotSupportedError)),
        };
        // Create the InstallTypes instance depending on the given string
        let install_type_enum = match install_type {
            "make" => InstallTypes::MakeInstall,
            _ => return Err(Box::new(InstallTypeNotSupportedError)),
        };
        Ok(Package {
            name: name.to_string(),
            version: version.to_string(),
            url: url.to_string(),
            file_type: file_type_enum,
            file: file_name.to_string(),
            install_type: install_type_enum,
            install_target: install_target.to_string(),
        })
    }

    /// Downloads the `Package` to the specified download directory.
    ///
    /// # Errors
    /// Check out the documentation for `reqwest::get`,
    /// the `copy_to` method of `reqwest::Response`
    /// and the `create` method of `std::fs::File` to see
    /// the conditions in which this function could return an error.
    pub fn download(&self, download_dir: &Path) -> Result<(), Box<dyn Error>> {
        // Configure the Progress bar
        let pb = self.start_download_progress();
        // Specify the target url
        let target = self.url.as_str();
        // Make the request
        let mut response = reqwest::get(target)?;

        // Create the destination file
        let mut dest_file = {
            let fname = &self.file;
            // create a PathBuf instance containing
            // the full path to the downloaded file
            let fname = download_dir.join(fname);
            // Create the dest file
            File::create(&fname)?
        };
        // Copy the file from the response to the destination
        response.copy_to(&mut dest_file)?;

        // Signal the progress bar to end
        self.finish_download_progress(pb);

        Ok(())
    }

    /// Install the 'Package' in the system.
    pub fn install<D: Decoder>(
        &self,
        download_dir: &Path,
        sage_home_path: Option<&str>,
        decoder: Option<D>,
    ) -> Result<(), Box<dyn Error>> {
        // Start the install progress bar
        let pb = self.start_install_progress();
        // analyze the type of the DOWNLOADED file
        match self.file_type {
            FileTypes::TarGz | FileTypes::TarXz => {
                // In this case we want to DECODE the tar archives so...
                // Check if a valid decoder has been passed as a parameter
                if let None = decoder {
                    return Err(Box::new(DecoderNotFoundError));
                } else {
                    // decode the file
                    decoder.unwrap().decode(
                        download_dir.join(&self.file).to_str().unwrap(),
                        download_dir,
                    )?;
                }
            } // TODO: Other file types...
        }
        // analyze the type of INSTALLATION process
        match self.install_type {
            InstallTypes::MakeInstall => {
                // Run the 'Make' command using the CommandRunner
                if let None = sage_home_path {
                    return Err(Box::new(PathNotFoundError));
                } else {
                    let make_cmd =
                        CommandRunner::Make(&self.install_target, sage_home_path.unwrap());
                    make_cmd.run()?
                }
            }
        }
        // end the progress bar
        self.finish_install_progress(pb);
        Ok(())
    }

    pub fn get_file_type(&self) -> &FileTypes {
        return &self.file_type;
    }
}

// Private API for Package
impl Package {
    fn start_download_progress(&self) -> indicatif::ProgressBar {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.enable_steady_tick(120);
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ])
                .template("{spinner:.green} {msg}"),
        );
        pb.set_message(&format!("Downloading {}@{}...", self.name, self.version));
        pb
    }

    fn finish_download_progress(&self, pb: indicatif::ProgressBar) {
        pb.finish_with_message(&format!(
            "Done! Package {}@{} successfully downloaded as {}.",
            self.name, self.version, self.file,
        ));
    }

    fn start_install_progress(&self) -> indicatif::ProgressBar {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.enable_steady_tick(120);
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ])
                .template("{spinner:.green} {msg}"),
        );
        pb.set_message(&format!(
            "Installing {}@{}. This may take a while...",
            self.name, self.version
        ));
        pb
    }

    fn finish_install_progress(&self, pb: indicatif::ProgressBar) {
        pb.finish_with_message(&format!(
            "Done! Package {}@{} successfully installed in your system.",
            self.name, self.version
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_created_correctly() {
        let pkg = Package::new(
            "Python",
            "3.8",
            "https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz",
            "tar.xz",
            "Python-3.8.0.tar.xz",
            "make",
            "Python-3.8.0",
        )
        .unwrap();

        assert_eq!(pkg.name, String::from("Python"));
        assert_eq!(pkg.version, String::from("3.8"));
        assert_eq!(
            pkg.url,
            String::from("https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz")
        );
        let mut f_type = None;
        if let FileTypes::TarXz = pkg.file_type {
            f_type = Some("tar.xz");
        };
        assert_eq!(f_type.unwrap(), String::from("tar.xz"));
        assert_eq!(pkg.file, String::from("Python-3.8.0.tar.xz"));
        assert_eq!(pkg.install_target, String::from("Python-3.8.0"));
    }
}
