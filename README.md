# 🐍 Rust Snake

A classic Snake game built with Rust and the Piston game engine, featuring smooth grid-based animations and progressive difficulty scaling.

![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Version](https://img.shields.io/badge/Version-1.0.0-green.svg)

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [Screenshots](#screenshots)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [How to Play](#how-to-play)
- [Game Mechanics](#game-mechanics)
- [Project Structure](#project-structure)
- [Dependencies](#dependencies)
- [Building for Release](#building-for-release)
- [Troubleshooting](#troubleshooting)
- [License](#license)

---

## About

**Rust Snake** is a modern implementation of the classic Snake arcade game, built using the [Piston game engine](https://www.piston.rs/) in Rust. The game demonstrates smooth animations, grid-based movement, and progressive difficulty scaling while maintaining the nostalgic feel of the original.

The implementation showcases Rust's performance capabilities combined with Piston's 2D graphics rendering, resulting in a responsive and visually appealing gaming experience.

---

## Features

- 🎮 **Classic Snake Gameplay** - Eat food, grow longer, avoid collisions
- ✨ **Smooth Animations** - Interpolated movement with easing functions for fluid visual experience
- 🎯 **Grid-Based Movement** - Precise 20x20 grid system for predictable gameplay
- 📈 **Progressive Difficulty** - Snake speed increases as you consume more food
- 🎨 **Modern Visual Design** - Clean aesthetic with distinct colors for snake head, body, and food
- ⚡ **60 FPS Rendering** - Smooth visual updates at 60 frames per second
- 🖥️ **Fixed Window Size** - Consistent 800x600 resolution across all systems

---

## Screenshots

> 📸 *Screenshots coming soon!* 

*Feel free to contribute by adding gameplay screenshots to this section.*

---

## Prerequisites

Before building this project, ensure you have the following installed:

### Rust Toolchain

Install Rust using `rustup` (recommended):

```bash
# Install rustup (Rust installer and version manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the stable Rust toolchain
rustup install stable

# Verify installation
rustc --version
cargo --version
```

### System Dependencies

Piston requires some system libraries for graphics and window management:

#### Linux (Ubuntu/Debian)

```bash
sudo apt-get install \
    libasound2-dev \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libxcursor-dev \
    libxinerama-dev \
    libgl1-mesa-dev \
    libwayland-dev \
    libxkbcommon-dev
```

#### Linux (Fedora)

```bash
sudo dnf install \
    alsa-lib-devel \
    libX11-devel \
    libXrandr-devel \
    libXi-devel \
    libXcursor-devel \
    libXinerama-devel \
    mesa-libGL-devel \
    wayland-devel \
    libxkbcommon-devel
```

#### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows

Windows users typically don't need additional dependencies. Visual Studio Build Tools or MSVC should be sufficient.

---

## Installation

### Clone the Repository

```bash
git clone https://github.com/your-username/rust-snake.git
cd rust-snake
```

### Build and Run

```bash
# Build and run in development mode
cargo run

# Or build first, then run
cargo build
cargo run
```

The game window should open immediately with the title "Snake".

---

## How to Play

### Controls

| Key | Action |
|-----|--------|
| `↑` (Up Arrow) | Move snake upward |
| `↓` (Down Arrow) | Move snake downward |
| `←` (Left Arrow) | Move snake leftward |
| `→` (Right Arrow) | Move snake rightward |
| `ESC` | Exit the game |

### Objective

1. **Eat the red food** to grow your snake
2. **Avoid collisions** with walls and your own body
3. **Survive as long as possible** while achieving the highest score

### Starting the Game

- The snake starts at position (0, 0) moving right
- Food spawns randomly within the game window
- The snake grows each time it consumes food
- Speed increases progressively with each food consumed

---

## Game Mechanics

### Snake Movement

The snake uses a sophisticated animation system:

- **Grid-based positioning**: Movement snaps to a 20-pixel grid
- **Smooth interpolation**: Positions are interpolated between grid cells using quadratic easing
- **Direction locking**: Cannot reverse direction (e.g., can't go left while moving right)
- **Progressive speed**: Movement speed increases by 10% with each food consumed (capped at minimum 0.1)

### Collision Detection

- **Food collision**: Snake head coordinates must match food coordinates exactly
- **Wall collision**: Game ends when snake hits window boundaries
- **Self-collision**: Game ends when snake head collides with any body segment

### Visual Design

| Element | Color (RGBA) | Description |
|---------|--------------|-------------|
| Background | `[0.15, 0.15, 0.15, 1.0]` | Dark gray background |
| Snake Head | `[0.0, 1.0, 0.6, 1.0]` | Bright cyan-green |
| Snake Body | `[0.3, 0.3, 0.3, 1.0]` | Medium gray |
| Food | `[1.0, 0.2, 0.2, 1.0]` | Bright red (60% of grid size) |

---

## Project Structure

```
rust-snake/
├── Cargo.toml              # Project manifest and dependencies
├── README.md               # This documentation file
├── LICENSE                 # MIT License
└── src/
    ├── main.rs             # Entry point, window configuration
    ├── app.rs              # Main game application logic
    ├── snake.rs            # Snake entity with movement and rendering
    ├── food.rs             # Food spawning and rendering
    └── vec2.rs             # 2D vector utility for positions
```

### Module Descriptions

#### `main.rs`
Game entry point responsible for:
- Creating the Piston window (800x600)
- Configuring update rate (60 UPS) and frame rate (60 FPS)
- Initializing the game application
- Running the main game loop (render, update, input)

#### `app.rs`
Core game application containing:
- Game state management (snake, food, window dimensions)
- Render pipeline (clear screen, draw food, draw snake)
- Update loop (snake movement, collision detection)
- Input handling (keyboard events)

#### `snake.rs`
Snake entity implementation:
- Head and body segment management
- Direction state machine (LEFT, RIGHT, UP, DOWN)
- Smooth animation with easing functions
- Speed progression system
- Collision detection with food

#### `food.rs`
Food system:
- Random position spawning within window bounds
- Grid-aligned placement
- Rendering with offset sizing

#### `vec2.rs`
2D vector utility:
- Position representation
- Collision detection
- Random position generation

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `piston_window` | 0.132.0 | Game window and event loop |
| `piston2d-graphics` | 0.45.0 | 2D graphics rendering |
| `rand` | 0.9.0 | Random number generation for food spawning |

---

## Building for Release

To create an optimized production build:

```bash
# Build in release mode with optimizations
cargo build --release

# Run the release build
cargo run --release
```

Release builds are significantly faster due to Rust's optimization passes.

### Binary Location

After building:
- **Debug**: `target/debug/snake`
- **Release**: `target/release/snake`

---

## Troubleshooting

### Common Issues

#### "Failed to build PistonWindow"

**Cause**: Missing system dependencies or graphics drivers.

**Solution**:
1. Install system dependencies (see [Prerequisites](#prerequisites))
2. Update graphics drivers
3. Ensure you're running in a graphical environment (not SSH without X11 forwarding)

#### Game runs slowly or stutters

**Cause**: Running debug build or insufficient hardware.

**Solution**:
```bash
# Build in release mode for better performance
cargo run --release
```

#### Window doesn't open

**Cause**: Wayland/X11 issues on Linux.

**Solution**:
```bash
# Try forcing X11 backend
export GDK_BACKEND=x11
cargo run

# Or try running with Wayland
export GDK_BACKEND=wayland
cargo run
```

#### Input lag or unresponsive controls

**Cause**: Event loop timing issues.

**Solution**:
- Ensure no other application is capturing keyboard input
- Try running in a clean terminal session
- Check that 60 UPS/FPS settings match your monitor's refresh rate

#### Compilation errors

**Cause**: Outdated Rust version or dependency conflicts.

**Solution**:
```bash
# Update Rust toolchain
rustup update

# Clear build cache and rebuild
cargo clean
cargo build
```

### Performance Tips

- **Release mode**: Always use `--release` for best performance
- **Close background applications**: Free up system resources
- **Update graphics drivers**: Ensure optimal GPU performance

---

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

### Summary

- ✅ Free to use for personal and commercial projects
- ✅ Modify and distribute
- ✅ Include original copyright notice
- ❌ No warranty provided

---

## Contributing

Contributions are welcome! Feel free to:

- 🐛 Report bugs
- 💡 Suggest features
- 🎨 Add screenshots or visual improvements
- 📝 Improve documentation
- 🔧 Submit pull requests

---

## Acknowledgments

- [Piston Game Engine](https://www.piston.rs/) - Excellent Rust game development framework
- Classic Snake arcade game - Original concept by Gremlin Industries (1976)

---

**Happy Gaming! 🎮🐍**
