# ProcDeck – Process Monitor & Manager

**ProcDeck** is a cross-platform system process monitor and manager built with **Rust** and the **Iced GUI framework**.
It provides real-time system insights and control over processes through a clean, modern interface.

---

## Features

* **Real-Time Monitoring:** Live graphs for CPU, Memory, Disk I/O, and Network usage
* **Process Management:** Kill, suspend, resume, and adjust process priorities
* **Smart Filtering:** Search processes by name or PID
* **Intelligent Alerts:** Suggestions for high CPU or memory usage
* **Modern UI:** Clean dark-themed interface with color-coded metrics
* **Cross-Platform:** Works on Windows, macOS, and Linux

---

## System Requirements

| Component       | Requirement                                |
| --------------- | ------------------------------------------ |
| **OS**          | Windows 10+, macOS 10.15+, or modern Linux |
| **Rust**        | 1.70 or higher                             |
| **Memory**      | 100 MB minimum                             |
| **Permissions** | Admin/root required for some operations    |

---

## Dependencies

### Core

* [`iced`](https://github.com/iced-rs/iced) – GUI framework
* [`sysinfo`](https://crates.io/crates/sysinfo) – System info and process stats
* [`serde`](https://crates.io/crates/serde) – Serialization support

### Platform-Specific

| Platform         | Dependencies          |
| ---------------- | --------------------- |
| **Windows**      | `windows-sys` (v0.59) |
| **Unix / macOS** | `nix`, `libc`         |

---

## Installation & Build

### 1. Prerequisites

Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository:

```bash
git clone https://github.com/zinmyolatt-kmitl/process_monitor_and_manager_in_rust
cd process_monitor_and_manager_in_rust
```

---

### 2. Building from Source

**Debug Build (for development):**

```bash
cargo build
```

**Release Build (optimized):**

```bash
cargo build --release
```

**Binary Location:**

```
target/debug/process_monitor_and_manager
target/release/process_monitor_and_manager
```

---

### 3. Running the Application

**Development Mode**

```bash
cargo run
```

**Release Mode**

```bash
cargo run --release
```

**Or run the binary directly**

```bash
# Linux/macOS
./target/release/process_monitor_and_manager

# Windows
.\target\release\process_monitor_and_manager.exe
```

---

## Platform-Specific Notes

### Windows

Add these features in `Cargo.toml` (already included):

```toml
[dependencies.windows-sys]
version = "0.59"
features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_Diagnostics",
    "Win32_System_Diagnostics_ToolHelp",
]
```

> Some operations (suspend/resume/priority) require **Administrator** privileges.

---

### macOS

No additional setup is required.
Some process control operations may need `sudo`.

---

### Linux

Install required development libraries:

```bash
sudo apt-get install libxkbcommon-dev libwayland-dev
sudo apt-get install libx11-dev
```

---

## Configuration

### `Cargo.toml`

```toml
[package]
name = "process_monitor_and_manager"
version = "0.1.0"
edition = "2021"

[dependencies]
iced = { version = "0.13", features = ["canvas", "tokio"] }
iced_widget = "0.13"
sysinfo = "0.32"
serde = { version = "1.0", features = ["derive"] }

[target.'cfg(target_family = "windows")'.dependencies]
windows-sys = { version = "0.59", features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_Diagnostics",
    "Win32_System_Diagnostics_ToolHelp",
] }

[target.'cfg(target_family = "unix")'.dependencies]
nix = { version = "0.29", features = ["signal"] }
libc = "0.2"
```

---

## Usage Guide

1. Launch the application
2. Use the **Filter** box to search by process name or PID
3. Click column headers to sort by CPU, memory, or other metrics
4. Use action buttons (**Kill**, **Suspend**, **Resume**, **Boost**, **Lower**)
5. Start new processes via the **Start Command** field
6. View real-time system graphs below the process table
7. Toggle CPU/Memory alerts via checkboxes

---

## Troubleshooting

### Build Errors

**Error:** `could not find Diagnostics in System`
→ Add missing feature flags in `Cargo.toml`:

```toml
features = [..., "Win32_System_Diagnostics", "Win32_System_Diagnostics_ToolHelp"]
```

**Error:** `linking with cc failed (Linux)`
→ Install required build tools:

```bash
sudo apt-get install build-essential pkg-config
```

---

### Runtime Issues

| Issue                                      | Solution                                                                   |
| ------------------------------------------ | -------------------------------------------------------------------------- |
| **Access Denied** when suspending/resuming | Run as Administrator (Windows) or with `sudo` (macOS/Linux)                |
| **High CPU usage**                         | Normal on startup; adjust `TICK` duration in `app.rs` (default: 700 ms)    |
| **Graphs not updating**                    | Ensure `sysinfo` can access system metrics; some need elevated permissions |

---

## Project Structure

```
procdeck/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application logic
│   ├── models.rs            # Data structures
│   ├── view.rs              # UI components
│   ├── styles.rs            # Custom styling
│   ├── graphs.rs            # Graph visualization
│   ├── platform.rs          # OS-specific operations
│   ├── system_monitor.rs    # System metrics collection
│   ├── suggestions.rs       # Alert logic
│   ├── util.rs              # Helper functions
│   └── lib.rs               # Module exports
├── Cargo.toml
└── README.md
```

---

## Contributing

1. Fork the repository
2. Create a feature branch:

   ```bash
   git checkout -b feature/new-feature
   ```
3. Commit your changes:

   ```bash
   git commit -m "Add new feature"
   ```
4. Push and open a pull request

---


## Authors

* **Zin Myo Latt** 
* **Chan Myae San** 
* **Arkar Zaw Htet** 



---

## Acknowledgments

* Built with [Iced GUI Framework](https://github.com/iced-rs/iced)
* System data via [sysinfo](https://crates.io/crates/sysinfo)
* Inspired by traditional process managers: *Task Manager*, *htop*, *Activity Monitor*

---

## Support

For issues or contributions:


* contact: **[lattzinmyo87@gmail.com](mailto:lattzinmyo87@gmail.com)**


