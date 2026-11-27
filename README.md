# touchpainted

<p align="center">
  <img src="assets/banner.svg" alt="touchpa(inte)d banner" width="100%"/>
</p>

<p align="center">
  <a href="https://github.com/TunahanBasri/touchpainted/actions"><img src="https://img.shields.io/github/actions/workflow/status/TunahanBasri/touchpainted/release.yml?branch=main&style=flat-square&label=build" alt="Build status"/></a>
  <a href="https://github.com/TunahanBasri/touchpainted/releases/latest"><img src="https://img.shields.io/github/v/release/TunahanBasri/touchpainted?style=flat-square&label=release" alt="Latest release"/></a>
  <img src="https://img.shields.io/badge/platform-Windows%2010%20%7C%2011%20x64-blue?style=flat-square" alt="Platform"/>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-non--commercial-orange?style=flat-square" alt="License"/></a>
</p>

<p align="center">
  <a href="README.tr.md">Türkçe</a> ·
  <a href="https://github.com/TunahanBasri/touchpainted/releases/latest">Download</a>
</p>

touchpainted is a small Windows tray application that turns a Precision Touchpad into a gesture launcher. Double-tap the touchpad with two fingers, draw a letter with one or two fingers without pressing down, and the program bound to that letter starts.

It is written in Rust with no external crates. It calls Win32 and the Windows HID parser (`hid.dll`) directly, and the release build is a single executable of about 230 KB.

<p align="center">
  <img src="assets/demo.gif" alt="Two-finger double-tap on a laptop touchpad, then drawing the letter C" width="780"/>
</p>

## Features

- **Raw touchpad input.** Reads Precision Touchpad reports (HID usage page `0x0D`, usage `0x05`) directly, so two-finger strokes are captured instead of being converted into scrolling by Windows.
- **No pressure needed.** Gliding one or two fingers over the surface is enough; no click or drag.
- **Stable two-finger strokes.** Movement is accumulated from centroid deltas. When a second finger lands or lifts, the stroke does not jump to the new centroid.
- **Auto-fit canvas.** Points are stored in unclamped coordinates and re-centered and scaled into the drawing area, so arcs near the edge are not flattened.
- **Recognition.** Built-in letters (`C`, `V`, `M`, `S`, `W`) are scored with simple geometric rules combined with a [$1 Unistroke](https://depts.washington.edu/acelab/proj/dollar/index.html) template matcher (64 resampled points, ±24° rotation search).
- **Teaching mode.** Record your own version of any letter `A`–`Z`. The template is saved to `config.json` and preferred during matching.
- **Event-driven.** There is no polling loop. The process sleeps in `GetMessageW` and wakes on touchpad input, low-level mouse hook events, and its own timers while the overlay is open.
- **Single instance.** Starting the executable again brings up the running instance instead of launching a second one.

## Requirements

- Windows 10 or 11, x64.
- A Precision Touchpad for gesture input. Without one, the overlay can still be opened with `Ctrl+Space` and drawn on with the mouse by holding the left button inside the canvas.

## Usage

1. Download `touchpainted.exe` (or the zip) from [Releases](https://github.com/TunahanBasri/touchpainted/releases/latest) and run it. It has no installer and lives in the system tray. `config.json` is created next to the executable on first run.
2. Open the overlay with a two-finger double-tap, `Ctrl+Space`, or a left click on the tray icon.
3. Draw a letter and lift your fingers. The bound program starts and the overlay closes.
4. To close without launching, press `Esc`, click the close button, or wait for the idle timeout.

### Teaching a letter

While the overlay is open, press `T` or click the teach button in the title bar. Pick a letter from the buttons at the bottom of the window or press `A`–`Z`, then draw it once. The stroke is normalized and written to `config.json`.

## Configuration

Each entry in `config.json` binds a letter to something `ShellExecuteW` can open: an executable, a folder, a URI such as `ms-settings:`, or an `https://` URL.

```json
{
  "idle_timeout_ms": 4000,
  "gestures": [
    { "letter": "C", "name": "Hesap Makinesi", "app": "calc.exe",     "param": "", "custom_points": [] },
    { "letter": "V", "name": "Dosya Gezgini",  "app": "explorer.exe", "param": "", "custom_points": [] },
    { "letter": "M", "name": "Çizim Tuvali",   "app": "mspaint.exe",  "param": "", "custom_points": [] },
    { "letter": "S", "name": "Ayarlar",        "app": "ms-settings:", "param": "", "custom_points": [] },
    { "letter": "W", "name": "Not Defteri",    "app": "notepad.exe",  "param": "", "custom_points": [] }
  ]
}
```

| Field | Meaning |
| :--- | :--- |
| `idle_timeout_ms` | Inactivity time before the overlay closes. Teaching mode always allows at least 12 seconds. |
| `letter` | The letter to draw. |
| `name` | Label shown in the overlay. |
| `app` | Executable, folder, URI, or URL to open. |
| `param` | Optional command-line arguments. |
| `custom_points` | Normalized template written by teaching mode. Leave empty to use the built-in recognizer. |

The config file can also be opened from the tray menu.

## Building

Requires Rust 1.85 or newer (edition 2024) with the MSVC toolchain. There are no dependencies to download.

```powershell
git clone https://github.com/TunahanBasri/touchpainted.git
cd touchpainted
cargo build --release
.\target\release\touchpainted.exe
```

Run with `--log` to write diagnostic output to `touchpad_log.txt` next to the executable.

## How it works

| Part | Implementation |
| :--- | :--- |
| Touchpad input | `RegisterRawInputDevices` with `HidP_GetValueCaps`, `HidP_GetUsages`, `HidP_GetUsageValue`. Contact collections and X/Y ranges are discovered at runtime, not hardcoded. |
| Tap fallback and scroll blocking | `WH_MOUSE_LL` low-level mouse hook. The hook only posts messages and returns immediately. |
| Stroke tracking | Centroid deltas accumulated into unclamped virtual coordinates. |
| Recognition | Geometric rules plus `$1 Unistroke` matching. |
| Rendering | Double-buffered GDI, tray icon via `Shell_NotifyIconW`. |
| Launching | `ShellExecuteW` on a background thread so the input thread is never blocked. |

## Known limitations

- The user interface text is Turkish only.
- The built-in recognizer covers `C`, `V`, `M`, `S`, `W`. Other letters work after being taught.
- Two-finger double-tap detection depends on the touchpad driver. If it is unreliable on your device, use `Ctrl+Space`.
- The executable is not code-signed, so SmartScreen may show a warning on first run.

## License

Copyright © 2026 TunahanBasri. Released under a [non-commercial personal use license](LICENSE).

Personal and educational use, reading the source, and building it yourself are allowed. Selling the software, distributing it for a fee, or including it in a commercial product requires written permission.
