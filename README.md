# Comparison of calculation speed by Leibniz series

For verifying the calculation speed of the Leibniz series of programs<br>
![Leibniz series](/leibniz_series.png)

## Requirement

- [Rust 1.44.1](https://www.rust-lang.org/)
- [java 14](https://www.oracle.com/java/technologies/javase-downloads.html)
- [Python 3.8.2](https://www.python.org/downloads/release/python-382/)
- [Node.js 12.18.1](https://nodejs.org/ja/)

## Installation

```
$ git clone https://github.com/m2aug9/speed_verification_programming.git
```

## Usage

- Rust 1.44.1
```
$ cd rust_src
$ cargo init
$ cargo run
## error: linker `link.exe` not found
Windows: Install visualcppbuildtools_full.exe
```
- java 14
```
$ cd java_src/bin
$ java java_src.main_function
```
- Python 3.8.2
```
$ cd python_src
$ python main_function.py
```
- Node.js 12.18.1
```
$ cd nodejs_src
$ node main_function.py
```
