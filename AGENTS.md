# AGENTS.md: Agentic Coding Guidance for rust-shell

This document contains comprehensive instructions for agents working in this repository, covering build, lint, and test commands along with explicit code style guidelines. Follow these best practices to ensure code consistency, correctness, and high maintainability.

---

## 1. Build, Lint, and Test Commands

### 1.1 Build
- To build the project:
  ```bash
  cargo build
  ```
- To build in release mode:
  ```bash
  cargo build --release
  ```

### 1.2 Lint (Rustfmt and Clippy)
- **Formatting (rustfmt):**
  ```bash
  cargo fmt
  ```
- **Linting (clippy):**
  ```bash
  cargo clippy
  ```
- **Common clippy usage:**
  - `cargo clippy --all-targets --all-features -- -D warnings` to treat warnings as errors.

### 1.3 Test
- To run all tests:
  ```bash
  cargo test
  ```
- To run a single test function:
  ```bash
  cargo test <testname>
  ```
  - `<testname>` may be the name of the test (or a substring)
  - Example: `cargo test test_cd_home`
- To run a single test in an integration test file:
  ```bash
  cargo test --test <integration_test_file> -- <testname>
  ```
- To see test output:
  ```bash
  cargo test -- --nocapture
  ```

- **Building and running the program:**
  - The shell executable can be built and run directly via:
    ```bash
    cargo run
    ./your_program.sh
    ```

---

## 2. Code Style Guidelines

### 2.1 Imports
- Standard library imports should precede external crates.
- Group related imports together:
  ```rust
  use std::io::{self, Write};
  use std::fs;
  use anyhow::{Result, Error};
  use crate::commands::command_type::{LineCommand, OutputType};
  ```
- Prefer explicit import paths (avoid glob imports):
  ```rust
  use crate::commands::builtin::exec_builtin;
  ```

### 2.2 Formatting
- Use `cargo fmt` to enforce style.
- Indent with 4 spaces; never use tabs.
- Max line length: 100 chars, unless readability dictates otherwise.
- Braces (`{}`) always open on the same line for functions, loops, conditionals.
- Prefer trailing commas on multi-line enum and struct definitions.
- Annotate modules using `mod`, usually one per line.

### 2.3 Types and Structs
- Use Rust's type system to enforce safety and clarity:
  - Preferred: `Option<T>`, `Result<T, E>`, `Vec<T>`, `String`, `&str`.
- Named structs over tuples for clarity:
  ```rust
  pub struct LineCommand {
      pub file_path: Option<String>,
      pub type_of: CommandType,
      pub executable: String,
      pub args: Option<Vec<String>>,
      pub params: Option<Vec<String>>,
  }
  ```
- Document public structs and enums. Example:
  ```rust
  /// Represents the shell command to execute
  pub struct LineCommand { ... }
  ```

### 2.4 Naming Conventions
- Modules: `snake_case`. Files must match module naming.
- Types, Structs, Enums: `CamelCase`.
- Functions and Variables: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.

### 2.5 Error Handling
- Prefer returning `Result<T, E>` for fallible operations.
- When using error crates (such as `anyhow` or `thiserror`),
  - Use context (`.context()` or `.map_err(...)`) to enrich errors.
- Avoid panics except in unreachable/critical cases.
- Prefer `unwrap_or_default()` or pattern matching over bare `.unwrap()`.
- For IO and external processes, handle errors gracefully and relay meaningful error messages.

### 2.6 Enums and Match Patterns
- Always handle all enum cases explicitly.
- Use default or wildcard patterns only if truly exhaustive cases exist.
- Example:
  ```rust
  match result.output_type {
      OutputType::Str => { ... }
      OutputType::Vec => { ... }
      OutputType::None => {}
  }
  ```

### 2.7 Comments and Documentation
- Use Rustdoc (`///`) for public APIs, structs, functions.
- Inline comments (`//`) should clarify non-obvious logic.
- Document assumptions, edge cases, error handling.
- Keep documentation up to date with changes.

### 2.8 Tests
- Place unit tests inline under `#[cfg(test)] mod tests {}` at the bottom of files.
- Integration tests go in `/tests` directory (if added).
- Name tests descriptive and relevant to tested behavior.
- Example:
  ```rust
  #[test]
  fn test_cd_home() { ... }
  ```
- Run single tests via `cargo test <testname>`.

### 2.9 Miscellaneous
- Keep public API stable and minimal.
- Use `pub(crate)` where possible over `pub` to limit scope.
- Use `#[derive(Debug, Clone, PartialEq, Eq)]` for types used in tests or logging.
- For shell behavior rules: Follow POSIX conventions (quote handling, escapes, output, etc.).
---

## 3. Editor & Tooling
- No explicit Cursor or Copilot rules detected; agents should default to Rust standard practices.
- If you add `.editorconfig`, `rustfmt.toml` or `.github/copilot-instructions.md`, update this file with all explicit rules.

## 4. Agent Workflow Guidance
- **Always run tests and lint before proposing changes.**
- **If project structure or conventions are unclear, prefer upstream Rust conventions until local standards are established.**
- Add new standards to this file if the team introduces new patterns or preferences.

---

_This file is maintained for agentic automation and human contributors alike. Update regularly to reflect current project conventions._
