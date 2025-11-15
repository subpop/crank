//! Run tests for Playdate projects

use crate::error::{CrankError, Result};
use crate::project::Project;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// Execute the 'test' command
pub fn execute(filter: Option<&str>) -> Result<()> {
    println!("{} tests...", "Running".cyan().bold());

    let project = Project::find_and_load()?;
    let tests_dir = project.tests_dir();

    if !tests_dir.exists() {
        println!("{} No tests directory found", "⚠".yellow().bold());
        return Ok(());
    }

    // Check if Lua interpreter is available
    let lua_cmd = find_lua_interpreter();
    if lua_cmd.is_none() {
        println!("{} No Lua interpreter found", "⚠".yellow().bold());
        println!();
        println!("To run tests, you need a Lua interpreter installed.");
        println!();
        println!("Install options:");
        println!("  - macOS: brew install lua");
        println!("  - Ubuntu/Debian: apt-get install lua5.4");
        println!("  - Windows: Download from https://luabinaries.sourceforge.net/");
        println!();
        println!("Alternatively, you can:");
        println!("  1. Include test files in your main project");
        println!("  2. Run them using the Playdate Simulator");
        return Ok(());
    }

    let lua_cmd = lua_cmd.unwrap();

    // Discover test files
    let mut test_files = Vec::new();

    for entry in WalkDir::new(&tests_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Skip luaunit.lua itself
            if name == "luaunit.lua" {
                continue;
            }

            if (name.starts_with("test_") || name.ends_with("_test.lua")) && name.ends_with(".lua")
            {
                // Apply filter if provided
                if let Some(f) = filter {
                    if !name.contains(f) {
                        continue;
                    }
                }
                test_files.push(path.to_path_buf());
            }
        }
    }

    if test_files.is_empty() {
        println!("{} No test files found", "⚠".yellow().bold());
        println!();
        println!("Test files should match one of these patterns:");
        println!("  - test_*.lua");
        println!("  - *_test.lua");
        if filter.is_some() {
            println!();
            println!(
                "Note: Filter '{}' may have excluded some tests",
                filter.unwrap()
            );
        }
        return Ok(());
    }

    println!("Found {} test file(s)", test_files.len());
    println!();

    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut all_success = true;

    // Run each test file
    for test_file in &test_files {
        let name = test_file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        println!("{} {}", "→".cyan().bold(), name.bold());

        match run_test_file(&lua_cmd, test_file, &tests_dir) {
            Ok(result) => {
                if result.success {
                    total_passed += 1;
                } else {
                    total_failed += 1;
                    all_success = false;
                }
            }
            Err(e) => {
                println!("  {} Error running test: {}", "✗".red().bold(), e);
                total_failed += 1;
                all_success = false;
            }
        }

        println!();
    }

    // Print summary
    println!("{}", "=".repeat(50));
    println!();

    if all_success {
        println!(
            "{} All tests passed! ({} test file(s))",
            "✓".green().bold(),
            total_passed
        );
    } else {
        println!(
            "{} Some tests failed: {} passed, {} failed",
            "✗".red().bold(),
            total_passed,
            total_failed
        );
        return Err(CrankError::TestsFailed);
    }

    Ok(())
}

/// Find an available Lua interpreter
fn find_lua_interpreter() -> Option<String> {
    let candidates = vec!["lua5.4", "lua54", "lua", "luajit"];

    for cmd in candidates {
        if Command::new(cmd)
            .arg("-v")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return Some(cmd.to_string());
        }
    }

    None
}

/// Result of running a test file
struct TestResult {
    success: bool,
}

/// Run a single test file
fn run_test_file(
    lua_cmd: &str,
    test_file: &std::path::Path,
    tests_dir: &std::path::Path,
) -> Result<TestResult> {
    // Set LUA_PATH to include the tests directory so 'require' works
    let lua_path = format!("{}/?.lua;;", tests_dir.display());

    let mut cmd = Command::new(lua_cmd);
    cmd.arg(test_file)
        .current_dir(tests_dir)
        .env("LUA_PATH", lua_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    // Read and display output in real-time
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(|r| r.ok()) {
            // Colorize output
            if line.contains("✓") || line.contains("SUCCESS") {
                println!("  {}", line.green());
            } else if line.contains("✗") || line.contains("FAILURE") || line.contains("Error") {
                println!("  {}", line.red());
            } else if line.contains("Running") || line.contains("Ran") {
                println!("  {}", line.cyan());
            } else if line.starts_with("---") || line.starts_with("===") {
                println!("  {}", line.dimmed());
            } else {
                println!("  {}", line);
            }
        }
    }

    // Collect stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(|r| r.ok()) {
            // Only show stderr if it contains errors
            if !line.trim().is_empty() {
                println!("  {}", line.red());
            }
        }
    }

    let status = child.wait()?;

    Ok(TestResult {
        success: status.success(),
    })
}
