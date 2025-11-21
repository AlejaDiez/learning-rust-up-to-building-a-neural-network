# Cargo

## Creating a New Project

```bash
cargo new hello_world
```

This command creates a new directory called `hello_world` with the following structure:

```plaintext
hello_world/
├── Cargo.toml
└── src/
    └── main.rs
```

- `Cargo.toml`: This is the manifest file for your Rust project. It contains metadata like the project's name, version, and dependencies.

- `src/main.rs`: This is where you write your application code. Cargo generates a "Hello, world!" program for you automatically, just like the one in your context file.

## Compiling and Running the Project

### Compile and Run in One Step

The easiest way to compile and run your project is with `cargo run`. Navigate into your new project directory and run the command:

```bash
cd hello_world
cargo run
```

- **Compile** your code if it has changed since the last compilation. The resulting executable file is placed in the `target/debug/` directory.

- **Run** run the resulting executable.

### Compiling Separately

```bash
cd hello_world
cargo build
```

This creates the executable at `target/debug/hello_world`. You can then run it directly:

```bash
./target/debug/hello_world
```

### Checking for Errors

```bash
cd hello_world
cargo check
```

### Building for Release

```bash
cd hello_world
cargo build --release
```

This will create an optimized executable in `target/release/` instead of `target/debug/`.
