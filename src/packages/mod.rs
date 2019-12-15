extern crate indicatif;
extern crate reqwest;

use std::ffi::OsString;
use std::fs::File;
use std::path;

pub struct Package {
    pub name: String,
    pub version: String,
    pub url: String,
    pub file: Option<OsString>,
}

// Public API for Package
impl Package {
    pub fn new(name: &str, version: &str, url: &str) -> Package {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            url: url.to_string(),
            file: None,
        }
    }

    pub fn download(&mut self, download_dir: &path::Path) {
        // Configure the Progress bar
        let pb = self.start_download_progress();
        // Specify the target url
        let target = self.url.as_str();
        // Make the request
        let mut response = reqwest::get(target).expect("Could not make the GET request");

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
            File::create(&fname).expect("Could not create file")
        };
        // Copy the file from the response to the destination
        response
            .copy_to(&mut dest_file)
            .expect("Could not copy to new file");

        // Signal the progress bar to end
        self.finish_download_progress(pb);
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
                .template("{spinner:.blue} {msg}"),
        );
        pb.set_message(&format!("Downloading {}@{}...", self.name, self.version));
        pb
    }

    fn finish_download_progress(&self, pb: indicatif::ProgressBar) {
        pb.finish_with_message(&format!(
            "Done! Package {}@{} successfully downloaded.",
            self.name, self.version
        ));
    }
}
