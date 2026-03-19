# README Summarizer

[中文文档](README.zh-CN.md)

A CLI tool written in Rust to automatically scan directories and generate summaries of README files in a single Markdown table.

## Features

- 🔍 Automatically scan all sub-projects in a specified directory
- 📝 Extract key information from README files (project name, features, problems solved)
- 💾 Persistent configuration with default settings
- 📊 Generate a single Markdown table file summarizing all projects
- 📁 Table includes clickable project directory links
- 🎯 Default output to source directory for easy access
- 💬 Interactive configuration on first run
- 👀 Watch mode: automatically detect file changes and regenerate
- 📚 Obsidian integration: one-click sync to Obsidian vault
- ⚡ Fast and efficient local parsing, no API calls required

## Installation

### Method 1: Download from GitHub Releases (Recommended)

Visit the [Releases page](https://github.com/dearfat/readme-summarizer/releases) to download pre-compiled binaries for your operating system:

**macOS**
```bash
# Download and install
curl -L https://github.com/dearfat/readme-summarizer/releases/latest/download/readme-sum-macos -o readme-sum
chmod +x readme-sum
sudo mv readme-sum /usr/local/bin/
```

**Linux**
```bash
# Download and install
curl -L https://github.com/dearfat/readme-summarizer/releases/latest/download/readme-sum-linux -o readme-sum
chmod +x readme-sum
sudo mv readme-sum /usr/local/bin/
```

**Windows**

Download `readme-sum-windows.exe`, rename it to `readme-sum.exe` and add it to your PATH environment variable.

### Method 2: Using Homebrew (macOS/Linux)

```bash
# Add tap
brew tap dearfat/tap

# Install
brew install readme-summarizer
```

### Method 3: Using Cargo

If you have the Rust toolchain installed:

```bash
# Install from GitHub
cargo install --git https://github.com/dearfat/readme-summarizer

# Install from crates.io (after published)
cargo install readme-summarizer
```

### Method 4: Build from Source

```bash
# Clone the repository
git clone https://github.com/dearfat/readme-summarizer
cd readme-summarizer

# Build release version
cargo build --release

# Binary file is located at target/release/readme-sum
# Copy it to a directory in your PATH
sudo cp target/release/readme-sum /usr/local/bin/
```

### Verify Installation

```bash
readme-sum --version
```

## Usage

### First Run

On first run, the tool will prompt you to enter the source directory to scan:

```bash
readme-sum

# You will be prompted:
# 📁 Welcome to README Summarizer!
# Please enter the source directory path: ~/projects
```

The configuration will be saved automatically for future use.

### Basic Usage

```bash
# Use configured default directory
readme-sum

# Specify source directory
readme-sum --source ~/projects

# Specify output file path
readme-sum --source ~/projects --output ~/Documents/summary.md

# Specify custom README filename
readme-sum --filename README.zh-CN.md
```

### Watch Mode

Automatically monitor source directory changes and regenerate summary when README files change or new projects are added:

```bash
# Use configured default directory
readme-sum watch

# Specify source directory
readme-sum watch --source ~/projects
```

### Obsidian Sync

Sync the generated summary file to your Obsidian vault:

```bash
# First time will prompt for vault path
readme-sum sync-obsidian

# Specify vault path
readme-sum sync-obsidian --vault ~/Documents/ObsidianVault
```

### Configuration Management

Use the `config` subcommand to manage configuration files:

```bash
# View current configuration
readme-sum config view

# Set source directory
readme-sum config set source ~/projects

# Set README filename
readme-sum config set readme-filename README.zh-CN.md

# Set output filename
readme-sum config set output-filename SUMMARY.md

# Set Obsidian vault path
readme-sum config set obsidian-vault ~/Documents/ObsidianVault

# Open configuration file in default editor
readme-sum config edit

# Show configuration file path
readme-sum config path
```

### Command Line Arguments

```
Commands:
  watch          Monitor source directory changes and auto-regenerate summary
  sync-obsidian  Sync summary file to Obsidian vault
  config         Manage configuration file
  help           Show help information

Options:
  -s, --source <PATH>      Source directory to scan (overrides config file)
  -o, --output <FILE>      Output file path (default: {source_dir}/README-SUMMARY.md)
  -f, --filename <NAME>    README filename (default: README.md)
  -h, --help              Show help information
  -V, --version           Show version information

config subcommands:
  view  View current configuration
  set   Set configuration item (source|readme-filename|output-filename|obsidian-vault)
  edit  Open configuration file in default editor
  path  Show configuration file path
```

### Configuration File

Configuration file location is automatically determined based on your operating system:

- **Linux**: `~/.config/readme-summarizer/config.toml`
- **macOS**: `~/Library/Application Support/readme-summarizer/config.toml`
- **Windows**: `%APPDATA%\readme-summarizer\config.toml`

Configuration file format:

```toml
source_directory = "/Users/username/projects"
readme_filename = "README.md"
output_filename = "README-SUMMARY.md"
obsidian_vault_path = "/Users/username/Documents/ObsidianVault"
```

On first run, the tool will interactively prompt for the source directory and automatically create the configuration file. You can manually edit this file to modify default values.

## How It Works

1. **Interactive Configuration**: Prompts for source directory on first run and saves it
2. **Directory Scanning**: Traverses one level of subdirectories in the source directory
3. **README Discovery**: Finds specified README files in each subdirectory
4. **Content Parsing**: Uses rules to extract project information:
   - **Project Name**: Extracts first `#` heading, otherwise uses directory name
   - **Features**: Looks for "Features", "功能", "Description" keyword sections
   - **Problems Solved**: Looks for "Problem", "问题", "Why", "Motivation" keyword sections
5. **Table Generation**: Creates a single Markdown file with all project summaries in a table format
6. **Output**: Saves to source directory (default) or specified path

## Example Output

Running the tool:

```bash
$ readme-sum --source ~/projects

📁 Scanning directory: /Users/username/projects
📝 README filename: README.md
📄 Output file: /Users/username/projects/README-SUMMARY.md

✓ Processed: my-web-app
✓ Processed: rust-cli-tool
✓ Processed: python-script

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Complete! Successfully processed 3 projects
📄 Summary file generated: /Users/username/projects/README-SUMMARY.md
```

Generated summary file format:

```markdown
# README Project Summary

Generated: 2026-03-19 17:30:00

| Project Name | Project Directory | Features | Problems Solved | Source File Path |
|--------------|-------------------|----------|-----------------|------------------|
| my-web-app | [📁](/path/to/my-web-app/) | Real-time analytics<br>Beautiful charts<br>... | Expensive tools<br>Complex setup<br>... | projects/my-web-app/README.md |
| rust-cli-tool | [📁](/path/to/rust-cli-tool/) | Fast execution<br>Cross-platform<br>... | Manual processes<br>Slow scripts<br>... | projects/rust-cli-tool/README.md |

---
*This summary was automatically generated by README Summarizer*
```

**Project Directory column** contains clickable folder icon links that open the project directory directly in your file manager.

## Tech Stack

- **Rust 2021 Edition**
- **clap**: Command-line argument parsing
- **serde + toml**: Configuration serialization
- **regex**: Text pattern matching
- **chrono**: Timestamp generation
- **walkdir**: Directory traversal
- **notify**: File system watching
- **dirs**: Cross-platform directory paths

## Project Structure

```
readme-summarizer/
├── src/
│   ├── main.rs          # Entry point and CLI handling
│   ├── config.rs        # Configuration management
│   ├── scanner.rs       # Directory scanning
│   ├── parser.rs        # README parsing
│   ├── generator.rs     # Summary generation
│   └── error.rs         # Error handling
├── Cargo.toml           # Dependencies and metadata
├── README.md            # This file
├── README.zh-CN.md      # Chinese documentation
├── LICENSE              # MIT License
└── CHANGELOG.md         # Version history
```

## Notes

- The tool scans only one level of subdirectories, not recursively
- Parsing rules are based on common README structures and may not cover all formats
- Default output file is located in the source directory with filename `README-SUMMARY.md`
- Existing summary files will be overwritten
- Long text in tables uses `<br>` to separate multiple lines
- Watch mode ignores changes to the output file itself to avoid infinite loops
- Obsidian sync will overwrite files with the same name in the vault

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- [Publishing Guide](HOW_TO_PUBLISH.md) - Complete guide to publishing
- [Release Guide](RELEASE.md) - Detailed release process
- [Homebrew Setup](HOMEBREW_SETUP.md) - Homebrew installation setup
- [Usage Examples](USAGE_EXAMPLES.md) - More usage examples
