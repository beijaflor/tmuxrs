# tmuxrs

## Core Innovation
- **Problem**: tmuxinator requires scattered config files or manual project specification
- **Solution**: Centralized configs (`~/.config/tmuxrs/`) with directory-aware execution
- **Key Feature**: `tmuxrs start` in any project directory auto-detects correct config

## Tech Stack
- Rust (latest stable)
- YAML parsing: serde_yaml
- CLI framework: clap  
- Error handling: thiserror
- Target: tmux 2.0+

## Project Architecture
```
src/
├── main.rs + cli.rs     # Command dispatch
├── config/              # YAML discovery and parsing
├── session/             # Session lifecycle management  
├── tmux/                # Direct tmux command execution
└── error.rs             # Custom error types
```

## Essential Commands
- `cargo build` - Build the project
- `cargo test` - Run unit tests only (integration tests automatically skip)
- `cargo run -- start test` - Test session creation
- `cargo clippy` - Lint code
- `cargo fmt` - Format code

### Integration Tests (Docker Only)
- `docker-compose run --rm integration-tests` - Run all tests including integration tests
- `docker-compose run --rm integration-tests cargo test TEST_NAME` - Run specific test
- `docker-compose build` - Build Docker test image
- `docker-compose down --volumes` - Clean up containers and volumes

## MVP Implementation Order
1. **CLI structure** (`clap` argument parsing)
2. **Config discovery** (directory basename detection)
3. **YAML parsing** (serde_yaml for tmuxinator format)
4. **Tmux commands** (direct execution, no script generation)
5. **Session management** (create/attach/kill)

## Configuration Discovery Logic
1. Look for `./.tmuxinator.yml` in current directory
2. Use directory basename to find `~/.config/tmuxrs/{basename}.yml`
3. Manual override: `tmuxrs start <name>` uses `~/.config/tmuxrs/<name>.yml`

## Code Patterns

### Session Management
- Always check `tmux has-session -t <name>` before creating
- Use `tmux attach-session` if session exists 
- Use `tmux new-session -d` for creation

### Error Handling
- Use `thiserror` for custom error types
- Return `Result<T, TmuxrsError>` from all fallible functions
- Provide specific error messages with context

### YAML Structure (tmuxinator-compatible)
```yaml
name: project_name
root: ~/path/to/project
windows:
  - editor: vim
  - server: 
      layout: main-vertical
      command: npm start
```

## MVP Scope
- **Commands**: `list`, `start <name>`, `stop <name>`
- **YAML support**: name, root, windows with basic layouts
- **Single pane per window** (multi-pane in future phases)
- **No hooks** (lifecycle hooks in future phases)

## Code Style
- 4-space indentation (Rust standard)
- Follow Rust naming conventions (snake_case, PascalCase)
- Prefer `Result<T, Error>` for error handling
- Write unit tests for all public functions
- Use descriptive error messages with context

## Key Constraints
- **Tmuxinator compatibility**: Must parse existing YAML files correctly
- **Performance target**: Faster session creation than tmuxinator
- **Direct tmux execution**: No shell script generation (simpler than tmuxinator)
- **MVP first**: Implement minimal features well before adding complexity

## Testing Strategy
- Unit tests for config parsing and discovery
- Integration tests with actual tmux sessions
- Shell interaction tests to ensure proper environment inheritance
- Test error cases (missing configs, invalid YAML)
- Manual testing checklist for session lifecycle

### Integration Testing
- **Automatic Skipping**: Integration tests always skip with `cargo test`
- **Files**: All tests in `tests/` directory that use `TmuxCommand` or `SessionManager`
- **Enable Flag**: Integration tests only run when `INTEGRATION_TESTS=1` is set
- **Commands**: Use `docker-compose run --rm integration-tests` to run integration tests
- **Skip Logic**: Tests run only when `INTEGRATION_TESTS=1` environment variable is set

### Docker Test Environment
- **Files**: `Dockerfile.test`, `compose.yml`
- **Purpose**: Provides isolated, reproducible test environment for integration tests
- **Features**:
  - Clean tmux environment for each test run
  - Consistent Rust toolchain (1.80) and tmux version (3.3a)
  - Shell interaction tests that were previously skipped in CI
  - No test artifacts or sessions persist between runs
- **Environment Variables**: 
  - `INTEGRATION_TESTS=1` - Required for integration tests to run (automatically set in Docker)

## Documentation References
- Architecture details: `docs/design/02-system-architecture.md`
- Feature decisions: `docs/design/01-feature-decisions.md`
- Implementation guide: `docs/guides/01-implementation-guide.md`
- Project roadmap: `docs/ROADMAP.md`