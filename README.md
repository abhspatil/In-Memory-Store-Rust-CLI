# Inmemory-cli CLI Tool

`inmemory-cli` is a Rust-based, in-memory CLI tool for managing key-value pairs and lists. It supports storing data persistently in files, allowing data to be retained between sessions. This tool is perfect for quick, lightweight storage and retrieval of key-value data.

## Features

- **Set** a key-value pair.
- **Append** values to a list.
- **Display** all stored key-value pairs and lists.
- Data **persistence** with JSON file storage.

## Requirements

- **Rust**: Make sure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).

## Setup and Installation

1. **Clone the Repository** (or create a new project with `cargo new inmemory-cli` if starting from scratch).

2. **Navigate to the Project Directory**:

   ```bash
   cd inmemory-cli
   ```

3. **Add Dependencies**:

   Open `Cargo.toml` and add these dependencies:

   ```toml
   [dependencies]
   clap = "4.0"          # Command-line argument parsing
   serde = { version = "1.0", features = ["derive"] }   # Serialization
   serde_json = "1.0"    # JSON handling
   ```

4. **Build the Project**:

   Compile the project in release mode to generate an optimized binary:

   ```bash
   cargo build --release
   ```

5. **Run the Binary**:

   Run the executable directly from the `target/release` directory:

   ```bash
   ./target/release/inmemory-cli --help
   ```

   Alternatively, move the binary to a global path for easier access:

   ```bash
   sudo mv target/release/inmemory-cli /usr/local/bin/
   ```

   Now, you can run `inmemory-cli` from anywhere.

## Usage

### Commands

1. **Set a Key-Value Pair**:

   ```bash
   inmemory-cli set <key> <value>
   ```

   Example:

   ```bash
   inmemory-cli set username abhishek
   ```

2. **Append a Value to a List**:

   ```bash
   inmemory-cli append <list> <value>
   ```

   Example:

   ```bash
   inmemory-cli append fruits apple
   ```

3. **Display All Data**:

   ```bash
   inmemory-cli display
   ```

   This command will print out all key-value pairs and lists stored in the system.

### Persistent Storage

The tool stores data in JSON files within the project directory:

- **Key-Value Store**: `kv_store.json`
- **List Store**: `list_store.json`

These files allow the data to persist across sessions, so data will not be lost even if the program is closed.

## Example Workflow

```bash
# Set key-value pair
inmemory-cli set username abhishek

# Append values to lists
inmemory-cli append fruits apple
inmemory-cli append fruits banana
inmemory-cli append colors red

# Display all stored data
inmemory-cli display
```

This will display:

```
Key-Value Store:
  username: pats

List Store:
  fruits: ["apple", "banana"]
  colors: ["red"]
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
