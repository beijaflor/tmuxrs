# tmuxinator Architecture

## Core Architecture

tmuxinator is a Ruby-based tmux session manager that translates YAML configurations into shell scripts containing tmux commands. It follows a hierarchical object model mirroring tmux's session→window→pane structure.

## Class Hierarchy

### Primary Classes
- **`Tmuxinator::CLI`**: Command-line interface and application entry point
- **`Tmuxinator::Config`**: Configuration file management and validation
- **`Tmuxinator::Project`**: Central session representation and script generation
- **`Tmuxinator::Window`**: Individual window management within sessions
- **`Tmuxinator::Pane`**: Individual pane management within windows

### Object Relationships
```
CLI → Config → Project
              ├── Window → Pane
              ├── Window → Pane
              └── Window → Pane
```

## YAML Configuration Processing

### Parsing Pipeline
1. **File Loading**: `Tmuxinator::Project.load(path, options)`
2. **ERB Processing**: `render_template()` handles embedded Ruby in YAML
3. **YAML Parsing**: `YAML.safe_load()` converts to Ruby hash
4. **Object Instantiation**: `Tmuxinator::Project.new(yaml, options)`
5. **Validation**: Configuration structure and required fields

### ERB Template System
- **Dynamic YAML**: Embedded Ruby code in configuration files
- **Environment Access**: `<%= ENV["PWD"] %>` for runtime values
- **Conditional Logic**: Ruby conditionals within YAML structure
- **Variable Interpolation**: Dynamic session and window names

## Command Generation Strategy

### Script Generation Process (`Project#render`)
1. **Template Loading**: Read `template.erb` file
2. **Data Binding**: Inject project data into ERB context
3. **Script Generation**: Process ERB template with project attributes
4. **Command Assembly**: Combine tmux commands into executable shell script

### ERB Template Structure
- **Session Creation**: `tmux new-session` commands
- **Window Setup**: `tmux new-window` for each configured window
- **Pane Creation**: `tmux split-window` for multi-pane windows
- **Command Execution**: `tmux send-keys` for initial commands
- **Client Attachment**: Final attachment to created session

### Key Command Methods
- **`tmux_new_session_command`**: Generates session creation command
- **`tmux_kill_session_command`**: Generates session termination command
- **`tmux_new_window_command`**: Generates window creation command
- **`tmux_window_target`**: Provides targeting strings for tmux commands

## Session Management Flow

### Command Execution Flow
1. **CLI Invocation**: User runs `tmuxinator start project_name`
2. **Project Creation**: `create_project()` loads and validates configuration
3. **Script Rendering**: `render_project()` generates shell script
4. **Script Execution**: `Kernel.exec(project.render)` runs generated script

### Configuration Discovery (`Config.project`)
Search order for configuration files:
1. **Local Directory**: `./project_name.yml`
2. **Environment Variable**: `$TMUXINATOR_CONFIG/project_name.yml`
3. **XDG Config**: `$XDG_CONFIG_HOME/tmuxinator/project_name.yml`
4. **Home Directory**: `~/.tmuxinator/project_name.yml`

### Validation Process (`Config.validate`)
- **File Existence**: Ensure configuration file exists
- **YAML Validity**: Verify parseable YAML structure
- **Required Fields**: Check for essential configuration elements
- **Project Instantiation**: Create `Tmuxinator::Project` object

## Window and Pane Management

### Window Configuration (`Tmuxinator::Window`)
- **Initialization**: `initialize(window_yaml, index, project)`
- **Pane Building**: `build_panes()` creates `Tmuxinator::Pane` objects
- **Command Generation**: `tmux_new_window_command` for window creation
- **Targeting**: `tmux_window_target` for command addressing

### Pane Management (`Tmuxinator::Pane`)
- **Creation Logic**: Built via `Window#build_panes` method
- **Command Handling**: Individual pane command execution
- **Layout Support**: Integration with tmux layout system
- **Index Tracking**: Proper pane numbering and targeting

## Hook System Architecture

### Lifecycle Hooks (`Tmuxinator::Hooks::Project`)
- **`on_project_start`**: Commands before session creation
- **`on_project_stop`**: Commands after session termination
- **`pre_window`**: Commands before each window setup
- **Hook Execution**: Integrated into ERB template generation

### Hook Processing
- **YAML Definition**: Hooks defined in configuration files
- **Script Integration**: Hooks embedded in generated shell scripts
- **Execution Timing**: Precise placement in command sequence
- **Error Handling**: Hook failures affect overall execution

## Key Design Patterns

### Template Method Pattern
- **`Project#render`**: Defines script generation algorithm
- **ERB Processing**: Template-based code generation
- **Extensible Structure**: New commands via template modification

### Factory Pattern
- **Object Creation**: `Project.load()` creates configured instances
- **Configuration-Driven**: Objects built from YAML specifications
- **Validation Integration**: Factory methods include validation

### Command Pattern (Implicit)
- **Command Objects**: Each tmux command as method call
- **Script Assembly**: Commands combined into executable script
- **Deferred Execution**: Commands generated then executed via `Kernel.exec`

### Decorator Pattern (Modules)
- **`Tmuxinator::Util`**: Utility method injection
- **`Tmuxinator::Deprecations`**: Backwards compatibility layer
- **`Tmuxinator::WemuxSupport`**: Optional feature extension

## Configuration File Architecture

### YAML Structure Requirements
```yaml
name: required_session_name
root: optional_working_directory

windows:
  - window_name: command
  - window_name:
      layout: layout_name
      panes:
        - command1
        - command2
```

### Advanced Configuration Features
- **Layout Specification**: tmux layout strings per window
- **Multi-Pane Windows**: Array of commands creating multiple panes
- **Working Directories**: Per-window root directory overrides
- **Command Lists**: Multiple commands per pane

## Error Handling Strategy

### Validation Layers
- **File System**: Configuration file existence and readability
- **YAML Syntax**: Valid YAML structure and parsing
- **Configuration Logic**: Required fields and valid values
- **Tmux Integration**: Valid tmux commands and options

### Error Propagation
- **Early Validation**: Fail fast on configuration errors
- **Clear Messages**: User-friendly error descriptions
- **Context Preservation**: Error location and cause tracking

## Key Insights for tmuxrs

1. **ERB Template System**: Powerful but complex - consider simpler templating
2. **Object Hierarchy**: Clean separation mirrors tmux structure well
3. **Configuration Discovery**: Multiple search locations improve usability
4. **Script Generation**: Converting config to commands is core functionality
5. **Hook System**: Lifecycle hooks are essential for complex setups
6. **Validation Strategy**: Multi-layer validation catches errors early
7. **Factory Pattern**: Configuration-driven object creation is clean approach
8. **Command Targeting**: Proper tmux targeting syntax is crucial for reliability