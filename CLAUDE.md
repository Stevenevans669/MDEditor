# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MDEditor is a lightweight external editor for Claude Code's `Ctrl+G` functionality, built with Tauri v2. It allows users to edit prompts in a dedicated window before returning content to Claude Code.

## Technology Stack

- **Framework**: Tauri v2
- **Frontend**: Native HTML/CSS/JS (no framework)
- **Platform**: macOS

## Build Commands

```bash
# Install dependencies
npm install

# Development mode with hot reload
npm run tauri dev

# Build production release
npm run tauri build
```

## Architecture

### Core Flow

1. Claude Code spawns `mdeditor <file_path>`
2. MDEditor reads file, displays editor window
3. User edits content
4. On close: save confirmation dialog if content changed
5. Exit with code 0 (saved) or 1 (cancelled)

### Project Structure

```
mdeditor/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── commands.rs  # Tauri IPC commands
│   │   └── file_ops.rs  # File operations
│   └── tauri.conf.json  # Tauri config
└── src/                 # Frontend
    ├── index.html
    ├── style.css
    └── main.js
```

### Tauri Commands

| Command | Purpose |
|---------|---------|
| `get_file_path` | Get file path from CLI args |
| `read_file` | Read file content (UTF-8) |
| `save_file` | Atomic save (write .tmp → fsync → rename) |
| `exit_app` | Exit with specified code |

### Exit Codes

- `0`: Confirmed (saved and exit)
- `1`: Cancelled (exit without saving)
- `2+`: Error

## Key Requirements

- File saves must be atomic (write to .tmp, fsync, rename)
- Process must block until user closes window (Claude Code waits for exit)
- Window size: 80% of screen, centered, minimum 400×300
- Use system monospace font (SF Mono, Menlo, monospace)
