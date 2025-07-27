# Integration Test Troubleshooting Guide

This guide covers common issues encountered when running tmuxrs integration tests and their solutions.

## Quick Diagnostics

### Environment Check
```bash
# Verify Docker environment
docker compose run --rm integration-tests bash -c "env | grep -E '(INTEGRATION|TMUX|TERM)'"

# Expected output:
# INTEGRATION_TESTS=1
# TMUX_TMPDIR=/tmp
# TERM=xterm-256color
```

### tmux Availability
```bash
# Check tmux installation
docker compose run --rm integration-tests bash -c "tmux -V"

# Expected output: tmux 3.3a (or later)
```

### Test List Verification
```bash
# List all integration tests
docker compose run --rm integration-tests bash -c "cargo test -- --list 2>/dev/null | grep -E 'test ' | wc -l"

# Or run briefly to see test counts
docker compose run --rm integration-tests bash -c "timeout 10s cargo test 2>&1 | grep 'running.*tests'"

# Should show: running 45 tests (for integration tests)
```

## Common Issues and Solutions

### 1. Tests Hang Indefinitely

**Symptoms:**
- Test execution freezes without output
- Docker container appears to be running but no progress
- Tests timeout in CI/CD environments

**Causes:**
- TTY operations in non-TTY environment
- Blocking on `tmux attach` commands
- Infinite loops in test logic

**Solutions:**

#### Check for TTY Operations
```bash
# Look for problematic patterns in tests
grep -r "attach_session" tests/
grep -r "attach.*true" tests/

# These should use TTY-safe patterns instead
```

#### Debug Hanging Test
```bash
# Run single test with timeout
timeout 30 docker compose run --rm integration-tests bash -c "cargo test test_name -- --nocapture"

# Use strace to see system calls (if available)
docker compose run --rm integration-tests bash -c "strace -e trace=file cargo test test_name"
```

#### Fix: Use TTY-Safe Patterns
```rust
// ❌ DON'T: This can hang in Docker
TmuxCommand::attach_session_with_socket(session_name, socket_path)?;

// ✅ DO: Test session readiness instead
let exists = TmuxCommand::session_exists_with_socket(session_name, socket_path)?;
assert!(exists, "Session should be ready for attachment");
```

### 2. Permission Denied Errors

**Symptoms:**
```
Permission denied (os error 13)
Failed to create socket directory
Error creating tmux session
```

**Causes:**
- Docker volume mount permission issues
- Incorrect user permissions in container
- SELinux/AppArmor restrictions

**Solutions:**

#### Check Docker Setup
```bash
# Verify user in container
docker compose run --rm integration-tests bash -c "whoami && id"

# Should show: testuser uid=1001(testuser) gid=1001(testuser)
```

#### Fix Volume Permissions
```bash
# Option 1: Update Docker Compose (if needed)
# Add to compose.yml:
user: "1001:1001"

# Option 2: Fix local permissions
sudo chown -R $(id -u):$(id -g) .
```

#### Verify Socket Creation
```bash
# Test socket creation manually
docker compose run --rm integration-tests bash -c "
cd /tmp && 
mkdir test-socket && 
tmux -S /tmp/test-socket/s1 new-session -d 'sleep 1' &&
tmux -S /tmp/test-socket/s1 list-sessions &&
tmux -S /tmp/test-socket/s1 kill-server
"
```

### 3. Socket Path Too Long

**Symptoms:**
```
No such file or directory
error connecting to /very/long/path/to/socket (No such file or directory)
```

**Causes:**
- Unix socket path length limits (typically 108 characters)
- Deeply nested temporary directories

**Solutions:**

#### Verify Path Length
```bash
# Check current socket path length
docker compose run --rm integration-tests bash -c "
python3 -c 'import tempfile; print(len(tempfile.mkdtemp() + \"/socket_name\"))'
"
```

#### Use Short Socket Names
```rust
// ✅ Good: Short socket names
let socket_path = socket_dir.path().join("s1");

// ❌ Bad: Long descriptive names
let socket_path = socket_dir.path().join("test_session_with_very_long_name_socket");
```

### 4. Environment Variable Issues

**Symptoms:**
```
Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1
```

**Causes:**
- Missing `INTEGRATION_TESTS=1` environment variable
- Running tests outside Docker environment
- Incorrect Docker Compose configuration

**Solutions:**

#### Verify Environment Variable
```bash
# Check if variable is set
docker compose run --rm integration-tests bash -c "echo \$INTEGRATION_TESTS"

# Should output: 1
```

#### Manual Override (Not Recommended)
```bash
# Only for debugging - use Docker instead
INTEGRATION_TESTS=1 cargo test
```

#### Fix Docker Compose
```yaml
# Ensure compose.yml has:
environment:
  - INTEGRATION_TESTS=1
```

### 5. Test Interference / Flaky Tests

**Symptoms:**
- Tests pass individually but fail when run together
- Inconsistent test results
- "Session already exists" errors

**Causes:**
- Shared global state between tests
- Cleanup not working properly
- Race conditions in parallel execution

**Solutions:**

#### Check Test Isolation
```bash
# Run tests individually to identify interference
docker compose run --rm integration-tests bash -c "cargo test test1"
docker compose run --rm integration-tests bash -c "cargo test test2"

# vs. running together
docker compose run --rm integration-tests bash -c "cargo test test1 test2"
```

#### Verify Cleanup
```rust
// Ensure using TmuxTestSession (auto-cleanup)
let session = TmuxTestSession::new("test-name");
// Drop trait automatically cleans up

// ❌ Don't manually manage tmux sessions
```

#### Check for Global State
```bash
# Look for problematic patterns
grep -r "TmuxCommand::" tests/ | grep -v "with_socket"

# Should mostly use socket-based functions
```

### 6. tmux Version Compatibility

**Symptoms:**
```
tmux: unknown option: --some-flag
tmux: can't find session
```

**Causes:**
- Using features not available in older tmux versions
- Version-specific command syntax differences

**Solutions:**

#### Check tmux Version
```bash
docker compose run --rm integration-tests bash -c "tmux -V"
```

#### Update Dockerfile if Needed
```dockerfile
# Install specific tmux version
RUN apt-get update && apt-get install -y tmux=3.3a-3
```

#### Use Compatible Commands
```rust
// Use basic tmux commands that work across versions
TmuxCommand::new_session_with_socket(name, path, socket);
// Instead of newer flags or features
```

### 7. Cargo Build Issues in Docker

**Symptoms:**
```
error: could not find `Cargo.toml`
permission denied while trying to create target directory
```

**Causes:**
- Incorrect working directory in Docker
- Permission issues with target directory
- Volume mount configuration problems

**Solutions:**

#### Check Working Directory
```bash
# Verify correct mount and working dir
docker compose run --rm integration-tests bash -c "pwd && ls -la"

# Should show /app with Cargo.toml present
```

#### Fix Target Directory Permissions
```bash
# Clean and rebuild
docker compose run --rm integration-tests bash -c "rm -rf /tmp/target && cargo clean"
docker compose build --no-cache
```

#### Update Docker Compose
```yaml
volumes:
  - .:/app:rw  # Ensure read-write access
working_dir: /app
environment:
  - CARGO_TARGET_DIR=/tmp/target  # Use writable location
```

### 8. Memory or Resource Limits

**Symptoms:**
```
Cannot allocate memory
tmux: server failed to start
```

**Causes:**
- Docker container memory limits
- Too many concurrent tmux servers
- Resource exhaustion

**Solutions:**

#### Increase Docker Resources
```yaml
# In compose.yml
services:
  integration-tests:
    mem_limit: 2g
    memswap_limit: 2g
```

#### Check Resource Usage
```bash
# Monitor during test execution
docker compose run --rm integration-tests bash -c "
cargo test &
PID=\$!
while kill -0 \$PID 2>/dev/null; do
  ps aux | grep tmux
  sleep 1
done
"
```

#### Optimize Test Execution
```rust
// Ensure proper cleanup
impl Drop for TmuxTestSession {
    fn drop(&mut self) {
        // Kill entire server, not just session
        TmuxCommand::kill_server_with_socket(Some(&self.socket_path));
    }
}
```

## Debug Workflow

### Step 1: Basic Environment Check
```bash
# Quick verification
docker compose run --rm integration-tests bash -c "
echo 'Environment:' && env | grep -E '(INTEGRATION|TMUX|TERM)' &&
echo 'tmux version:' && tmux -V &&
echo 'Cargo available:' && cargo --version &&
echo 'Working directory:' && pwd && ls -la Cargo.toml
"
```

### Step 2: Isolate Failing Test
```bash
# Run single test with full output
docker compose run --rm integration-tests bash -c "
RUST_LOG=debug RUST_BACKTRACE=full cargo test test_name -- --nocapture
"
```

### Step 3: Check for Resource Issues
```bash
# Monitor resources during test
docker compose run --rm integration-tests bash -c "
cargo test test_name &
TEST_PID=\$!
while kill -0 \$TEST_PID 2>/dev/null; do
  echo '--- Resource Check ---'
  ps aux | head -5
  df -h /tmp
  ls -la /tmp/tmp* 2>/dev/null | wc -l
  sleep 2
done
"
```

### Step 4: Verify Test Isolation
```bash
# Check for leftover sessions
docker compose run --rm integration-tests bash -c "
cargo test session::test_basic_session_creation &&
echo 'Checking for leftover sessions:' &&
tmux list-sessions 2>/dev/null && echo 'Found sessions!' || echo 'Clean - no sessions'
"
```

### Step 5: Deep Debug
```bash
# Detailed logging and tracing
docker compose run --rm integration-tests bash -c "
RUST_LOG=trace cargo test test_name -- --nocapture 2>&1 | tee test_output.log
"
```

## Prevention Best Practices

### Writing Robust Tests

1. **Always Use TmuxTestSession**
   ```rust
   let session = TmuxTestSession::new("test-name");
   // Automatic isolation and cleanup
   ```

2. **Check Environment Early**
   ```rust
   if !should_run_integration_tests() {
       eprintln!("Skipping integration test");
       return;
   }
   ```

3. **Use Socket-Based Operations**
   ```rust
   TmuxCommand::new_session_with_socket(name, path, Some(socket_path))?;
   // Not: TmuxCommand::new_session(name, path)?;
   ```

4. **Avoid TTY Dependencies**
   ```rust
   // Test behavior, not interactive operations
   let exists = session.exists()?;
   session.send_keys("window", "command")?;
   ```

### CI/CD Configuration

1. **Set Timeouts**
   ```yaml
   - name: Integration Tests
     run: docker compose run --rm integration-tests
     timeout-minutes: 10
   ```

2. **Clean Up After Tests**
   ```yaml
   - name: Cleanup
     run: docker compose down --volumes
     if: always()
   ```

3. **Cache Docker Layers**
   ```yaml
   - name: Cache Docker layers
     uses: actions/cache@v3
     with:
       path: /tmp/.buildx-cache
       key: ${{ runner.os }}-buildx
   ```

## Getting Help

### Collect Debug Information
When reporting issues, include:

1. **Environment Details**
   ```bash
   docker compose run --rm integration-tests bash -c "
   echo 'OS:' && cat /etc/os-release | head -2 &&
   echo 'tmux:' && tmux -V &&
   echo 'Rust:' && cargo --version &&
   echo 'Env:' && env | grep -E '(INTEGRATION|TMUX|TERM)'
   "
   ```

2. **Test Output**
   ```bash
   docker compose run --rm integration-tests bash -c "
   RUST_LOG=debug cargo test failing_test -- --nocapture
   " 2>&1 | tee debug_output.log
   ```

3. **Resource Information**
   ```bash
   docker system df
   docker compose config
   ```

### Common Commands Reference

```bash
# Clean slate
docker compose down --volumes
docker compose build --no-cache
docker compose run --rm integration-tests

# Debug specific test
docker compose run --rm integration-tests bash -c "
RUST_LOG=debug cargo test test_name -- --nocapture
"

# Check test list
docker compose run --rm integration-tests bash -c "
cargo test 2>&1 | grep 'running.*tests'
"

# Verify environment
docker compose run --rm integration-tests bash -c "
env | grep -E '(INTEGRATION|TMUX|TERM)' && tmux -V
"
```

Remember: The integration test suite is designed to be robust and reliable. Most issues stem from environment configuration rather than test logic problems. When in doubt, start with a clean Docker environment and verify the basic setup before diving into specific test failures.