# {{PROJECT_NAME}}

A Playdate game created with crank.

## Development

Build the game:
```
crank build
```

Run in simulator:
```
crank run
```

Watch for changes:
```
crank watch
```

## Project Structure

- `source/` - Game source code
  - `main.lua` - Game entry point
  - `pdxinfo` - Game metadata
  - `.luarc.json` - Lua Language Server configuration
- `assets/` - Game assets (images, sounds, fonts)
- `tests/` - Test files
- `playdate-luacats/` - Type definitions for IDE support (optional)
- `build/` - Build output (generated)

## IDE Support

This project includes:
- `.luarc.json` - Configuration for Lua Language Server (sumneko.lua extension in VS Code)
- `playdate-luacats/` - Type definitions for the Playdate SDK API

These provide autocomplete, type checking, and inline documentation for Playdate APIs in your IDE.

