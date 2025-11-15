# crank

A command-line build tool for Playdate game development, inspired by Cargo. Simplify your Playdate development workflow with project scaffolding, build automation, and development tools.

## Features

- **Project Scaffolding**: Create new Playdate projects with sensible defaults
- **Build Automation**: Wrap the Playdate SDK compiler with a simple interface
- **Run in Simulator**: Build and launch your game in one command
- **Hot Reload**: Watch for changes and rebuild automatically
- **Testing Support**: Run tests with LuaUnit integration
- **Cross-Platform**: Works on Windows, Linux, and macOS
- **Multi-Language**: Lua support now, Swift planned for the future

## Installation

### From Source

```bash
git clone https://github.com/subpop/crank.git
cd crank
cargo install --path .
```

### From Cargo (coming soon)

```bash
cargo install crank
```

## Prerequisites

- [Playdate SDK](https://play.date/dev/) installed and configured
- Set `PLAYDATE_SDK_PATH` environment variable (optional, crank will try to detect it)

## Quick Start

### Create a New Project

```bash
crank new my-awesome-game
cd my-awesome-game
```

This creates a new project with the following structure:

```
my-awesome-game/
├── .gitignore
├── .luarc.json
├── Playdate.toml
├── source/
│   └── main.lua
├── assets/
│   ├── images/
│   ├── sounds/
│   └── fonts/
├── playdate-luacats/   (Lua LSP type definitions)
└── tests/
    ├── luaunit.lua     (Testing framework)
    └── test_basic.lua  (Example tests)
```

### Build Your Game

```bash
crank build
```

This compiles your game using the Playdate SDK's `pdc` compiler and outputs to `build/my-awesome-game.pdx`.

### Run in Simulator

```bash
crank run
```

This builds your game and launches it in the Playdate Simulator.

### Watch for Changes

```bash
crank watch
```

This watches your source files and assets, automatically rebuilding when changes are detected.

### Run Tests

```bash
crank test
```

Runs all test files matching `test_*.lua` or `*_test.lua` in the `tests/` directory using [LuaUnit](https://github.com/bluebird75/luaunit), a popular xUnit-style testing framework.

**Features:**
- Automatic test discovery
- Comprehensive assertion library
- Colorized output
- Test filtering with `--filter`
- CI/CD ready with standard exit codes

**Requirements:** A system Lua interpreter (lua5.4, lua, or luajit) must be installed.

## Configuration

Projects are configured via `Playdate.toml`:

```toml
[package]
name = "my-awesome-game"
version = "1.0.0"
author = "Your Name"
description = "An awesome Playdate game"
bundle_id = "com.example.myawesomegame"

[build]
language = "lua"
source_dir = "src"
output_dir = "build"
assets_dir = "assets"

[playdate]
content_warning = ""
content_warning_2 = ""
image_path = "icon"

[dev]
hot_reload = true
auto_build = true
```

## Commands

### `crank new <name> [path]`

Create a new Playdate project.

**Arguments:**
- `<name>`: Name of the project (required)
- `[path]`: Directory where the project will be created (optional, defaults to current directory)

**Options:**
- `--template <template>`: Choose a project template (default: lua-basic)

**Examples:**
```bash
# Create project in current directory
crank new my-game

# Create project in a specific location
crank new my-game ~/projects

# Create project with custom template
crank new my-game --template lua-basic
```

### `crank build`

Build the current project using the Playdate SDK's `pdc` compiler.

The build process:
1. Validates project structure and configuration
2. Detects Playdate SDK installation
3. Compiles source code to `.pdx` bundle
4. Reports build time and bundle size

**Options:**
- `--release, -r`: Build in release mode (currently same as debug, reserved for future optimizations)
- `--verbose, -v`: Show detailed build output including SDK paths and compilation details

**Examples:**
```bash
# Basic build
crank build

# Verbose output (helpful for debugging)
crank build --verbose

# Release build
crank build --release

# Combined options
crank build -rv
```

**Output:**
```
Building Playdate project (debug mode)...

✓ Built successfully in 1.23s

  Output: build/my-game.pdx
  Size: 245 KB

Next steps:
  crank run
```

### `crank run`

Build and run the project in the Playdate Simulator in a single command.

The run process:
1. Builds the project (same as `crank build`)
2. Validates the build output
3. Launches the Playdate Simulator
4. Loads your game automatically

**Options:**
- `--release, -r`: Build and run in release mode

**Examples:**
```bash
# Basic run (debug mode)
crank run

# Release mode
crank run --release
```

**Output:**
```
════════════════════════════════════════════════════════
▶ Build and Run
════════════════════════════════════════════════════════

→ Building project...

Building Playdate project (debug mode)...

✓ Built successfully in 1.23s

  Output: build/my-game.pdx
  Size: 245 KB

────────────────────────────────────────────────────────

→ Launching Playdate Simulator...

  Game: my-game
  Bundle: build/my-game.pdx
  Simulator: ~/Developer/PlaydateSDK/bin/Playdate Simulator.app

✓ Simulator launched successfully!

The game is now running in the Playdate Simulator.
```

### `crank watch`

Watch for changes and rebuild automatically.

**Options:**
- `--no-run`: Don't launch simulator, just rebuild

**Example:**
```bash
crank watch
```

### `crank test`

Run tests in the `tests/` directory.

**Options:**
- `--filter <pattern>`: Run only tests matching pattern

**Example:**
```bash
crank test --filter player
```

### `crank clean`

Remove build artifacts.

**Example:**
```bash
crank clean
```

## Project Structure

A typical crank project:

```
my-game/
├── Playdate.toml          # Project configuration
├── source/                # Source code
│   ├── main.lua          # Entry point
│   ├── pdxinfo           # Game metadata
│   ├── .luarc.json       # Lua LSP configuration
│   ├── player.lua        # Game modules
│   └── enemies.lua
├── assets/               # Game assets
│   ├── images/           # Images and sprites
│   │   ├── player.png
│   │   └── background.png
│   ├── sounds/           # Audio files
│   │   └── jump.wav
│   └── fonts/            # Custom fonts
│       └── game-font.fnt
├── playdate-luacats/      # Lua type definitions for IDE
├── tests/                # Test files
│   ├── luaunit.lua       # LuaUnit testing framework
│   ├── test_basic.lua
│   ├── test_player.lua
│   └── test_enemies.lua
└── build/                # Build output (generated)
    └── my-game.pdx/      # Playdate executable
```

## SDK Detection

crank automatically detects your Playdate SDK installation in the following order:

1. `PLAYDATE_SDK_PATH` environment variable
2. `simulator_path` in `Playdate.toml`
3. Platform-specific default locations:
   - **macOS**: `~/Developer/PlaydateSDK`
   - **Windows**: `%USERPROFILE%\Documents\PlaydateSDK`
   - **Linux**: `~/PlaydateSDK`

## IDE Support

New projects include IDE support out of the box:

- **`.luarc.json`** - Configuration for Lua Language Server
  - Enables autocomplete for Playdate APIs
  - Provides inline documentation
  - Type checking and diagnostics
  - Works with VS Code's Lua extension (sumneko.lua)

- **`playdate-luacats/`** - Type definitions from [notpeter/playdate-luacats](https://github.com/notpeter/playdate-luacats)
  - Comprehensive type annotations for Playdate SDK
  - Automatically cloned during project creation
  - Improves IDE intelligence and catches errors early

- **Recommended VS Code Extensions**
  - [Lua by sumneko](https://open-vsx.org/extension/sumneko/lua)
  - [Playdate Debug by midouest](https://open-vsx.org/extension/midouest/playdate-debug)

## Testing

New projects include [LuaUnit](https://github.com/bluebird75/luaunit) testing framework for writing and running unit tests.

### Writing Tests

Create test files in the `tests/` directory following naming conventions:
- `test_*.lua` (e.g., `test_player.lua`)
- `*_test.lua` (e.g., `player_test.lua`)

Example test file:

```lua
local luaunit = require('luaunit')

function testBasicAssertion()
    luaunit.assertEquals(1 + 1, 2)
end

function testStringOperations()
    local str = "Hello, Playdate!"
    luaunit.assertIsString(str)
    luaunit.assertTrue(#str > 0)
end

-- Run all tests
os.exit(luaunit.LuaUnit.run())
```

### Running Tests

```bash
# Run all tests
crank test

# Run tests matching a pattern
crank test --filter player
```

**Requirements**: You need a system Lua interpreter installed (lua5.4, lua, or luajit).

Installation:
- **macOS**: `brew install lua`
- **Ubuntu/Debian**: `apt-get install lua5.4`
- **Windows**: Download from [luabinaries.sourceforge.net](https://luabinaries.sourceforge.net/)

### Available Assertions

LuaUnit provides a comprehensive set of assertions:

```lua
-- Equality
luaunit.assertEquals(actual, expected)
luaunit.assertNotEquals(actual, expected)

-- Boolean
luaunit.assertTrue(value)
luaunit.assertFalse(value)

-- Nil checks
luaunit.assertNil(value)
luaunit.assertNotNil(value)

-- Type checks
luaunit.assertIsString(value)
luaunit.assertIsNumber(value)
luaunit.assertIsTable(value)

-- Numeric comparisons
luaunit.assertAlmostEquals(actual, expected, margin)
```

See [TEST_COMMAND_IMPLEMENTATION.md](TEST_COMMAND_IMPLEMENTATION.md) for comprehensive testing documentation.

## Development

### Building from Source

```bash
git clone https://github.com/subpop/crank.git
cd crank
cargo build --release
```

The binary will be at `target/release/crank`.

### Running Tests

```bash
cargo test
```

## Roadmap

- [x] Project creation
- [x] Build system
- [x] Run in simulator
- [x] Watch mode with hot reload
- [x] Test runner with LuaUnit
- [ ] Swift language support
- [ ] Asset processing pipeline
- [ ] Dependency management

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Inspired by [Cargo](https://doc.rust-lang.org/cargo/) for Rust
- Built for the [Playdate](https://play.date/) handheld gaming system by Panic Inc.

## Resources

- [Playdate Developer Site](https://play.date/dev/)
- [Playdate SDK Documentation](https://sdk.play.date/)
- [Issue Tracker](https://github.com/subpop/crank/issues)
- [Discussions](https://github.com/subpop/crank/discussions)

---

Made with ❤️ for the Playdate community

