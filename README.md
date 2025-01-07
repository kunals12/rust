# Rust Workspace Project

This repository is a Rust workspace containing multiple projects for learning and practice. Each project is structured as a standalone Rust binary and is managed under a common workspace.


## **Prerequisites**
- [Rust and Cargo](https://www.rust-lang.org/tools/install): Ensure you have Rust installed (version 1.60+ recommended).

## **Setup**
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/your-repo-name.git
   cd your-repo-name
    ```

2. Ensure all dependencies are installed:
    ```bash
    cargo check
    ```

## How to Build

To build all the projects in the workspace:
```bash
cargo build
```

## How to Run Projects
```bash
cargo run -p <project-name>
```

## Example
```bash
cargo run -p hello_world
```