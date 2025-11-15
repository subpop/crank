# crank - Design Document

## Overview

**crank** is a command-line build tool for Playdate game development, inspired by Cargo's workflow. It provides project scaffolding, build automation, and development tools to streamline the Playdate game development process.

### Goals

- Simplify Playdate game project creation and management
- Provide a unified interface for building and running games
- Support hot-reload and watch mode for rapid iteration
- Enable testing infrastructure for Playdate games
- Support multiple languages (Lua first, Swift planned)
- Work seamlessly across Windows, Linux, and macOS

## Architecture

### Design Principles

1. **SDK Wrapping, Not Replacement**: crank wraps official Playdate SDK tools (pdc, PlaydateSimulator) rather than reimplementing them
2. **Convention over Configuration**: Sensible defaults with optional customization
3. **Language Agnostic**: Core architecture supports multiple languages through abstraction
4. **Cross-Platform**: Single codebase works on all major platforms

### Core Components

#### 1. CLI Interface (`cli.rs`)

Uses `clap` v4 with the derive API for ergonomic command definition:

```rust
Commands:
  new <name>     Create a new Playdate project
  build          Build the project
  run            Build and run in simulator
  test           Run tests
  watch          Watch for changes and rebuild
  clean          Clean build artifacts
```

#### 2. Configuration System (`config.rs`)

**Playdate.toml** structure:

```toml
[package]
name = "game-name"
version = "1.0.0"
author = "Developer Name"
description = "Game description"
bundle_id = "com.example.game"

[build]
language = "lua"           # or "swift"
source_dir = "source"
output_dir = "build"
assets_dir = "assets"

[playdate]
content_warning = ""       # Optional content warnings
content_warning_2 = ""
image_path = "icon"        # Path to game icon

[dev]
hot_reload = true
auto_build = true
simulator_path = ""        # Optional: override SDK path

[dependencies]
# Future: library dependencies
```

#### 3. SDK Integration (`sdk.rs`)

Responsibilities:
- Detect Playdate SDK installation
- Locate `pdc` compiler
- Locate PlaydateSimulator executable
- Handle platform-specific paths (macOS app bundle, Windows/Linux executables)
- Execute SDK tools with proper arguments

**SDK Detection Strategy**:
1. Check `PLAYDATE_SDK_PATH` environment variable
2. Check config file override (`simulator_path`)
3. Platform-specific default locations:
   - macOS: `~/Developer/PlaydateSDK`
   - Windows: `%USERPROFILE%\Documents\PlaydateSDK`
   - Linux: `~/PlaydateSDK`

#### 4. Project Management (`project.rs`)

Handles:
- Project structure validation
- Template instantiation
- Directory creation
- Asset organization

**Standard Project Layout**:
```
my-game/
├── Playdate.toml
├── source/
│   ├── main.lua
│   └── pdxinfo
├── assets/
│   ├── images/
│   ├── sounds/
│   └── fonts/
├── tests/
│   └── test_*.lua
└── build/
    └── my-game.pdx/
```

#### 5. Asset Pipeline (`asset.rs`)

Future capabilities:
- Image optimization (convert to 1-bit, table format)
- Font compilation
- Audio conversion
- Asset manifest generation

#### 6. Build System (`commands/build.rs`)

Process:
1. Validate project structure and Playdate.toml
2. Process assets (if needed)
3. Invoke `pdc` with appropriate flags
4. Handle build errors and display user-friendly messages
5. Generate `.pdx` bundle in output directory

#### 7. Development Workflow (`commands/watch.rs`)

Uses `notify` crate for file system watching:
- Monitor source directory for changes
- Monitor assets directory for changes
- Debounce rapid changes
- Trigger rebuild on file save
- Optional: signal simulator to reload (if possible)

#### 8. Test Runner (`commands/test.rs`)

Test discovery patterns:
- `tests/test_*.lua`
- `tests/*_test.lua`

Future implementation details:
- Execute tests in simulator or headless mode
- Parse test output
- Display results with pass/fail summary

### Error Handling

Custom error types using `thiserror`:

```rust
pub enum PdbmError {
    SdkNotFound,
    InvalidProject,
    BuildFailed(String),
    ConfigError(String),
    IoError(std::io::Error),
}
```

Graceful error messages with context using `anyhow`.

## Multi-Language Support

### Language Abstraction

```rust
trait Language {
    fn name(&self) -> &str;
    fn source_extension(&self) -> &str;
    fn entry_point(&self) -> &str;
    fn build(&self, sdk: &Sdk, project: &Project) -> Result<()>;
}

struct LuaLanguage;
struct SwiftLanguage;  // Future
```

### Template System

Templates stored in `templates/` directory:
- `templates/lua-basic/` - Minimal Lua project
- `templates/lua-advanced/` - Lua with examples
- `templates/swift-basic/` - Future Swift support

Templates are embedded in binary at compile time for easy distribution.

## Command Implementations

### `crank new <name>`

1. Validate project name (alphanumeric, hyphens)
2. Create project directory
3. Copy template files
4. Replace template variables ({{name}}, {{bundle_id}})
5. Initialize git repository (optional)
6. Display success message with next steps

### `crank build`

1. Locate project root (search for Playdate.toml)
2. Parse configuration
3. Validate SDK installation
4. Process assets (future)
5. Execute `pdc source_dir output_dir`
6. Report build time and output size

### `crank run`

1. Execute build process
2. Locate simulator executable
3. Launch simulator with `.pdx` path
4. Handle platform-specific launching:
   - macOS: `open -a PlaydateSimulator game.pdx`
   - Windows: `PlaydateSimulator.exe game.pdx`
   - Linux: `PlaydateSimulator game.pdx`

### `crank watch`

1. Start file watcher on source and assets
2. Display "Watching for changes..." message
3. On file change:
   - Clear console
   - Show changed file
   - Trigger rebuild
   - Report status
4. Handle Ctrl+C gracefully

### `crank test`

1. Discover test files
2. Build test runner
3. Execute in simulator
4. Parse output for test results
5. Display summary with colors (green/red)

## Development Phases

### Phase 1: Foundation (Current)
- Project structure
- Basic CLI scaffolding
- Configuration parsing
- Template system

### Phase 2: Core Commands
- `new` command with Lua template
- `build` command with pdc integration
- SDK detection across platforms
- Error handling

### Phase 3: Run & Simulator
- `run` command
- Simulator launching
- Platform-specific path handling

### Phase 4: Development Tools
- `watch` command
- File watching with debouncing
- Asset processing basics

### Phase 5: Testing & Polish
- `test` command implementation
- Documentation
- Examples
- Performance optimization

## Future Enhancements

1. **Dependency Management**
   - Package registry for Lua libraries
   - Version resolution
   - Lock file for reproducible builds

2. **Advanced Asset Pipeline**
   - Automatic image optimization
   - Sprite sheet generation
   - Audio compression

3. **Deployment Tools**
   - `pdbm package` for distribution
   - Sideload to device
   - Itch.io integration

4. **Swift Support**
   - Swift project templates
   - Xcode integration
   - C API bindings

5. **Plugin System**
   - Custom build steps
   - Third-party integrations
   - Language extensions

## Technical Decisions

### Why Rust?
- Cross-platform compilation
- Excellent error handling
- Fast execution
- Rich ecosystem (clap, serde, etc.)
- Single binary distribution

### Why Wrap SDK Tools?
- Avoid reimplementing complex compilation
- Stay compatible with official tools
- Leverage Panic's ongoing improvements
- Easier maintenance

### Why TOML for Configuration?
- Familiar to Rust developers (Cargo.toml)
- Human-readable and editable
- Strong typing with serde
- Comments supported

## Security Considerations

- Validate all user input (project names, paths)
- Sanitize shell commands
- Don't execute arbitrary code from config
- Safe file operations (no path traversal)

## Performance Considerations

- Lazy SDK detection (only when needed)
- Efficient file watching (debouncing)
- Minimal overhead over direct SDK usage
- Fast project creation with embedded templates

## Compatibility

### Minimum Requirements
- Playdate SDK 1.0+
- Rust 1.70+ (for development)
- Platform-specific:
  - macOS 10.14+
  - Windows 10+
  - Linux (glibc 2.27+)

### Tested Configurations
- macOS (ARM64, x86_64)
- Windows 10/11
- Linux (Ubuntu 20.04+, Fedora 36+)

## Documentation Strategy

1. **README.md**: Quick start and installation
2. **DESIGN.md**: This document
3. **User Guide**: Command reference and examples
4. **Contributing Guide**: Development setup
5. **Code Comments**: Inline documentation
6. **Example Projects**: Sample games using crank

## License

MIT License for maximum compatibility and adoption.

