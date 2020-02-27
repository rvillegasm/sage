# Sage

A simple package manager for Linux, macOs and Windows.

[![Build Status](https://travis-ci.org/rvillegasm/sage.svg?branch=master)](https://travis-ci.org/rvillegasm/sage)


<!-- PROJECT LOGO -->
<!-- <br />
<p align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Best-README-Template</h3>

  <p align="center">
    An awesome README template to jumpstart your projects!
    <br />
    <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/othneildrew/Best-README-Template">View Demo</a>
    ·
    <a href="https://github.com/othneildrew/Best-README-Template/issues">Report Bug</a>
    ·
    <a href="https://github.com/othneildrew/Best-README-Template/issues">Request Feature</a>
  </p>
</p> -->

<!-- TABLE OF CONTENTS -->
## Table of Contents

* [About Sage](#about-sage)
  * [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)



<!-- ABOUT THE PROJECT -->
## About Sage
Sage is a **cross-platform system package manager** that helps you download,
install, update and remove software packages at will, and you can do so
with just a couple commands. 

But wait, you may be telling yourself: 
*"What even is a system package manager?"*
Well, it's quite like your typical package managers, like
node's npm or rust's cargo. The difference is that it's not associated to a
specific programming language, but rather to your operating system.
With things like npm and cargo you would install libraries or binaries
written in their associated programming language, but with a system package
manager, like Sage, you can install anything that you want, from tools like 
`cat` or `grep` to programming language SDKs like `openjdk`, provided that it is somehow available to the package manager.

If you are familiar with Linux, and to some extent with macOs, then you
have probably used a package manager of this sort before. 
Debian's [apt](https://en.wikipedia.org/wiki/APT_(software)), 
Arch's [pacman](https://wiki.archlinux.org/index.php/Pacman) 
and macOs's [homebrew](https://brew.sh/) are a couple of examples of
system package managers that people which use those OSes interact with
all the time.

I know that by now you may be thinking:
*"If there are so many system package managers out there,
then why bother creating a new one? This sounds like a waste of time, I'll
just leave".* **Wait**! Don't go anywhere just yet and hear me out, because
there is a very good reason for another package manager to exist.

Most package managers out there only work for a specific platform: *apt* 
only works on debian based systems, *pacman* only works on the mighty
Arch system, and *homebrew* only works on unix-based systems. That means that
if you are one of those people that uses several operating systems at the
same time, you would have to get used to working with two to three
different pieces of software to manage every single program that you use.

Sage, on the other hand, works fine on every mayor OS out there. Linux? Check. MacOs? Check. Windows? Check. And, the best part of it is that it
works the same no matter the platform.

### Built With
Sage was built using the [Rust](https://www.rust-lang.org/)
programming language.

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites
* Rust: To get Sage up and running in your system you first have to install 
  Rust. I recommend using the 
  [rustup](https://www.rust-lang.org/tools/install) 
  tool provided by the Rust developers.

### Installation
Then, after that you should run the following command:
```bash
$ cargo install --git https://github.com/rvillegasm/sage
```
That will clone the repo, compile it and add it to your PATH.

<!-- USAGE EXAMPLES -->
## Usage

### Getting information about a certain package
```bash
$ sage info <name-of-package>
```

### Getting specific details about a version of a package
```bash
$ sage details <name-of-package>@<version>
```

### Downloading a version of a package
```bash
$ sage download <name-of-package>@<version>
```

### Downloading and installing a version of a package
```bash
$ sage install <name-of-package>@<version>
```

<!-- ROADMAP -->
## Roadmap
See the [open issues](https://github.com/rvillegasm/sage/issues) for a list
of proposed features (and known issues).

The proyect is still in early development, and MANY more features
and helpful documentation will be introduced in the comming months.

<!-- CONTRIBUTING -->
## Contributing
Please, if you have an idea about some new feature, and you want to add
it to Sage, please do so!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License
Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact
Rafael Villegas Michel - rafa.villegas.michel@gmail.com

Project Link: [https://github.com/rvillegasm/sage](https://github.com/rvillegasm/sage)
