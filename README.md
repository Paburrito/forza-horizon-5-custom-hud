# Forza Horizon 5 — Custom HUD

A lightweight overlay HUD for Forza Horizon 5, made by making a visual mix between NFSU2 default HUD and Forza Horizon 5's HUD, built with Tauri and Rust. 
Displays a real-time tachometer, speed, gear indicator, and live HP/Torque/Boost gauges by reading Forza's telemetry data output.

![HUD Preview](src-tauri/icons/Square150x150Logo.png)

## Features

- Real-time tachometer with automatic rev limiter learning
- Speed display in KM/H or MPH
- Live HP, Torque and Boost gauges
- Gear indicator with shift light
- Click-through window lock — overlay the game without interference
- Demo mode for setup without the game running
- First-run tutorial
- Remembers every car's rev limiter across sessions

## Download

Grab the latest `.exe` from the [Releases](../../releases) page — no install required, it's meant to run as a standalone app.

## Setup

1. Open Forza Horizon 5
2. Go to **Settings → HUD and Gameplay**
3. Toggle **Data Out** ON
4. Set **Data Out IP Address** to `127.0.0.1`
5. Set **Data Out IP Port** to `5300`
6. Turn off the in-game **Speedometer** (so you're not running two HUDs)
7. Launch the HUD — the built-in tutorial will walk you through the rest

## Controls & Hotkeys

| Hotkey | Action |
|--------|--------|
| Drag titlebar | Move window |
| `Ctrl+L` | Lock / unlock window & enable click-through |
| `Ctrl+S` | Save current window position |
| `Ctrl+Shift+R` | Restore saved position |
| `Ctrl+R` | Force re-learn rev limiter for current car |
| `Ctrl+Alt+R` | Quick restart the HUD |

More info can be found in the Info section of the app once it's running

## Building from Source

### Prerequisites

- [Node.js](https://nodejs.org/) v18 or later
- [Rust](https://rustup.rs/) (stable toolchain)
- Tauri CLI: `cargo install tauri-cli`

### Steps

```bash
git clone https://github.com/yourusername/forza-horizon-5-custom-hud.git
cd forza-horizon-5-custom-hud
npm install
npm run tauri build
```

The compiled executable will be at:
```
src-tauri/target/release/bundle/nsis/Forza Horizon 5 Custom HUD - By Paburrito_x.x.x_x64-setup.exe
```

Or the standalone exe at:
```
src-tauri/target/release/Forza-Horizon-5-Custom-HUD-By-Paburrito.exe
```

## Tech Stack

- [Tauri](https://tauri.app/) — native window and system integration
- [Rust](https://www.rust-lang.org/) — UDP telemetry listener and WebSocket bridge
- Vanilla HTML/CSS/JS — HUD rendering via Canvas API

## Credits

Made by me, **Paburrito** with way too much caffeine, ADHD meds and genuine love for Forza Horizon 5 and an old great that was Need For Speed Underground 2.

## License

MIT — do whatever you want with it, just don't sell it and please give proper credits (link and all) towards this repository.
