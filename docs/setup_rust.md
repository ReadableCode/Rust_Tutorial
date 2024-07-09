# Setup Rust

## Setup on Windows

- Using WinGet

  - Open powershell as administrator and run:

  ```bash
  winget install -e --id Rustlang.Rustup
  ```

## Testing Installation

- Close and reopen the terminal to make sure installation is successful and hen run the folling commands to verify the versions of rustup and rustc:
  
  ```bash
  rustup --version
  rustc --version
  ```

## Compile and Run a Rust Program

- Create a new file with the `.rs` extension and write the following code:

  ```rust
  fn main() {
      println!("hello world!");
  }
  ```

- Compile the program using the following command:

  ```bash
  rustc <filename>.rs
  ```

- Run the compiled program using the following command:

  ```bash
  ./<filename>
  ```
