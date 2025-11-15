# AI Agent Contribution Guide

This guide is specifically designed for AI coding assistants to effectively contribute to the **crank** project. It provides context, conventions, and practical guidance for understanding and modifying this codebase.

## Project Overview

**crank** is a command-line build tool for Playdate game development, written in Rust. It's inspired by Cargo and wraps the official Playdate SDK tools to provide a streamlined developer experience.

### Core Purpose
- **SDK Wrapper**: Wraps `pdc` (Playdate Compiler) and `PlaydateSimulator`, doesn't replace them
- **Developer Experience**: Provides Cargo-like workflow for Playdate developers
- **Cross-Platform**: Single Rust codebase for Windows, macOS, and Linux
- **Multi-Language**: Currently supports Lua, with Swift planned

## Architecture Overview

### Module Structure

```
src/
├── main.rs           # Entry point, command dispatch
├── cli.rs            # CLI argument definitions (clap)
├── error.rs          # Custom error types (thiserror)
├── config.rs         # Playdate.toml parsing (serde)
├── project.rs        # Project structure and validation
├── sdk.rs            # SDK detection and tool execution
└── commands/         # Command implementations
    ├── mod.rs
    ├── new.rs        # Project creation
    ├── build.rs      # Build orchestration
    ├── run.rs        # Build + simulator launch
    ├── watch.rs      # File watching & hot reload
    ├── test.rs       # Test runner
    └── clean.rs      # Build artifact cleanup
```

### Key Dependencies

- **clap 4.5**: CLI argument parsing (derive API)
- **serde + toml**: Configuration file parsing
- **notify 6.1**: File system watching
- **anyhow**: Error handling with context
- **thiserror**: Custom error type definitions
- **tokio**: Async runtime
- **colored**: Terminal output styling
- **indicatif**: Progress indicators

## Design Principles

When contributing, respect these core principles:

1. **Convention over Configuration**: Provide sensible defaults
2. **Wrap, Don't Replace**: Use official SDK tools, don't reimplement them
3. **Cross-Platform First**: Test on all major platforms
4. **Ergonomic CLI**: Follow Cargo's UX patterns
5. **Clear Error Messages**: Help users fix problems quickly

## Common Tasks

### Adding a New Command

1. **Define CLI arguments** in `src/cli.rs`:
   ```rust
   #[derive(Subcommand)]
   pub enum Commands {
       // ... existing commands
       
       /// Your new command description
       YourCommand {
           #[arg(short, long)]
           some_flag: bool,
       },
   }
   ```

2. **Create command module** at `src/commands/your_command.rs`:
   ```rust
   use anyhow::Result;
   use crate::config::Config;
   use crate::project::Project;
   
   pub fn execute(/* args */) -> Result<()> {
       // Implementation
       Ok(())
   }
   ```

3. **Register module** in `src/commands/mod.rs`:
   ```rust
   pub mod your_command;
   ```

4. **Wire to main** in `src/main.rs`:
   ```rust
   Commands::YourCommand { some_flag } => {
       commands::your_command::execute(some_flag)?;
   }
   ```

5. **Add documentation** in appropriate `.md` file or create `YOUR_COMMAND_IMPLEMENTATION.md`

### Modifying Project Configuration

The `Playdate.toml` structure is defined in `src/config.rs`. To add new fields:

1. **Update structs** with serde annotations:
   ```rust
   #[derive(Debug, Deserialize)]
   pub struct BuildConfig {
       // ... existing fields
       
       #[serde(default)]
       pub new_field: Option<String>,
   }
   ```

2. **Provide defaults** where appropriate using `#[serde(default)]`

3. **Update template** in `templates/lua-basic/Playdate.toml`

4. **Update documentation** in README.md and DESIGN.md

### Working with SDK Detection

SDK detection logic is in `src/sdk.rs`. The detection order is:

1. `PLAYDATE_SDK_PATH` environment variable
2. `simulator_path` in Playdate.toml
3. Platform-specific defaults:
   - macOS: `~/Developer/PlaydateSDK`
   - Windows: `%USERPROFILE%\Documents\PlaydateSDK`
   - Linux: `~/PlaydateSDK`

When modifying SDK interaction:
- Handle all three platforms
- Test with both environment variable and auto-detection
- Provide helpful error messages when SDK isn't found
- Consider macOS app bundle structure vs. Linux/Windows executables

### Adding Error Types

Use the `CrankError` enum in `src/error.rs`:

```rust
#[derive(Error, Debug)]
pub enum CrankError {
    // ... existing errors
    
    #[error("Your descriptive error message: {0}")]
    YourError(String),
}
```

Use `anyhow::Context` to add context to errors:
```rust
std::fs::read_to_string(&path)
    .context("Failed to read configuration file")?;
```

### Working with Templates

Templates are stored in `templates/` and embedded in the binary:

1. Create template directory structure
2. Use template variables (not yet implemented, but planned):
   - `{{name}}` - project name
   - `{{bundle_id}}` - package bundle ID
   - `{{author}}` - author name
3. Register template in project creation logic

## Code Style & Conventions

### Rust Standards

- **Format**: Always run `cargo fmt` before committing
- **Lint**: Run `cargo clippy` and address warnings
- **Documentation**: Add `///` doc comments for public APIs
- **Errors**: Use `Result<T>` or `anyhow::Result<T>` for fallible functions
- **Naming**:
  - `snake_case` for functions and variables
  - `PascalCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants

### Error Handling Patterns

```rust
// ✅ Good: Use context for better error messages
let config = Config::load(&path)
    .context("Failed to load project configuration")?;

// ✅ Good: Use custom error types for domain errors
return Err(CrankError::SdkNotFound);

// ❌ Avoid: Don't use .unwrap() or .expect() in production code
let value = option.unwrap(); // Use ? or provide fallback

// ✅ Good: Handle errors gracefully
let value = option.ok_or_else(|| CrankError::ConfigNotFound)?;
```

### CLI Output Patterns

Use colored output for better UX:

```rust
use colored::Colorize;

// Success messages
println!("✓ {}", "Built successfully".green().bold());

// Error messages
eprintln!("✗ {}", format!("Build failed: {}", err).red());

// Info messages
println!("→ {}", "Building project...".cyan());

// Warnings
println!("⚠ {}", "Warning: ...".yellow());
```

### File Path Handling

```rust
use std::path::{Path, PathBuf};

// ✅ Good: Use Path/PathBuf for cross-platform compatibility
let path = PathBuf::from(&config.source_dir);
let full_path = project_root.join(&path);

// ✅ Good: Canonicalize when comparing paths
let canonical = path.canonicalize()?;

// ✅ Good: Use dunce to strip UNC prefixes on Windows
// (if added as dependency)
```

## Testing Strategy

### Unit Tests

Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_parsing() {
        // Test implementation
    }
}
```

### Integration Tests

For command-level testing, use temporary directories:

```rust
use tempfile::tempdir;

#[test]
fn test_new_command() {
    let dir = tempdir().unwrap();
    let result = commands::new::execute("test-project", Some(dir.path()), None);
    assert!(result.is_ok());
    // Verify created files
}
```

### Manual Testing Checklist

When modifying commands, test:
- ✅ Happy path with valid inputs
- ✅ Error handling with invalid inputs
- ✅ Cross-platform compatibility (if possible)
- ✅ Edge cases (empty directories, missing SDK, etc.)
- ✅ Help text (`crank command --help`)

## Debugging Tips

### Environment Variables

```bash
# Enable Rust backtrace for better error messages
export RUST_BACKTRACE=1

# Set SDK path explicitly
export PLAYDATE_SDK_PATH=/path/to/PlaydateSDK

# Run with verbose output
cargo run -- build --verbose
```

### Common Issues

1. **SDK Not Found**: Check platform-specific default paths
2. **Build Failures**: Verify `pdc` executable permissions and path
3. **Simulator Won't Launch**: Check app bundle structure on macOS
4. **Watch Mode Issues**: Verify file system permissions and notify crate compatibility

### Useful Commands

```bash
# Build and run locally
cargo run -- new test-game
cd test-game
cargo run -- build

# Check for compilation errors
cargo check

# Run with backtrace
RUST_BACKTRACE=1 cargo run -- build

# Run specific tests
cargo test test_name

# Check documentation
cargo doc --open
```

## Reading the Codebase

### Start Here

1. **src/main.rs** - Entry point, understand command dispatch
2. **src/cli.rs** - See all available commands and their arguments
3. **src/config.rs** - Understand project configuration structure
4. **src/commands/build.rs** - Core build logic, good reference for command structure

### Understanding Data Flow

```
User Input (CLI)
    ↓
main.rs (dispatch)
    ↓
commands/[command].rs (execution)
    ↓
├─ config.rs (load Playdate.toml)
├─ project.rs (validate structure)
└─ sdk.rs (locate and run SDK tools)
    ↓
Output / Error handling
```

### Key Patterns

1. **Configuration Loading**:
   ```rust
   let project_root = Project::find_root()?;
   let config = Config::load(&project_root.join("Playdate.toml"))?;
   ```

2. **SDK Tool Execution**:
   ```rust
   let sdk = Sdk::detect()?;
   sdk.run_pdc(&source_dir, &output_dir)?;
   ```

3. **Error Propagation**:
   ```rust
   fn execute() -> Result<()> {
       let config = load_config()?; // Early return on error
       build_project(&config)?;      // Propagate errors up
       Ok(())
   }
   ```

## Implementation Documentation

Several commands have detailed implementation documentation:

- `BUILD_COMMAND_IMPLEMENTATION.md` - Build system details
- `CLEAN_COMMAND_IMPLEMENTATION.md` - Cleanup logic
- `NEW_COMMAND_IMPLEMENTATION.md` - Project creation
- `RUN_COMMAND_IMPLEMENTATION.md` - Simulator launching
- `TEST_COMMAND_IMPLEMENTATION.md` - Test runner
- `PATH_PARAMETER_IMPLEMENTATION.md` - Path handling
- `PDXINFO_IMPLEMENTATION.md` - PDX metadata
- `LUARC_AND_LUACATS_IMPLEMENTATION.md` - IDE integration

**Always read relevant implementation docs before modifying commands.**

## Platform-Specific Considerations

### macOS
- Simulator is an `.app` bundle: `PlaydateSimulator.app/Contents/MacOS/PlaydateSimulator`
- Use `open -a` or direct binary execution
- Default SDK path: `~/Developer/PlaydateSDK`

### Windows
- Executables have `.exe` extension
- Use `\` path separator (but Rust's PathBuf handles this)
- Default SDK path: `%USERPROFILE%\Documents\PlaydateSDK`
- Handle UNC paths (`\\?\C:\...`)

### Linux
- Direct executable execution
- Watch for case-sensitive file systems
- Default SDK path: `~/PlaydateSDK`
- May need different SDL dependencies

## Contributing Guidelines

### Before Starting Work

1. Read `CONTRIBUTING.md` for general guidelines
2. Check existing issues for related work
3. Read `DESIGN.md` to understand architecture
4. Review relevant `*_IMPLEMENTATION.md` files

### Pull Request Checklist

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated (README.md, DESIGN.md, etc.)
- [ ] Implementation documentation added/updated if needed
- [ ] Commit messages are clear and descriptive

### Documentation Requirements

When adding features:
1. Update README.md with user-facing changes
2. Update DESIGN.md with architectural changes
3. Add command-specific `*_IMPLEMENTATION.md` for complex features
4. Add inline code documentation (`///`) for public APIs
5. Include examples in documentation

## Future Directions

Be aware of planned features when contributing:

- **Swift Support**: Keep language abstraction in mind
- **Dependency Management**: Plan for package registry integration
- **Asset Pipeline**: Image optimization, sprite sheets
- **Plugin System**: Extensibility for third-party tools
- **Device Deployment**: Sideloading to physical Playdate

Design changes should not preclude these future additions.

## Useful Context

### Playdate SDK Structure

```
PlaydateSDK/
├── bin/
│   ├── pdc                    # Compiler (macOS/Linux)
│   ├── pdc.exe                # Compiler (Windows)
│   └── PlaydateSimulator.app  # Simulator (macOS)
├── C_API/                      # C headers
├── CoreLibs/                   # Lua standard library
└── Examples/                   # Sample games
```

### .pdx Bundle Structure

```
game.pdx/
├── pdxinfo          # Metadata (name, author, bundle ID)
├── main.pdz         # Compiled Lua bytecode
├── main.pdz.debug   # Debug symbols (optional)
└── [assets]/        # Images, sounds, fonts
```

### Playdate.toml Schema

See `src/config.rs` for the authoritative schema. Key sections:
- `[package]` - Project metadata
- `[build]` - Build configuration
- `[playdate]` - Playdate-specific settings
- `[dev]` - Development options

## Getting Help

When stuck or uncertain:

1. **Read the code**: Start with similar existing commands
2. **Check docs**: README, DESIGN, and implementation docs
3. **Examine tests**: See how features are tested
4. **Review PRs**: Look at previous changes for patterns
5. **Ask questions**: Open a discussion on GitHub

## Tips for AI Agents

### Effective Code Reading

- Start with `main.rs` and follow the execution flow
- Read struct definitions before implementation
- Pay attention to error types - they reveal edge cases
- Check tests to understand expected behavior

### Making Changes

- **Small, focused changes**: One logical change per modification
- **Test incrementally**: Build after each significant change
- **Follow existing patterns**: Match the style of surrounding code
- **Preserve cross-platform compatibility**: Test assumptions about file paths, executables

### Understanding Context

- Read git history for problematic areas (`git log --follow path/to/file.rs`)
- Check TODO comments in code for planned work
- Review GitHub issues for context on features
- Read implementation docs before touching commands

### Common Pitfalls to Avoid

1. ❌ Assuming Unix-only paths (use `PathBuf`)
2. ❌ Hardcoding file separators (use `path.join()`)
3. ❌ Using `.unwrap()` in production code
4. ❌ Ignoring cross-platform testing
5. ❌ Breaking backward compatibility without discussion
6. ❌ Adding dependencies without justification

## Summary

**crank** is a well-structured Rust project that wraps Playdate SDK tools. Key points:

- Commands live in `src/commands/`
- Configuration is in `src/config.rs`
- SDK interaction is in `src/sdk.rs`
- Error handling uses custom types + anyhow
- Follow Cargo's UX patterns
- Maintain cross-platform compatibility
- Write clear, helpful error messages
- Document complex features in `*_IMPLEMENTATION.md` files

When in doubt, follow existing patterns and read the implementation documentation.

---

**Happy contributing! 🎮**

