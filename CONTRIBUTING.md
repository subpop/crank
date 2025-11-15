# Contributing to crank

Thank you for your interest in contributing to crank! This document provides guidelines for contributing to the project and helps you get started with development.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Playdate SDK (for testing)
- Git

### Project Structure

```
crank/
├── Cargo.toml              # Rust project manifest
├── README.md               # User documentation
├── DESIGN.md               # Architecture documentation
├── CONTRIBUTING.md         # Contribution guidelines
├── LICENSE                 # MIT License
├── .gitignore             # Git ignore rules
│
├── src/                    # Source code
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library root
│   ├── cli.rs             # CLI argument parsing
│   ├── error.rs           # Error types
│   ├── config.rs          # Playdate.toml parsing
│   ├── sdk.rs             # Playdate SDK integration
│   ├── project.rs         # Project management
│   └── commands/          # Command implementations
│       ├── mod.rs
│       ├── new.rs         # Create new project
│       ├── build.rs       # Build project
│       ├── run.rs         # Build and run
│       ├── watch.rs       # Watch for changes
│       ├── test.rs        # Run tests
│       └── clean.rs       # Clean build
│
└── templates/             # Project templates
    └── lua-basic/         # Basic Lua template
        ├── main.lua
        ├── Playdate.toml
        ├── README.md
        └── test_basic.lua
```

### Setting Up Development Environment

1. Clone the repository:
   ```bash
   git clone https://github.com/subpop/crank.git
   cd crank
   ```

2. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Optionally install Playdate SDK for testing:
   - Download from https://play.date/dev/
   - Set `PLAYDATE_SDK_PATH` environment variable:
     ```bash
     export PLAYDATE_SDK_PATH="$HOME/Developer/PlaydateSDK"
     ```

4. Build the project:
   ```bash
   cargo build
   ```

5. Run tests:
   ```bash
   cargo test
   ```

6. Try the CLI:
   ```bash
   cargo run -- --help
   ```

## Building crank

### Build Commands

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run without installing
cargo run -- --help

# Install to your system
cargo install --path .
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Manual Testing

1. Create a test project:
   ```bash
   cargo run -- new test-game
   cd test-game
   ```

2. Build the project (requires Playdate SDK):
   ```bash
   cargo run -- build
   ```

3. Run in simulator (requires Playdate SDK):
   ```bash
   cargo run -- run
   ```

### Code Quality

```bash
# Check for common mistakes
cargo clippy

# Format code
cargo fmt

# Check formatting without changing files
cargo fmt -- --check
```

### Testing Guidelines

- Add unit tests for new functionality
- Test on multiple platforms if possible (Windows, macOS, Linux)
- Test with actual Playdate SDK when relevant
- Update integration tests as needed

## Development Workflow

### For Contributors

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** with clear, descriptive commits
4. **Add tests** for new functionality
5. **Run tests** to ensure nothing breaks:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
6. **Push to your fork** and submit a pull request

### Daily Development Cycle

1. **Make changes** to source files
2. **Build** to check for compilation errors
3. **Run clippy** to catch common issues
4. **Format** code with `cargo fmt`
5. **Test** your changes
6. **Commit** with a clear message

## Code Style

- Follow Rust standard conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write clear, self-documenting code with comments where needed
- Add documentation comments (`///`) for public APIs

## Available Commands

Once built, crank provides these commands:

```bash
crank new <name>        # Create new project
crank build             # Build project
crank run               # Build and run in simulator
crank watch             # Watch for changes
crank test              # Run tests
crank clean             # Clean build artifacts
```

## Pull Request Process

1. **Update documentation** if you've changed functionality
2. **Update DESIGN.md** if you've changed architecture
3. **Add yourself** to the contributors list if it's your first contribution
4. **Describe your changes** clearly in the PR description
5. **Link related issues** using GitHub keywords (e.g., "Fixes #123")
6. **Be responsive** to review feedback

## Commit Messages

Write clear commit messages:
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit first line to 72 characters
- Reference issues and PRs when relevant

Examples:
```
Add watch command with file system monitoring
Fix SDK detection on Windows
Update build command to support verbose flag
```

## Key Features Implemented

✅ **Project Creation**
- Creates standard project structure
- Generates Playdate.toml configuration
- Includes Lua template with basic game

✅ **Build System**
- Detects Playdate SDK automatically
- Wraps pdc compiler
- Validates project structure

✅ **Simulator Integration**
- Launches Playdate Simulator
- Platform-specific handling (macOS, Windows, Linux)

✅ **Watch Mode**
- Monitors source and asset files
- Auto-rebuilds on changes
- Debouncing to prevent rapid rebuilds

✅ **Configuration**
- TOML-based project configuration
- Sensible defaults
- Extensible structure

## Areas for Contribution

### High Priority
- Test runner implementation
- Asset pipeline (image optimization, etc.)
- Better error messages and diagnostics
- Cross-platform testing and bug fixes

### Medium Priority
- Additional project templates
- Configuration validation
- Performance optimizations
- Documentation improvements

### Future Features
- Swift language support
- Dependency management system
- Device deployment tools
- Plugin system

## Features Planned for Future

🔲 **Test Runner**
- Execute Lua tests
- Report results
- Integration with simulator

🔲 **Asset Pipeline**
- Image optimization
- Audio conversion
- Font compilation

🔲 **Swift Support**
- Swift project templates
- Swift compilation

🔲 **Dependency Management**
- Lua library dependencies
- Version resolution
- Package registry

## Troubleshooting

### SDK Not Found

If you get "SDK not found" errors:

1. Install the Playdate SDK
2. Set environment variable:
   ```bash
   export PLAYDATE_SDK_PATH="$HOME/Developer/PlaydateSDK"
   ```
3. Or add to your shell profile (~/.bashrc, ~/.zshrc, etc.)

### Build Errors

If you get Rust compilation errors:

1. Update Rust: `rustup update`
2. Clean build: `cargo clean`
3. Rebuild: `cargo build`

### Platform-Specific Issues

**macOS:**
- Ensure Xcode Command Line Tools are installed: `xcode-select --install`

**Windows:**
- Ensure Visual Studio Build Tools are installed
- Use PowerShell or Command Prompt

**Linux:**
- Install build essentials: `sudo apt install build-essential`

## Reporting Bugs

When reporting bugs, please include:
- crank version (`crank --version`)
- Operating system and version
- Playdate SDK version
- Steps to reproduce
- Expected vs actual behavior
- Any error messages or logs

## Suggesting Features

We love feature suggestions! Please:
- Check if the feature already exists or is planned
- Describe the use case clearly
- Explain how it fits with crank's goals
- Consider backward compatibility

## Next Steps

1. **Read DESIGN.md** for architecture details
2. **Try building** crank and creating a test project
3. **Explore the code** starting with `src/main.rs`
4. **Check existing issues** for ideas on what to work on
5. **Open issues** for bugs or feature requests

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Playdate SDK Docs](https://sdk.play.date/)
- [clap Documentation](https://docs.rs/clap/)
- [serde Documentation](https://serde.rs/)

## Questions?

- Open a discussion on GitHub
- Check existing issues and documentation
- Be respectful and patient

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to crank! 🎮
