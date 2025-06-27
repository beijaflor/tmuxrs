# tmuxrs Feature Decision Framework

*Based on analysis in [Comparison Matrix](../research/05-comparison-matrix.md). See [System Architecture](02-system-architecture.md) for implementation details.*

## 1. Configuration System

### A. Configuration Format Support

**Options:**

1. **YAML-only** (tmuxinator compatibility)
   - ✅ Perfect tmuxinator migration
   - ✅ Single format simplicity
   - ❌ Limited user choice
2. **Multi-format** (YAML + JSON + TOML)
   - ✅ User preference flexibility
   - ✅ Better tooling integration
   - ❌ More complexity
   - ❌ Testing multiple formats

**Decision:** [x] Option 1: YAML-only [ ] Option 2: Multi-format

### B. Configuration Compatibility

**Options:**

1. **Strict tmuxinator compatibility**
   - ✅ Drop-in replacement
   - ✅ Easy migration
   - ❌ Limited by tmuxinator's design
2. **Enhanced format with backward compatibility**
   - ✅ Modern features possible
   - ✅ Still supports tmuxinator configs
   - ❌ More complex parsing
3. **New format inspired by tmuxinator**
   - ✅ Clean, modern design
   - ❌ No direct migration path
   - ❌ Users must rewrite configs

**Decision:** [x] Option 1: Strict [ ] Option 2: Enhanced [ ] Option 3: New format

### C. Configuration Loading Strategy

**Options:**

1. **Single file per project** (tmuxinator style)
   - ✅ Simple mental model
   - ✅ Perfect tmuxinator compatibility
   - ❌ No configuration reuse
2. **Hierarchical loading** (global + local)
   - ✅ Shared defaults + project overrides
   - ✅ Better for teams
   - ❌ More complex
3. **Template + instance** (dmux style)
   - ✅ High reusability
   - ✅ Flexible directory application
   - ❌ Different mental model from tmuxinator

**Decision:** [x] Option 1: Single file [ ] Option 2: Hierarchical [ ] Option 3: Template
**ADDITIONAL NOTES FOR DECISION**: Auto-discovery without explicit mapping. Support both centralized configs (~/.config/tmuxrs/) AND local directory configs (.tmuxinator.yml in project root).

### D. Dynamic Configuration

**Options:**

1. **Static YAML only**
   - ✅ Simple, predictable
   - ✅ Easy validation
   - ❌ No dynamic content
2. **Environment variable substitution**
   - ✅ Basic dynamic content
   - ✅ Common pattern
   - ❌ Limited flexibility
3. **Template engine** (like ERB)
   - ✅ Full dynamic configuration
   - ✅ Tmuxinator compatibility
   - ❌ Complex implementation

**Decision:** [x] Option 1: Static [ ] Option 2: Env vars [ ] Option 3: Templates

## 2. Session Management

### A. Session Creation Strategy

**Options:**

1. **Script generation** (tmuxinator approach)
   - ✅ Exact tmuxinator compatibility
   - ✅ Full shell script power
   - ❌ Complex, harder to debug
2. **Direct tmux commands** (dmux/tuxmux approach)
   - ✅ Simpler implementation
   - ✅ Better error handling
   - ✅ Easier testing
   - ❌ May limit some advanced features

**Decision:** [ ] Option 1: Script generation [x] Option 2: Direct commands

### B. Existing Session Handling

**Options:**

1. **Always create new** (tmuxinator default)
   - ✅ Predictable behavior
   - ❌ Can create duplicates
2. **Check and attach if exists** (modern approach)
   - ✅ Prevents duplicates
   - ✅ Faster when session exists
   - ❌ Different from tmuxinator
3. **Configurable behavior**
   - ✅ User choice
   - ❌ More complexity

**Decision:** [ ] Option 1: Always new [x] Option 2: Check first [ ] Option 3: Configurable

### C. Session Discovery

**Options:**

1. **Manual specification only**
   - ✅ Simple, explicit
   - ✅ Tmuxinator compatible
   - ❌ No modern conveniences
2. **Directory-based auto-detection**
   - ✅ Convenient workflow
   - ✅ Modern UX
   - ❌ Magic behavior
3. **Git repository discovery** (tuxmux style)
   - ✅ Developer-focused
   - ✅ Automatic workspace detection
   - ❌ Git-only

**Decision:** [ ] Option 1: Manual only [x] Option 2: Directory detection [ ] Option 3: Git discovery
**ADDITIONAL NOTES FOR DECISION**: Revised for thin MVP - use directory path basename instead of Git repo detection. Git integration can be added later.

## 3. User Experience Features

### A. Session Selection Interface

**Options:**

1. **CLI arguments only**
   - ✅ Scriptable
   - ✅ Simple implementation
   - ❌ No interactive discovery
2. **Fuzzy finder integration** (fzf)
   - ✅ Modern, interactive UX
   - ✅ Fast session discovery
   - ❌ External dependency
3. **Built-in interactive selector**
   - ✅ No external dependencies
   - ✅ Integrated experience
   - ❌ More implementation work

**Decision:** [x] Option 1: CLI only [ ] Option 2: fzf integration [ ] Option 3: Built-in
**ADDITIONAL NOTES FOR DECISION**: Option 2 is nice to have later, but not for now.

### B. Directory Handling Philosophy

**Options:**

1. **Project-specific configs** (tmuxinator)
   - ✅ Rich per-project customization
   - ✅ Familiar to tmuxinator users
   - ❌ Config proliferation
2. **Directory-agnostic templates** (dmux)
   - ✅ High reusability
   - ✅ Less configuration maintenance
   - ❌ Less project-specific customization
3. **Hybrid approach**
   - ✅ Best of both worlds
   - ❌ More complex implementation

**Decision:** [ ] Option 1: Project-specific [x] Option 2: Templates [ ] Option 3: Hybrid

### C. Git Integration Level

**Options:**

1. **No Git integration**
   - ✅ General purpose
   - ✅ Simple implementation
   - ❌ Misses developer workflows
2. **Basic Git awareness** (detect git repos)
   - ✅ Helpful for developers
   - ✅ Not too opinionated
   - ❌ Limited usefulness
3. **Advanced Git features** (worktrees, branches)
   - ✅ Powerful for Git workflows
   - ✅ Unique selling point
   - ❌ Complex implementation
   - ❌ Git-only focus

**Decision:** [x] Option 1: No Git [ ] Option 2: Basic Git [ ] Option 3: Advanced Git
**ADDITIONAL NOTES FOR DECISION**: Revised for thin MVP - eliminate Git dependencies. Directory basename detection is much simpler. Git integration can be added later.

### D. Quick Session Switching

**Options:**

1. **No quick switching**
   - ✅ Simple implementation
   - ❌ Misses productivity feature
2. **Jump lists** (tuxmux style)
   - ✅ Fast session switching
   - ✅ Modern productivity feature
   - ❌ Additional state management
3. **Recent sessions** (simple history)
   - ✅ Useful without complexity
   - ✅ Easier implementation
   - ❌ Less powerful than jump lists

**Decision:** [x] Option 1: No switching [ ] Option 2: Jump lists [ ] Option 3: Recent history

## 4. Tmux Feature Support

### A. Window Layout Support

**Options:**

1. **No layout support** (MVP)
   - ✅ Simple implementation
   - ❌ Limited tmux features
2. **Basic layout support** (predefined layouts)
   - ✅ Common use cases covered
   - ✅ Manageable complexity
3. **Full layout support** (custom layouts, percentages)
   - ✅ Complete tmux feature parity
   - ❌ Complex implementation

**Decision:** [ ] Option 1: No layouts [x] Option 2: Basic layouts [ ] Option 3: Full layouts
**ADDITIONAL NOTES FOR DECISION**: Percentages would be nice to have.

### B. Multi-pane Window Support

**Options:**

1. **Single pane per window only** (MVP)
   - ✅ Simple implementation
   - ❌ Limited usefulness
2. **Multi-pane with basic splitting**
   - ✅ Common use cases
   - ✅ Reasonable complexity
3. **Full pane management** (complex layouts, commands per pane)
   - ✅ Complete feature set
   - ❌ High complexity

**Decision:** [x] Option 1: Single pane [ ] Option 2: Basic multi-pane [ ] Option 3: Full pane mgmt
**ADDITIONAL NOTES FOR DECISION**: Revised for thin MVP - single pane keeps tmux command generation very simple.

### C. Lifecycle Hooks

**Options:**

1. **No hooks** (MVP)
   - ✅ Simple implementation
   - ❌ Limited automation
2. **Basic hooks** (pre_window, on_project_start)
   - ✅ Common automation needs
   - ✅ Tmuxinator compatibility
3. **Full lifecycle hooks** (comprehensive events)
   - ✅ Maximum flexibility
   - ❌ Complex implementation

**Decision:** [x] Option 1: No hooks [ ] Option 2: Basic hooks [ ] Option 3: Full hooks

### D. Advanced Tmux Features

**Options:**

1. **Basic features only** (sessions, windows, panes)
   - ✅ Covers 80% of use cases
   - ✅ Manageable scope
2. **Extended features** (synchronization, custom options)
   - ✅ Power user features
   - ✅ Better tmuxinator compatibility
3. **Complete tmux API** (all tmux capabilities)
   - ✅ Full feature parity
   - ❌ Very large scope

**Decision:** [x] Option 1: Basic [ ] Option 2: Extended [ ] Option 3: Complete

## 5. Implementation Strategy

### A. MVP Scope

**Options:**

1. **Minimal MVP** (list, start, stop only)
   - ✅ Fast initial delivery
   - ✅ Clear scope boundaries
   - ❌ Limited initial usefulness
2. **Extended MVP** (+ layouts, multi-pane, basic hooks)
   - ✅ More useful initially
   - ✅ Still manageable scope
   - ❌ Longer development time
3. **Feature-rich MVP** (most features except stretch goals)
   - ✅ Competitive from day one
   - ❌ Long development cycle
   - ❌ Higher risk

**Decision:** [x] Option 1: Minimal [ ] Option 2: Extended [ ] Option 3: Feature-rich

### B. Error Handling Strategy

**Options:**

1. **Basic error handling** (simple error messages)
   - ✅ Simple implementation
   - ❌ Poor debugging experience
2. **Rich error reporting** (context, suggestions)
   - ✅ Better user experience
   - ✅ Easier troubleshooting
   - ❌ More implementation work

**Decision:** [x] Option 1: Basic errors [ ] Option 2: Rich errors

### C. External Dependencies

**Options:**

1. **Minimal dependencies** (essential crates only)
   - ✅ Faster compilation
   - ✅ Smaller binary
   - ❌ More implementation work
2. **Pragmatic dependencies** (use good crates for common tasks)
   - ✅ Faster development
   - ✅ Battle-tested code
   - ❌ Larger dependency tree

**Decision:** [ ] Option 1: Minimal deps [x] Option 2: Pragmatic deps

## Decision Summary Template

Once you make your choices, I'll create the architecture design document based on your decisions:

```
Configuration: [Your choices]
Session Management: [Your choices]
User Experience: [Your choices]
Tmux Features: [Your choices]
Implementation: [Your choices]
```
