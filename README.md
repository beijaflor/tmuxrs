# tmuxrs

[![Crates.io](https://img.shields.io/crates/v/tmuxrs.svg)](https://crates.io/crates/tmuxrs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/beijaflor/tmuxrs#license)
[![Build Status](https://img.shields.io/github/actions/workflow/status/beijaflor/tmuxrs/ci.yml?branch=main)](https://github.com/beijaflor/tmuxrs/actions)

> A modern, Rust-based tmux session manager with centralized configuration and directory-aware execution.

**tmuxrs** is a drop-in replacement for [tmuxinator](https://github.com/tmuxinator/tmuxinator) that solves the scattered configuration problem while maintaining full compatibility. Manage all your tmux sessions from a centralized location with automatic project detection.

## ✨ Key Features

- 🔄 **Drop-in tmuxinator replacement** - Use existing configs without changes
- 📁 **Centralized configuration** - All configs in `~/.config/tmuxrs/`
- 🎯 **Directory-aware execution** - Auto-detects sessions from current directory
- ⚡ **Rust performance** - Faster session creation than Ruby tmuxinator
- 🔗 **TTY-aware attachment** - Seamless terminal takeover
- 🏗️ **Layout support** - Complex window layouts with multiple panes
- 🛠️ **Modern CLI** - Better error messages and user experience

## 💡 Philosophy

tmuxrs is built on three core principles:

### Configuration as Code
- Define your entire workspace layout in YAML files
- Version control your development environments  
- Share consistent setups across teams

### Zero Friction Context Switching
- Switch between projects with a single command
- Automatically restore complex window/pane layouts
- Maintain project-specific environments

### Tmuxinator Compatibility
- Drop-in replacement for tmuxinator
- Use existing configuration files without modification
- Familiar commands and workflow

## 🚀 Quick Start

### Installation

```bash
# Install from crates.io
cargo install tmuxrs

# Or from source
git clone https://github.com/beijaflor/tmuxrs
cd tmuxrs
cargo install --path .

# Install man page (optional)
sudo cp man/tmuxrs.1 /usr/local/share/man/man1/
sudo mandb  # or makewhatis on some systems
```

### Basic Usage

```bash
# Create a config file
mkdir -p ~/.config/tmuxrs
cat > ~/.config/tmuxrs/myproject.yml << EOF
name: myproject
root: ~/code/myproject
windows:
  - editor:
      layout: main-vertical
      panes:
        - vim
        - rails server
  - monitoring:
      layout: tiled
      panes:
        - htop
        - tail -f log/development.log
EOF

# Start the session
tmuxrs start myproject

# Or auto-detect from directory
cd ~/code/myproject
tmuxrs start  # Automatically finds myproject.yml

# List available sessions
tmuxrs list

# Stop a session
tmuxrs stop myproject
```

## 🔄 Migration from tmuxinator

tmuxrs is designed as a **drop-in replacement** for tmuxinator:

```bash
# Your existing tmuxinator configs work unchanged
cp ~/.config/tmuxinator/myproject.yml ~/.config/tmuxrs/myproject.yml

# Same commands, better performance
tmuxrs start myproject    # Instead of: tmuxinator start myproject
tmuxrs stop myproject     # Instead of: tmuxinator stop myproject
tmuxrs list              # Instead of: tmuxinator list
```

## 🏗️ Configuration

### Simple Window Configuration
```yaml
name: simple-project
root: ~/code/simple-project
windows:
  - editor: vim
  - server: rails server
  - shell: bash
```

### Advanced Layout Configuration
```yaml
name: complex-project
root: ~/code/complex-project
windows:
  - main:
      layout: main-vertical
      panes:
        - vim src/main.rs
        - cargo watch -x run
  - monitoring:
      layout: tiled
      panes:
        - htop
        - iostat 2
        - tail -f /var/log/system.log
        - netstat -i
```

### Available Layouts
- `main-vertical` - Side-by-side with main pane on left
- `main-horizontal` - Top/bottom with main pane on top
- `tiled` - All panes equally sized in grid
- `even-horizontal` - All panes equal width
- `even-vertical` - All panes equal height

## 🎯 Core Innovation: Centralized + Directory-Aware

### The Problem with tmuxinator
tmuxinator requires either:
- Manual project specification: `tmuxinator start webapp`
- Scattered config files: `.tmuxinator.yml` in every project directory

This creates friction - you either lose convenience or end up with config files scattered across projects.

### tmuxrs Solution: Best of Both Worlds

**Centralized Configuration:**
```
~/.config/tmuxrs/
├── webapp.yml
├── api-server.yml
└── mobile-app.yml
```

**Directory-Aware Execution:**
```bash
cd /path/to/webapp/
tmuxrs start                    # Auto-detects webapp.yml

cd /different/path/to/webapp/   
tmuxrs start                    # Still finds webapp.yml (same project)

cd /path/to/api-server/
tmuxrs start                    # Auto-detects api-server.yml
```

## 📖 Command Line Interface

```bash
# Session management
tmuxrs start [NAME]             # Start session (auto-detect if no name)
tmuxrs start --no-attach        # Start detached session
tmuxrs start --append           # Add windows to existing session
tmuxrs stop <NAME>              # Stop session
tmuxrs list                     # List available configurations

# Examples
tmuxrs start                    # Auto-detect from current directory
tmuxrs start myproject          # Start specific session
tmuxrs start --no-attach        # Start without attaching
```

## 🔧 Development

```bash
# Clone and build
git clone https://github.com/beijaflor/tmuxrs
cd tmuxrs
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- [tmuxinator](https://github.com/tmuxinator/tmuxinator) - The original inspiration
- [tmux](https://github.com/tmux/tmux) - The amazing terminal multiplexer

---

**Happy tmuxing!** 🚀