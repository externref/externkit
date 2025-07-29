# ExternKit

A comprehensive project management toolkit written in Rust with Python integration. ExternKit provides command-line utilities for environment variable management, text editing, and Python tooling.

## Features

- **Environment Variable Management**: Add, update, and delete environment variables
- **Built-in Text Editor**: Nano-like terminal text editor with syntax highlighting
- **Python Integration**: Python extension module and pip management tools
- **Cross-platform**: Available for Windows and Linux

## Installation

### Pre-built Binaries

Download the latest release for your platform:
- **Linux**: `externkit-linux-x86_64.tar.gz`
- **Windows**: `externkit-windows-x86_64.exe.zip`

### From Source

```bash
git clone https://github.com/externref/externkit.git
cd externkit
cargo build --release
```

### Python Package

```bash
pip install externkit
```

## Usage

### Initialize Project

```bash
externkit init
```

### Environment Variable Management

```bash
# Add an environment variable
externkit env add MY_VAR "my_value"

# Update an environment variable
externkit env update MY_VAR "new_value"

# Delete an environment variable
externkit env delete MY_VAR
```

### Text Editor

Launch the built-in nano-like text editor:

```bash
# Create or edit a file
externkit edit myfile.txt

# Start with a new file
externkit edit
```

#### Editor Shortcuts

- **Ctrl+S**: Save file
- **Ctrl+Q**: Quit editor
- **Ctrl+O**: Open file
- **Arrow Keys**: Navigate cursor
- **Backspace**: Delete character
- **Delete**: Delete character forward
- **Enter**: Insert new line

### Python Tools

```bash
# Install pip for a specific Python executable
externkit get_pip --python-path /usr/bin/python3
```

## Python Integration

ExternKit can also be used as a Python module:

```python
import externkit

# Use externkit functions from Python
```

## Development

### Prerequisites

- Rust 1.70+
- Python 3.8+
- Cargo

### Building

```bash
# Build the Rust binary
cargo build --release

# Build the Python extension
pip install maturin
maturin develop
```

### Running Tests

```bash
cargo test
```

## Project Structure

```
externkit/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Python extension entry point
│   ├── backend/          # Core functionality
│   │   ├── env.rs        # Environment variable management
│   │   ├── python_tools.rs # Python tooling
│   │   └── utils.rs      # Utility functions
│   └── editor/           # Text editor implementation
│       ├── editor.rs     # Core editor logic
│       ├── input.rs      # Input handling
│       └── display.rs    # Display rendering
├── Cargo.toml           # Rust dependencies
├── pyproject.toml       # Python package configuration
└── README.md
```

## CI/CD

The project includes GitHub Actions workflows for:
- **Build**: Cross-platform binary builds for Linux and Windows
- **Publish**: Automated PyPI publishing with trusted publishing

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**externref** - [GitHub](https://github.com/externref)

---

*ExternKit - Your all-in-one project management toolkit*