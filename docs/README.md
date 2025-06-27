# tmuxrs Documentation

## 📂 Documentation Structure

```
docs/
├── README.md          # This file
├── ROADMAP.md         # Development phases and features
│
├── research/          # Analysis of existing tools
│   ├── 01-tmux-internals.md
│   ├── 02-tmuxinator-analysis.md
│   ├── 03-tuxmux-analysis.md
│   ├── 04-dmux-analysis.md
│   └── 05-comparison-matrix.md
│
├── design/            # tmuxrs design decisions
│   ├── 01-feature-decisions.md
│   └── 02-system-architecture.md
│
└── guides/            # Implementation guides
    └── 01-implementation-guide.md
```

## 📖 Reading Order

### For Understanding the Project
1. **Root Directory Files** (Start here)
   - `CONCEPT.md` - Vision and core innovation
   - `CLAUDE.md` - Quick development reference
   - `docs/ROADMAP.md` - Development phases and features

2. **Research Phase** (`docs/research/`)
   - `01-tmux-internals.md` - Understanding tmux
   - `02-tmuxinator-analysis.md` - Compatibility reference
   - `03-tuxmux-analysis.md` - Modern Rust patterns
   - `04-dmux-analysis.md` - Alternative approaches
   - `05-comparison-matrix.md` - Feature comparison

3. **Design Phase** (`docs/design/`)
   - `01-feature-decisions.md` - All design choices
   - `02-system-architecture.md` - Technical blueprint

4. **Implementation** (`docs/guides/`)
   - `01-implementation-guide.md` - Step-by-step coding

## 🎯 Document Purposes

### Root Directory
- **CONCEPT.md**: Why tmuxrs exists, core problem/solution
- **CLAUDE.md**: How to build it (constraints, style guide)

### Documentation Root
- **docs/ROADMAP.md**: What we're building, MVP and future features

### Research (`docs/research/`)
- **01-tmux-internals**: Client-server model, session hierarchy
- **02-tmuxinator-analysis**: Ruby architecture, YAML processing
- **03-tuxmux-analysis**: KDL config, Git integration patterns
- **04-dmux-analysis**: Directory-agnostic templates
- **05-comparison-matrix**: Side-by-side feature analysis

### Design (`docs/design/`)
- **01-feature-decisions**: Each architectural choice explained
- **02-system-architecture**: Modules, data flow, error handling

### Guides (`docs/guides/`)
- **01-implementation-guide**: Practical coding roadmap with examples