extern crate reqwest;

use std::fs::File;
use std::io::copy;

pub struct Package {
    pub name: String,
    pub version: String,
    pub url: String,
    pub file: Option<String>,
}

impl Package {
    pub fn new(name: &str, version: &str, url: &str) -> Package {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            url: url.to_string(),
            file: None,
        }
    }

    pub fn download(&mut self) {
        // Specify the target url
        let target = self.url.as_str();
        // Make the request
        let mut response = reqwest::get(target).expect("Could not make the GET request");
        // Create the destination file
        let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            println!("file to download: '{}'", fname);
            // Append the rest of the path to the file
            let fname = format!("./out/{}", fname);
            // Set this package's file to said value
            self.file = Some(fname.clone());
            // Create the dest file
            File::create(fname).expect("Could not create file")
        };
        // Copy the file from the response to the destination
        copy(&mut response, &mut dest).expect("Could not copy to new file");
    }
}
