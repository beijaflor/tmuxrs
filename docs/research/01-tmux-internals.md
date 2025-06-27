# tmux Architecture

## Client-Server Model

tmux operates on a robust client-server architecture that enables persistent sessions.

### Core Components
- **Server Process**: Background daemon managing all tmux operations
- **Client Process**: User interface that connects to server via Unix socket
- **Socket Communication**: Located in `/tmp`, enables multiple clients per server

### Session Persistence
- Server persists even when clients disconnect
- Sessions survive SSH timeouts and terminal closures
- Server automatically forks to background on first launch
- Server terminates only when all sessions are closed

## Hierarchical Structure

tmux organizes display in a strict hierarchy:

```
Server (one per user)
├── Session ($0, $1, $2...)
│   ├── Window (@0, @1, @2...)
│   │   ├── Pane (%0, %1, %2...)
│   │   └── Pane (%3, %4...)
│   └── Window (@3...)
└── Session ($3...)
```

### Unique Identifiers
- **Sessions**: Prefixed with `$` (e.g., `$0`, `$42`)
- **Windows**: Prefixed with `@` (e.g., `@1`, `@99`) 
- **Panes**: Prefixed with `%` (e.g., `%0`, `%93`)
- IDs are unique within a server instance but can repeat across servers

## Key Data Structures

### Server Management
- `clients`: TAILQ of connected client objects
- `sessions`: Global RB-tree of session objects
- `windows`: Global RB-tree of window objects
- `all_window_panes`: Global RB-tree of pane objects

### Session Structure
- `struct session`: Contains winlinks, current window pointer, session metadata
- `struct winlink`: Links windows to sessions at specific indices
- Allows same window to appear in multiple sessions

### Window and Pane Structure
- `struct window`: Contains panes, layout tree, window metadata
- `struct window_pane`: Individual terminal instance with process ID, file descriptor
- `struct screen`: Visual state representation with grid data
- `struct grid`: Character data and attributes for terminal display

## Communication Protocol

### Message Types
- `MSG_COMMAND`: Client commands to server
- `MSG_KEY`: Keystroke events from client
- `MSG_IDENTIFY`: Client identification
- `MSG_STDOUT`/`MSG_STDERR`: Server output to clients
- `MSG_EXIT`/`MSG_SHUTDOWN`: Session termination
- `MSG_READY`: Client attachment confirmation

### Client-Server Flow
1. Client connects via Unix socket
2. Client sends identification and commands
3. Server processes commands and updates state
4. Server sends updates to affected clients
5. Client renders changes to terminal

## Session Lifecycle

### Creation
- `tmux new-session` creates session and first window
- Server automatically created if none exists
- Session gets unique ID and optional name

### Attachment Logic
- **Outside tmux**: Use `tmux attach-session -t <session>`
- **Inside tmux**: Use `tmux switch-client -t <session>`
- Multiple clients can attach to same session simultaneously

### Persistence Mechanism
- Sessions remain active in server process
- Windows and panes maintain running processes
- State preserved across client disconnections
- Automatic cleanup when session ends

## Key Insights for tmuxrs

1. **Server Management**: Not needed for tmuxrs - we'll use tmux's existing server
2. **Command Generation**: Focus on translating configs to tmux commands
3. **Session Detection**: Use `tmux has-session` before creating
4. **Attachment Logic**: Detect `$TMUX` environment to choose attach method
5. **Hierarchical Thinking**: Design configs around session→window→pane structure