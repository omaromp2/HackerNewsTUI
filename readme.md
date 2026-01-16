# Hacker News TUI

A simple and fast terminal user interface for browsing Hacker News, built with Rust and [ratatui](https://github.com/ratatui-org/ratatui).

<!-- ![HN TUI Screenshot](https://via.placeholder.com/800x600?text=Hacker+News+TUI+Screenshot) --> 

## Features

- **Browse Top Stories**: View the latest top, new, best, and other Hacker News story categories
- **Keyboard Navigation**: Efficient keyboard-first controls for power users
- **Open Links**: Launch stories directly in your default browser
- **Pagination**: Load more stories on demand
- **Details View**: Toggle detailed story information
- **Responsive UI**: Clean, readable interface built with ratatui

## Controls

| Key | Action |
|-----|--------|
| `j` / `Down` | Move selection down |
| `k` / `Up` | Move selection up |
| `Space` | Switch story category (top, new, best, show, ask, jobs) |
| `o` | Open story in browser |
| `d` | Toggle story details |
| `m` | Load more stories |
| `r` | Refresh stories |
| `PageDown` | Scroll down one page |
| `PageUp` | Scroll up one page |
| `Home` | Jump to first item |
| `End` | Jump to last item |
| `q` | Quit |

## Installation

### Prerequisites

- Rust 1.56 or later
- Cargo

### Build from Source

```bash
git clone https://github.com/omaromp2/hn-tui.git
cd hn-tui
cargo build --release
```

### Install

```bash
cargo install --path .
```

<!-- Or download a prebuilt binary from the [Releases](https://github.com/yourusername/hn-tui/releases) page. --> 

## Usage

Simply run:

```bash
hn-tui
```

## Dependencies

- `ratatui` - Terminal user interface library
- `crossterm` - Terminal handling library
- `reqwest` - HTTP client for API requests
- `tokio` - Async runtime
- `serde` / `serde_json` - JSON serialization
- `chrono` - Date/time handling
- `open` - Open URLs in browser

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

<!-- ## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. --> 

## Acknowledgments

- [Hacker News API](https://github.com/HackerNews/API) for providing the data
- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI library
