extern crate indicatif;
extern crate reqwest;

use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

/// Representation of a valid `sage` package.
pub struct Package {
    name: String,
    version: String,
    url: String,
    file_type: Option<String>,
    file: Option<OsString>,
}

// Public API for Package
impl Package {
    /// Wraps the given name, version and download url
    /// (the three of them string slices) as a  `Package`.
    pub fn new(name: &str, version: &str, url: &str) -> Package {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            url: url.to_string(),
            file_type: None,
            file: None,
        }
    }

    /// Downloads the `Package` to the specified download directory.
    ///
    /// # Errors
    /// Check out the documentation for `reqwest::get`,
    /// the `copy_to` method of `reqwest::Response`
    /// and the `create` method of `std::fs::File` to see
    /// the conditions in which this function could return an error.
    pub fn download(&mut self, download_dir: &Path) -> Result<(), Box<dyn Error>> {
        // Configure the Progress bar
        let pb = self.start_download_progress();
        // Specify the target url
        let target = self.url.as_str();
        // Make the request
        let mut response = reqwest::get(target)?;

        // Create the destination file
        let mut dest_file = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            let fname = download_dir.join(fname);
            // Set this package's file to said value
            self.file = Some(OsString::from(
                fname
                    .file_name()
                    .expect("No file name found in the specified path"),
            ));
            // Create the dest file
            File::create(&fname)?
        };
        // Copy the file from the response to the destination
        response.copy_to(&mut dest_file)?;

        // Signal the progress bar to end
        self.finish_download_progress(pb);

        Ok(())
    }

    // pub fn install(&self, install_dir: &Path) -> Result<(), Box<dyn Error>> {
    //     // implement
    // }
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
            self.name,
            self.version,
            self.file.as_ref().unwrap().to_str().unwrap()
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
        );

        assert_eq!(pkg.name, String::from("Python"));
        assert_eq!(pkg.version, String::from("3.8"));
        assert_eq!(
            pkg.url,
            String::from("https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz")
        );
    }
}
