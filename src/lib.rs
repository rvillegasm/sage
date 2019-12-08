mod packages;

use packages::Package;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut pkg = Package::new(
        "Python",
        "3.8",
        "https://www.python.org/ftp/python/3.8.0/Python-3.8.0.tar.xz",
    );

    println!(
        "Name: {}\nVersion: {}\nUrl: {}",
        pkg.name, pkg.version, pkg.url
    );

    pkg.download();

    Ok(())
}
