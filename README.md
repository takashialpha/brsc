# brsc - Basic Rust Calculator

**brsc** (Basic Rust Calculator) is a modern CLI calculator, inspired by the classic `bc` (Basic Calculator), and built with Rust. It allows users to evaluate mathematical expressions with customizable precision and a quiet output mode for clean results.

> **Status: This project has been discontinued and is no longer maintained.**
> Feel free to fork the repository to continue its development or adapt it to your own needs.

Created by: **takashialpha**

---
![Project Version](https://img.shields.io/badge/version-0.9.1-brightgreen)
![Tests](https://img.shields.io/badge/tests-passing-brightgreen)
![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-brightgreen)
![License](https://img.shields.io/badge/license-Apache_2.0-blue.svg)

---

## Features

- Evaluate complex mathematical expressions.
- Adjustable precision for floating-point results.
- Quiet mode to display only the result.
- Lightweight, fast, and efficient.

---

### Installation

To install **brsc**, clone the repository and run the installation script:

```sh
git clone https://github.com/takashialpha/brsc.git
cd brsc
sudo ./INSTALL.sh
```

---

## Usage

Run `brsc` from your terminal with the following options:

```sh
brsc [OPTIONS]
```

### Options:

- `-e, --expr <EXPR>`  
  Mathematical expression to evaluate (e.g., `2 + 2 * 3`).

- `-p, --precision <PRECISION>`  
  Set the precision for floating-point results.  
  Default: `2`.

- `-q, --quiet`  
  Suppress all output except the result.

- `-h, --help`  
  Show help information with a summary of available options.

- `-V, --version`  
  Show the current version of the program.

---

## Examples

Evaluate a simple expression:

```sh
brsc -e "2 + 2 * 3"
```

Set precision to 5 decimal places:

```sh
brsc -e "1 / 3" -p 5
```

Use quiet mode to only show the result:

```sh
brsc -e "2*2" -q
```

---

## Development

Contributions to **brsc** are no longer accepted by the original maintainer. However, you are encouraged to fork the project and adapt it as needed. For reference, consult the [CONTRIBUTING.md](CONTRIBUTING.md) file.

---

## License

This project is licensed under the [Apache License 2.0](LICENSE).

---

## Acknowledgments

- Inspired by the classic `bc` (Basic Calculator) tool.
- Built using Rust for its safety, performance, and modern tooling.

