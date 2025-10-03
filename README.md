# Opus To-Do TUI

[Opus](https://www.dictionary.com/browse/opus) is a lightweight **Terminal based To-Do App** build in _Rust🦀_  featuring a clean TUI(_Text User Interface_) for managing projects and tasks.

## Features
- **Project-based organization** - keeping tasks grouped in projects.
- **Minimalistic TUI** - Designed with ratatui for smooth terminal interaction.
- **Keyboard Driven** - Add, Delete and toggle tasks without leaving your keyboard.
- **Persistent Storage** - Projects and Tasks are neatly stored in json files.

## Installation
### Prerequisites
- Rust toolchain [install rust](https://rust-lang.org/tools/install/).
- `make`
- `sudo`

### Build and Install

```bash
# Cloning the repo
git clone https://github.com/NullSector-dev/Opus.git
# Enter the repo directory
cd Opus
```
Building
```bash
make build
```
Installing
```bash
sudo make install
```

## Usage
Run Opus using
```bash
opus
```

## Binds
### In Projects tab
|  Keybind  |        Action       |
| :-------- | :------------------ |
|   Enter   |     Open Project    |
|     a     |     Add  Project    |
|  up/down  |Move between Projects|
|     d     |    Delete Project   |
|     q     |     Quit Session    |

### In Tasks tab
| Keybind | Actions |
|:--------|:--------|
|t|Toggle Task Completion|
|a|Add Task|
|d|Delete Task|
|up/down| Move between Tasks|
|Esc|Go back to Projects tab|

### Popup
Keybinds for pop up are universal
|Keybind|Action|
|:------|:-----|
|Enter|Add Task/Project|
|Esc|Exit Popup|

## License
This project is licensed under the MIT License -- See the [LICENSE](/LICENSE) file to see details.
