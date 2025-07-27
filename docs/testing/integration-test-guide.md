# Integration Test Guide for tmuxrs

This comprehensive guide covers the integration testing strategy for tmuxrs, including architecture, best practices, and troubleshooting.

## Overview

tmuxrs uses a sophisticated integration testing approach that combines:
- **Isolated tmux servers** for each test to prevent interference
- **Docker-based environments** for consistent, reproducible testing
- **TTY-safe testing patterns** that work reliably in CI/CD environments
- **Modular test organization** for better maintainability

## Architecture

### Test Isolation Design

Each integration test gets its own isolated tmux server with a unique socket path:

```rust
let session = TmuxTestSession::new("test-name");
// Creates isolated tmux server at unique socket path
// No interference with other tests or system tmux
```

Benefits:
- **Parallel Execution**: Tests can run concurrently without conflicts
- **Clean State**: Each test starts with a fresh tmux environment
- **Automatic Cleanup**: Drop trait ensures no artifacts remain
- **CI/CD Friendly**: Reliable in containerized environments

### TmuxTestSession Architecture

```rust
pub struct TmuxTestSession {
    session_name: String,      // Unique test session name
    temp_dir: Option<TempDir>, // Optional temporary directory
    socket_path: PathBuf,      // Unique tmux socket path
    socket_dir: TempDir,       // Temporary directory for socket
}
```

Key features:
- **Unique Naming**: Process ID + counter ensures globally unique names
- **Short Socket Paths**: Avoids path length limitations on some systems
- **Automatic Cleanup**: Drop implementation kills isolated tmux server
- **Temp Directory Support**: Optional temporary directories for config testing

## Environment Setup

### Docker Environment

The integration test environment is defined in `Dockerfile.test`:

```dockerfile
FROM rust:1.80

# Install tmux and terminal utilities
RUN apt-get update && apt-get install -y \
    tmux bash zsh procps git screen xterm

# Create non-root user for testing
RUN useradd -m -s /bin/bash testuser

# Set environment variables
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1
ENV TERM=xterm-256color
ENV TMUX_TMPDIR=/tmp
```

### Docker Compose Configuration

```yaml
services:
  integration-tests:
    build:
      context: .
      dockerfile: Dockerfile.test
    environment:
      - INTEGRATION_TESTS=1    # Required flag
      - RUST_LOG=debug
      - RUST_BACKTRACE=1
      - TERM=xterm-256color
      - TMUX_TMPDIR=/tmp
      - CARGO_TARGET_DIR=/tmp/target
    tty: false                 # Disabled for TTY-safe testing
    stdin_open: false
```

### Environment Variables

| Variable | Purpose | Required |
|----------|---------|----------|
| `INTEGRATION_TESTS=1` | Enables integration tests | Yes |
| `RUST_LOG=debug` | Enables debug logging | No |
| `RUST_BACKTRACE=1` | Shows stack traces | No |
| `TERM=xterm-256color` | Terminal type | No |
| `TMUX_TMPDIR=/tmp` | tmux temporary directory | No |

## Running Tests

### Quick Reference

```bash
# Unit tests only (fast, no tmux required)
cargo test

# All integration tests (Docker)
docker compose run --rm integration-tests

# Specific test module
docker compose run --rm integration-tests bash -c "cargo test cli"

# Single test with verbose output
docker compose run --rm integration-tests bash -c "cargo test test_name -- --nocapture"

# Clean rebuild
docker compose build --no-cache
```

### Test Categories

#### CLI Module (`cli/`)
```bash
cargo test --test integration_tests cli
```
Tests: 12 tests covering command-line interface, argument parsing, core workflows

#### Session Module (`session/`)
```bash
cargo test --test integration_tests session
```
Tests: 13 tests covering session lifecycle, attachment behavior, configuration

#### Window Module (`window/`)
```bash
cargo test --test integration_tests window
```
Tests: 8 tests covering window creation, splitting, layout management

#### Shell Module (`shell/`)
```bash
cargo test --test integration_tests shell
```
Tests: 7 tests covering environment inheritance, interactive features

#### tmux Module (`tmux/`)
```bash
cargo test --test integration_tests tmux
```
Tests: 7 tests covering low-level tmux operations, server isolation

## Test Categories Deep Dive

### CLI Tests
- **Help Commands**: Verify CLI help displays correctly
- **Argument Parsing**: Test flag combinations (--attach, --no-attach, --append)
- **Command Workflows**: End-to-end start/stop/list operations
- **Configuration Integration**: Config directory handling and session detection

### Session Tests  
- **Lifecycle Management**: Creation, existence checking, destruction
- **Attachment Behavior**: TTY-safe testing of attach logic
- **Configuration Loading**: YAML parsing, directory detection
- **Error Handling**: Invalid configs, missing sessions

### Window Tests
- **Window Creation**: Single and multi-window sessions
- **Layout Operations**: Horizontal/vertical splitting
- **Layout Selection**: main-horizontal, main-vertical, tiled layouts
- **Config-driven Creation**: Complex window configurations

### Shell Tests
- **Environment Inheritance**: Variable passing between sessions
- **Shell Initialization**: Profile and rc file execution
- **Interactive Features**: Aliases, functions, command history
- **State Independence**: Multi-window shell isolation

### tmux Tests
- **Command Execution**: Basic tmux operation verification
- **Server Isolation**: Unique socket handling
- **Error Scenarios**: Invalid commands, missing sessions
- **Multi-operation Workflows**: Complex command sequences

## TTY and Attach Handling

### The Challenge

Traditional tmux attach tests fail in Docker/CI environments because:
- No TTY available for interactive attachment
- `tmux attach` blocks waiting for terminal input
- Tests hang indefinitely in non-interactive environments

### Our Solution

Instead of testing actual attachment, we test the **behavior and setup** that precedes attachment:

```rust
// ❌ DON'T: This hangs in Docker
TmuxCommand::attach_session_with_socket(session_name, socket_path)?;

// ✅ DO: Test session readiness without blocking
let exists = TmuxCommand::session_exists_with_socket(session_name, socket_path)?;
assert!(exists, "Session should be ready for attachment");

// ✅ DO: Test session interaction capability
TmuxCommand::send_keys_with_socket(
    session_name, 
    "window_name", 
    "echo 'session active'", 
    socket_path
)?;
```

### Attach Test Patterns

#### Pattern 1: Verify Session Readiness
```rust
// Create session
session.create()?;

// Verify it exists and is ready for attachment  
let exists = session.exists()?;
assert!(exists, "Session should be ready for attachment");

// Test interaction capability (proves session is functional)
session.send_keys("0", "echo 'test command'")?;
```

#### Pattern 2: Test Existing Session Logic
```rust
// Create session first
session_manager.start_session_with_options(name, config_dir, false, false)?;

// Test "already exists" behavior  
let result = session_manager.start_session_with_options(name, config_dir, false, false)?;
assert!(result.contains("already exists"));
```

#### Pattern 3: Mock Attach Scenarios
```rust
// Test what would happen with attach=true but use attach=false
match session_manager.start_session_with_options(name, config_dir, false, false) {
    Ok(msg) => {
        // Session created successfully, would be ready for attach
        assert!(msg.contains("detached") || msg.contains("Started"));
    }
    Err(e) => {
        // Handle configuration or setup errors
        panic!("Setup failed: {}", e);
    }
}
```

## Writing Integration Tests

### Best Practices

#### 1. Use TmuxTestSession for Isolation
```rust
#[test]
fn test_my_feature() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("my-feature");
    // Test uses isolated tmux server automatically
    
    // No manual cleanup needed - Drop trait handles it
}
```

#### 2. Create Temporary Configs When Needed
```rust
let session = TmuxTestSession::with_temp_dir("config-test");
let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
std::fs::create_dir_all(&config_dir).unwrap();

let config_file = config_dir.join("test.yml");
std::fs::write(&config_file, yaml_content).unwrap();
```

#### 3. Use SessionManager with Socket
```rust
let session_manager = SessionManager::with_socket(session.socket_path());
let result = session_manager.start_session_with_options(
    Some(session.name()),
    Some(&config_dir),
    false, // attach = false (TTY-safe)
    false, // append = false
)?;
```

#### 4. Test Multiple Scenarios
```rust
// Test successful case
assert!(result.is_ok(), "Operation should succeed: {:?}", result);

// Test error case
let error_result = session_manager.stop_session("nonexistent");
assert!(error_result.is_err(), "Should fail for nonexistent session");
```

### Anti-patterns to Avoid

#### ❌ Don't Use Global tmux State
```rust
// BAD: Uses system tmux, can interfere with other tests
TmuxCommand::new_session("test-session", &PathBuf::from("."))?;
```

#### ❌ Don't Block on Attach
```rust
// BAD: Hangs in Docker environments
TmuxCommand::attach_session("session-name")?;
```

#### ❌ Don't Skip Environment Check
```rust
// BAD: Test will fail outside Docker
#[test]
fn test_without_env_check() {
    // Missing should_run_integration_tests() check
    let session = TmuxTestSession::new("test");
    // ...
}
```

#### ❌ Don't Manual Cleanup
```rust
// BAD: Manual cleanup is error-prone and unnecessary
#[test]
fn test_with_manual_cleanup() {
    let session = TmuxTestSession::new("test");
    // ... test logic ...
    session.kill().unwrap(); // Unnecessary - Drop handles this
}
```

## Performance Considerations

### Test Execution Times

- **Unit Tests**: ~50ms (no tmux dependency)
- **Integration Tests**: ~1-2s per test (tmux server creation)
- **Full Suite**: ~45s (45 tests in Docker)

### Optimization Strategies

1. **Parallel Test Execution**: Tests use isolated servers, safe for parallelization
2. **Minimal Docker Rebuilds**: Smart caching of Docker layers
3. **Efficient Socket Paths**: Short names to avoid filesystem overhead
4. **Cleanup Automation**: Drop trait eliminates manual cleanup delays

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Run Integration Tests
  run: docker compose run --rm integration-tests
  timeout-minutes: 10

# Check for test artifacts (should be none)
- name: Verify Cleanup
  run: |
    if [ -n "$(tmux list-sessions 2>/dev/null)" ]; then
      echo "ERROR: tmux sessions found after tests"
      exit 1
    fi
```

## Troubleshooting

### Common Issues

#### Test Hangs in Docker
**Symptom**: Tests hang indefinitely
**Cause**: Using blocking tmux operations
**Solution**: Use TTY-safe patterns, avoid direct attach

#### Permission Errors
**Symptom**: "Permission denied" when creating sockets
**Cause**: Docker volume mount permissions
**Solution**: Ensure proper user permissions in Dockerfile

#### Socket Path Too Long
**Symptom**: "No such file or directory" socket errors
**Cause**: Unix socket path length limits
**Solution**: Tests use short socket names (`s1`, `s2`, etc.)

#### Environment Variable Issues
**Symptom**: Integration tests skip unexpectedly  
**Cause**: Missing `INTEGRATION_TESTS=1`
**Solution**: Use Docker Compose which sets it automatically

### Debug Commands

```bash
# Check test environment
docker compose run --rm integration-tests bash -c "env | grep -E '(INTEGRATION|TMUX|TERM)'"

# Verify tmux installation
docker compose run --rm integration-tests bash -c "tmux -V"

# List running tests
docker compose run --rm integration-tests bash -c "cargo test --list"

# Run with maximum verbosity
docker compose run --rm integration-tests bash -c "RUST_LOG=trace cargo test test_name -- --nocapture"

# Check for leftover processes
docker compose run --rm integration-tests bash -c "ps aux | grep tmux"
```

### Development Workflow

#### 1. Write Test Locally
```bash
# Start with unit test to verify logic
cargo test my_new_feature_unit_test

# Write integration test  
# Add to appropriate module (cli/, session/, etc.)
```

#### 2. Test in Docker
```bash
# Run new test in isolation
docker compose run --rm integration-tests bash -c "cargo test my_new_feature -- --nocapture"

# Run related test suite
docker compose run --rm integration-tests bash -c "cargo test session"
```

#### 3. Verify CI Compatibility
```bash
# Full test suite
docker compose run --rm integration-tests

# Check for cleanup issues
docker compose run --rm integration-tests bash -c "cargo test && tmux list-sessions 2>/dev/null | wc -l"
```

## Integration with Unit Tests

### Clear Separation

- **Unit Tests**: Located in `#[cfg(test)]` modules within source files
- **Integration Tests**: Located in `tests/` directory with `INTEGRATION_TESTS` gate
- **No Overlap**: Unit tests never use TmuxCommand, integration tests focus on behavior

### When to Use Each

| Test Type | Use When | Example |
|-----------|----------|---------|
| Unit | Testing logic, parsing, validation | Config YAML parsing |
| Integration | Testing tmux interaction, end-to-end workflows | Session creation |

### Example Unit Test
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let yaml = "name: test\nroot: /tmp\nwindows:\n  - main: echo hello";
        let config = Config::parse_yaml(yaml).unwrap();
        assert_eq!(config.name, "test");
    }
}
```

### Example Integration Test
```rust
#[test]
fn test_session_creation() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use Docker");
        return;
    }

    let session = TmuxTestSession::new("creation-test");
    let result = session.create();
    assert!(result.is_ok(), "Session creation should succeed");
    
    let exists = session.exists().unwrap();
    assert!(exists, "Created session should exist");
}
```

## Maintenance and Evolution

### Adding New Tests

1. **Choose Module**: Determine appropriate test module (cli/, session/, etc.)
2. **Follow Patterns**: Use existing tests as templates
3. **Use TmuxTestSession**: Always use isolated test sessions
4. **Avoid TTY Operations**: Use headless testing patterns
5. **Test in Docker**: Verify compatibility before committing

### Updating Existing Tests

1. **Preserve Intent**: Maintain original test purpose
2. **Improve Reliability**: Migrate away from global state or TTY dependencies
3. **Update Documentation**: Keep README and guides current
4. **Verify Compatibility**: Ensure changes work across all environments

### Future Enhancements

- **Performance Optimization**: Reduce test execution time
- **Enhanced Isolation**: Further improve test independence
- **Better Error Messages**: More helpful test failure diagnostics
- **Extended Coverage**: Additional edge cases and error conditions

This guide provides the foundation for reliable, maintainable integration testing of tmuxrs. The key is combining isolated test infrastructure with TTY-safe testing patterns to achieve consistent results across all environments.