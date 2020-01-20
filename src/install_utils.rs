extern crate flate2;
extern crate tar;
extern crate tempfile;
extern crate xz2;

use flate2::read::GzDecoder;
use tar::Archive;
use tempfile::NamedTempFile;
use xz2::read::XzDecoder;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

/// Enum containing every single supported installation type
pub enum InstallTypes {
    MakeInstall, // TODO: Add single file binary installation
}

/// Enum cotaining every single file type that can be downloaded
pub enum FileTypes {
    TarXz,
    TarGz,
}

/// General description of how a decoder must behave
pub trait Decoder {
    fn new() -> Self;
    fn decode(&self, file_name: &str, unpack_dir: &Path) -> Result<(), Box<dyn Error>>;
}

/// A decoder for tar.xz files
pub struct TarXzDecoder {}
/// A decoder for tar.gz files
pub struct TarGzDecoder {}

impl Decoder for TarXzDecoder {
    /// creates a new tar.xz decoder
    fn new() -> TarXzDecoder {
        TarXzDecoder {}
    }

    /// Decodes a tar.xz file
    /// # Observations
    /// - file_name must be the full path to the file
    fn decode(&self, file_name: &str, unpack_dir: &Path) -> Result<(), Box<dyn Error>> {
        let xz_file = File::open(file_name)?;
        let mut decompressed = XzDecoder::new(xz_file);
        // create a buffer in which to store the xz decomp data
        let mut buf: Vec<u8> = Vec::new();
        decompressed.read_to_end(&mut buf)?;
        // create a temp file and store the contents of the buffer in it
        let mut tar_file = NamedTempFile::new()?;
        tar_file.write_all(buf.as_mut_slice())?;
        // reopen the temp file to have another handle, and extract the contents
        let tar_file_2 = tar_file.reopen()?;
        let mut archive = Archive::new(tar_file_2);
        archive.unpack(unpack_dir)?;

        Ok(())
    }
}

impl Decoder for TarGzDecoder {
    /// creates a new tar.gz decoder
    fn new() -> TarGzDecoder {
        TarGzDecoder {}
    }

    // TODO: this implementation is too slow, try to optimize it
    /// Decodes a tar.gz file.
    /// # Observations
    /// - file_name must be the full path to the file
    fn decode(&self, file_name: &str, unpack_dir: &Path) -> Result<(), Box<dyn Error>> {
        let gz_file = File::open(file_name)?;
        let tar = GzDecoder::new(gz_file);
        let mut archive = Archive::new(tar);
        archive.unpack(unpack_dir)?;

        Ok(())
    }
}

/// Command runner that abstracts the interaction with third-party
/// programs that aid in the installation process, like *make*, *git*, etc
pub enum CommandRunner {
    Make(String, String), // args: program, sage_home_path
                          // Git, // not implemented yet // TODO: Add other commands to the command runner, like git.
}

impl CommandRunner {
    /// Run the specified command
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            // If the specified command is make
            Self::Make(program, sage_home_path) => {
                // create the ./configure command
                let mut configure = Command::new("./configure");
                configure
                    .current_dir(format!("{}/downloads/{}", sage_home_path, program))
                    .arg(format!("--prefix={}", sage_home_path))
                    .arg(format!("--exec-prefix={}", sage_home_path))
                    // run it
                    .status()?;
                // now create the make command
                let mut make = Command::new("make");
                make.current_dir(format!("{}/downloads/{}", sage_home_path, program));
                // run make
                make.status()?;
                // now run make install
                make.arg("install").status()?;

                Ok(())
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tar_xz_decoded_successfully() {
//         let decoder = TarXzDecoder::new();
//         let path = std::path::PathBuf::from("/home/rvillegasm/.sage/downloads");
//         let file = "/home/rvillegasm/.sage/downloads/Python-3.8.0.tar.xz";

//         decoder.decode(file, &path).unwrap();
//     }

//     #[test]
//     fn tar_gz_decoded_successfully() {
//         let decoder = TarGzDecoder::new();
//         let path = std::path::PathBuf::from("/home/rvillegasm/.sage/downloads");
//         let file = "/home/rvillegasm/.sage/downloads/Python-3.8.1.tgz";

//         decoder.decode(file, &path).unwrap();
//     }
// }
