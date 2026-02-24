# ◈ NEXMON

A futuristic, animated system monitor CLI tool built in Rust. It's designed to be visually stunning, real-time, and highly performant. 

*Nexmon provides an ultra-modern alternative to `htop` or `top` with cyber-aesthetic visuals including sparklines, neon accent colors, and box-drawing layouts.*

![Nexmon Screenshot Description]
> The terminal window features a stylized double-border layout. At the top is a bright neon header with glitch aesthetic bounds. Below it, the CPU section displays dynamic gauges transitioning from green to yellow to red for each core, with an overall sparkline dancing underneath. To the right, memory bars for RAM (Cyan) and Swap (Magenta) breathe vividly with usage. The middle section shows network interfaces with incoming (Green) and outgoing (Yellow) sparkline histograms moving in real-time. Finally, the bottom section features a sortable process table highlighted in dark gray and accented heavily in cyan and pink for high-usage applications. The footer provides an interactive command bar.

## Installation

### Pre-built Binaries
You can download the pre-built binaries for Windows, macOS, and Linux from the [Releases](https://github.com/user/nexmon/releases) page.

### Build from source
Ensure you have the Rust toolchain installed.

```bash
git clone https://github.com/hozi8-web3/nexmon.git
cd nexmon
cargo build --release
```

The binary will be available at `./target/release/nexmon`.

## Usage

Run the `nexmon` binary directly:

```bash
nexmon
```

### CLI Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `-i, --interval <MS>` | Refresh interval in milliseconds | 500 |
| `--show-loopback` | Show loopback network interfaces | false |
| `-p, --processes <NUM>` | Max number of processes to show | 100 |
| `-s, --sort <COL>` | Sort processes by: cpu, mem, pid, name | cpu |

## Keybindings

| Key | Action |
|-----|--------|
| `q` or `Q` | Quit the application |
| `j` or `↓` | Scroll processes down |
| `k` or `↑` | Scroll processes up |
| `c` | Sort by CPU% |
| `m` | Sort by Memory |
| `p` | Sort by PID |
| `n` | Sort by Name |
| `r` | Reverse sort order |
| `/` | Enter search/filter mode |
| `Enter` | Exit search/filter mode |
| `Esc` | Clear search and exit mode |

## Built With
- `tokio` - Async runtime
- `ratatui` - Terminal UI framework
- `sysinfo` - Cross-platform system info
- `crossterm` - Cross-platform terminal interactions

## License
MIT License
