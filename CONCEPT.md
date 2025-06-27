# tmuxrs - Application Concept

## Overview

`tmuxrs` is a command-line tool that automates tmux session management through declarative YAML configuration files. It serves as a modern, Rust-based alternative to tmuxinator, providing the same functionality with improved performance and maintaining full compatibility with existing tmuxinator configurations.

## Core Philosophy

### 1. Configuration as Code
- Define your entire workspace layout in YAML files
- Version control your development environments
- Share consistent setups across teams

### 2. Zero Friction Context Switching
- Switch between projects with a single command
- Automatically restore complex window/pane layouts
- Maintain project-specific environments

### 3. Tmuxinator Compatibility
- Drop-in replacement for tmuxinator
- Use existing configuration files without modification
- Familiar commands and workflow

## Key Concepts

### Sessions
A **session** in tmuxrs represents a complete tmux workspace, typically corresponding to a single project or task context. Each session contains:
- Multiple windows (tabs)
- Multiple panes per window
- Specific layouts and commands

### Configuration Files
- Stored in `~/.config/tmuxrs/*.yml` or `*.yaml`
- Each file defines one session
- Name of the file becomes the session identifier

### Directory Context
- Sessions can be associated with specific directories via the `root` field
- Running `tmuxrs start` without arguments detects the appropriate session based on current directory
- Enables automatic workspace activation

## Core Innovation: Centralized + Directory-Aware Configuration

### The Problem with tmuxinator
tmuxinator requires either:
- Manual project specification: `tmuxinator start webapp`
- Local config files scattered across projects: `.tmuxinator.yml` in each directory

### tmuxrs Solution: Best of Both Worlds

**Centralized Configuration Management:**
```
~/.config/tmuxrs/
├── webapp.yml
├── api-server.yml  
└── mobile-app.yml
```

**Directory-Aware Execution:**
```bash
cd /path/to/webapp/
tmuxrs start                    # Auto-detects webapp.yml via Git repo name

cd /different/path/to/webapp/   
tmuxrs start                    # Still uses webapp.yml (same project)

cd /path/to/api-server/
tmuxrs start                    # Auto-detects api-server.yml
```

**Key Benefits:**
- ✅ All configs in one place (easier management)
- ✅ Directory-aware execution (convenient workflow)  
- ✅ No config files scattered across projects
- ✅ Works regardless of project location
- ✅ Still supports local `.tmuxinator.yml` for compatibility

### Configuration Discovery
Smart session detection combines centralized management with directory awareness. 

*See [System Architecture](docs/design/02-system-architecture.md#configuration-discovery) for detailed discovery logic.*

*For detailed architectural analysis of existing tools, see [Research Documentation](docs/research/).*