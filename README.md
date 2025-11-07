ProcDeck - Process Monitor & Manager
==========================================
A cross-platform system process monitor and manager built with Rust and Iced GUI framework. Monitor CPU, memory, disk I/O, and network usage in real-time, with the ability to control processes directly from the interface.

Features
•	📊 Real-time Monitoring: Live graphs for CPU, Memory, Disk I/O, and Network usage
•	🔍 Process Management: Kill, suspend, resume, and adjust process priorities
•	🎯 Smart Filtering: Search processes by name or PID
•	🚨 Intelligent Alerts: Automatic suggestions for high CPU/memory usage
•	🎨 Modern UI: Clean, dark-themed interface with color-coded metrics
•	🖥️ Cross-Platform: Works on Windows, macOS, and Linux

System Requirements
•	Operating System: Windows 10+, macOS 10.15+, or Linux (any modern distribution)
•	Rust: 1.70 or higher
•	Memory: 100MB RAM minimum
•	Permissions: Administrator/root access for some operations (priority changes, system processes)

Dependencies
Core Dependencies
•	iced - GUI framework
•	sysinfo - System information gathering
•	serde - Serialization support

Platform-Specific
•	Windows: windows-sys (v0.59)
•	Unix/macOS: nix, libc

Installation & Build

Prerequisites
1.	Install Rust (if not already installed):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2.	Clone the repository:
git clone https://github.com/yourusername/procdeck.git
cd procdeck
Building from Source
Debug Build (for development)
cargo build
Release Build (optimized)
cargo build --release
The compiled binary will be in:
•	Debug: target/debug/process_monitor_and_manager (or .exe on Windows)
•	Release: target/release/process_monitor_and_manager (or .exe on Windows)
Running the Application
Development Mode
cargo run
Release Mode
cargo run --release

Or run the binary directly:
# Linux/macOS
./target/release/process_monitor_and_manager

# Windows
.\target\release\process_monitor_and_manager.exe

--------------------------------------------------------

Platform-Specific Notes

Windows

# Cargo.toml already includes:
[dependencies.windows-sys]
version = "0.59"
features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_Diagnostics",
    "Win32_System_Diagnostics_ToolHelp",
]

Some operations (suspend, resume, priority changes) may require running as Administrator.
macOS
No additional setup required. Some operations may require sudo for system processes.

Linux
# Install required development libraries (Ubuntu/Debian)
sudo apt-get install libxkbcommon-dev libwayland-dev

# For X11 support
sudo apt-get install libx11-dev
Configuration
Cargo.toml
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
Usage Quick Start
1.	Launch the application
2.	Filter processes: Type in the "Filter" box to search by name or PID
3.	Sort columns: Click column headers to sort by different metrics
4.	Control processes: Use action buttons (Kill, Suspend, Resume, Boost, Lower)
5.	Start new processes: Enter command in "Start command" field and click "Start"
6.	Monitor graphs: View real-time system metrics at the bottom
7.	Toggle alerts: Enable/disable CPU and Memory alerts with checkboxes
Troubleshooting
Build Errors
Error: could not find Diagnostics in System
# Add the feature flag to Cargo.toml
[dependencies.windows-sys]
features = [..., "Win32_System_Diagnostics", "Win32_System_Diagnostics_ToolHelp"]
Error: linking with cc failed (Linux)
sudo apt-get install build-essential pkg-config
Runtime Issues
"Access Denied" when suspending/resuming processes
•	Windows: Run as Administrator
•	macOS/Linux: Use sudo or run as root
High CPU usage
•	Normal during first few seconds (collecting initial metrics)
•	Adjust TICK duration in app.rs if needed (default: 700ms)
Graphs not updating
•	Check that sysinfo can access system metrics
•	Some metrics may require elevated permissions
Project Structure
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
├── Cargo.toml               # Dependencies
├── README.md                # This file
├── ARCHITECTURE.md          # System architecture
├── USER_GUIDE.md            # Detailed user guide
└── TEST_PLAN.md             # Testing documentation
Contributing
1.	Fork the repository
2.	Create your feature branch (git checkout -b feature/amazing-feature)
3.	Commit your changes (git commit -m 'Add amazing feature')
4.	Push to the branch (git push origin feature/amazing-feature)
5.	Open a Pull Request
License
This project is licensed under the MIT License - see the LICENSE file for details.
Authors
•	Your Name - Initial work
•	Contributor Name - Contributor
Acknowledgments
•	Built with Iced GUI Framework
•	System information via sysinfo
•	Inspired by traditional process managers (Task Manager, htop, Activity Monitor)
Support
For issues, questions, or contributions:
•	Open an issue on GitHub
•	Contact: your.email@example.com
________________________________________
Note: This application requires appropriate permissions to manage system processes. Always use caution when killing or modifying processes.