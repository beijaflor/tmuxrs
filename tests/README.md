# Integration Test Organization

This directory contains the reorganized integration test suite for tmuxrs, structured for better maintainability and organization. All tests use isolated tmux servers and run reliably in Docker environments.

## Structure

```
tests/
├── integration_tests.rs     # Main test entry point
├── common/                  # Shared test utilities and infrastructure
│   └── mod.rs              # TmuxTestSession, skip macros, helpers
├── cli/                    # CLI interface and core command tests
│   └── mod.rs              # Help commands, argument parsing, core workflows
├── session/                # Session lifecycle management tests
│   └── mod.rs              # Creation, existence, attachment, stopping
├── window/                 # Window and layout management tests
│   └── mod.rs              # Window creation, splitting, layout selection
├── shell/                  # Shell integration and environment tests
│   └── mod.rs              # Environment vars, aliases, command execution
└── tmux/                   # Low-level tmux operations tests
    └── mod.rs              # Direct tmux command execution, server isolation
```

## Test Categories

### CLI Module (`cli/`)
- **Help and Interface**: CLI help display, command existence verification
- **Argument Parsing**: Flag parsing (--attach, --no-attach, --append)
- **Core Workflows**: Start/stop/list commands with configuration

### Session Module (`session/`)
- **Lifecycle Management**: Session creation, existence checking, destruction
- **Attachment**: Session attachment behavior testing using headless operations 
- **Configuration**: Directory detection, config loading, session naming

### Window Module (`window/`)
- **Window Management**: Window creation within sessions
- **Layout Operations**: Splitting (horizontal/vertical), layout selection
- **SessionManager Integration**: Config-driven window creation

### Shell Module (`shell/`)
- **Environment**: Variable inheritance, shell initialization
- **Interactive Features**: Aliases, functions, command execution
- **Independence**: Multi-window shell state isolation
- **Compatibility**: Operation without custom shell configuration

### tmux Module (`tmux/`)
- **Command Execution**: Basic tmux command building and execution
- **Server Isolation**: Isolated tmux server operations for test independence
- **Error Handling**: Proper error handling for various tmux scenarios
- **Multi-operation**: Complex workflows with multiple tmux operations

## Test Infrastructure

### TmuxTestSession (common/)
- **Isolation**: Each test gets its own tmux server with unique socket
- **Automatic Cleanup**: Drop trait ensures no test artifacts persist
- **Temp Directories**: Optional temporary directories for config testing
- **Socket Management**: Unique socket paths prevent test interference

### Integration Test Environment
- **Environment Flag**: Tests only run when `INTEGRATION_TESTS=1` is set
- **Docker Support**: Full test suite runs in Docker for CI/isolation
- **Skip Logic**: Tests automatically skip in inappropriate environments
- **TTY Handling**: Attach tests gracefully handle non-TTY environments

## Running Tests

### Unit Tests Only (Fast)
```bash
cargo test  # Automatically skips integration tests
```

### Integration Tests (Docker)
```bash
docker compose run --rm integration-tests  # Full test suite
```

### Specific Test Categories
```bash
# CLI tests only
cargo test --test integration_tests cli

# Session management tests
cargo test --test integration_tests session

# Window/layout tests  
cargo test --test integration_tests window

# Shell integration tests
cargo test --test integration_tests shell

# Low-level tmux tests
cargo test --test integration_tests tmux
```

## Test Status

- **Total Tests**: 45 integration tests across all modules
- **All Tests Active**: All tests now run successfully in Docker environments
- **No Ignored Tests**: Previously problematic attach tests have been refactored to work reliably
- **Docker Compatible**: All tests use isolated tmux servers and avoid TTY dependencies

## Benefits of Reorganization

1. **Better Organization**: Related tests are grouped logically
2. **Reduced Redundancy**: Eliminated duplicate tests across files
3. **Easier Navigation**: Clear module structure for finding relevant tests
4. **Maintainability**: Changes to functionality can update related tests together
5. **Parallel Development**: Teams can work on different test modules independently
6. **Documentation**: Clear categorization makes test purpose obvious
7. **Reliable CI/CD**: All tests work consistently in Docker environments
8. **Test Isolation**: Each test uses its own tmux server, preventing interference

## Docker Testing Approach

### TTY-Safe Testing
Our integration tests avoid TTY dependencies by:
- Using headless tmux operations (`send-keys`, session management)
- Testing attach behavior without actually attaching to terminals
- Verifying session readiness rather than interactive attachment
- Using isolated tmux servers with unique socket paths

### Test Isolation Benefits
- **No Test Interference**: Each test runs on its own tmux server
- **Parallel Execution**: Tests can run concurrently without conflicts
- **Automatic Cleanup**: TmuxTestSession Drop trait ensures no artifacts remain
- **Consistent Environment**: Docker provides reproducible test conditions

## Troubleshooting

### Common Issues
- **Permission Errors**: Ensure Docker has proper volume mount permissions
- **Test Timeouts**: Check if tmux is properly installed in test environment
- **Socket Path Length**: Tests use short socket names to avoid path limits
- **Environment Variables**: Ensure `INTEGRATION_TESTS=1` is set for Docker runs

### Debug Commands
```bash
# Run specific test with verbose output
docker compose run --rm integration-tests bash -c "cargo test test_name -- --nocapture"

# Check tmux availability in container
docker compose run --rm integration-tests bash -c "tmux -V"

# Verify test environment
docker compose run --rm integration-tests bash -c "env | grep -E '(INTEGRATION|TMUX|TERM)'"
```

## Migration Notes

All original test files have been consolidated into the new modular structure:
- `integration.rs` + `core_commands_test.rs` → `cli/mod.rs`
- Session-related tests → `session/mod.rs`  
- Window/layout tests → `window/mod.rs`
- Shell tests → `shell/mod.rs`
- Low-level tmux tests → `tmux/mod.rs`

Test functionality has been preserved while improving organization and removing duplication.