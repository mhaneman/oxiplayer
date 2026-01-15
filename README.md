# OxiPlayer 🎵

A simple, elegant TUI (Terminal User Interface) music player written in Rust using ratatui.

## Features

- Browse and play music files from any directory
- Support for common audio formats: MP3, WAV, FLAC, OGG, M4A, AAC
- Clean, intuitive terminal interface
- Keyboard navigation and controls
- Real-time status updates

## Installation

### Prerequisites

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

### Building from source

```bash
git clone <your-repo-url>
cd oxiplayer
cargo build --release
```

## Usage

### Running the player

```bash
# Use default music directory (usually ~/Music or current directory)
cargo run

# Or specify a custom music directory
cargo run /path/to/your/music/folder
```

### Controls

| Key | Action |
|-----|--------|
| `↑` or `k` | Move up in the file list |
| `↓` or `j` | Move down in the file list |
| `Enter` or `Space` | Play selected track |
| `p` | Pause/Resume playback |
| `s` | Stop current playback |
| `+` or `=` | Volume up |
| `-` | Volume down |
| `r` | Refresh file list |
| `q` | Quit the application |

## Interface

The TUI is divided into several sections:

- **Header**: Shows the application title and current music directory
- **File List**: Displays all music files found in the directory
- **Player Info**: Shows currently playing track, playback status, and volume level
- **Help Panel**: Lists available controls
- **Status Bar**: Displays current status and messages

## Supported Audio Formats

- MP3
- WAV
- FLAC
- OGG
- M4A
- AAC

## Features in Detail

### Audio Controls
- Play any supported audio file
- Pause and resume playback with `p`
- Stop playback with `s`
- Volume control with `+`/`-` keys (0-100%)

### File Management
- Automatically scans directory for music files
- Supports recursive directory scanning
- Real-time file list refresh with `r`
- Intelligent file filtering by extension

### User Interface
- Clean, responsive terminal interface
- Visual indicators for currently playing track
- Real-time status updates and feedback
- Intuitive keyboard navigation

## Dependencies

- `ratatui` - Terminal user interface framework
- `crossterm` - Cross-platform terminal manipulation
- `rodio` - Audio playback library
- `walkdir` - Recursive directory walking
- `dirs` - Platform-specific directory detection
- `anyhow` - Error handling

## System Requirements

- Linux, macOS, or Windows
- Audio output device
- Terminal emulator

## Troubleshooting

### No audio output
- Ensure your system has audio drivers installed
- Check that your audio device is not muted
- Verify that other applications can play audio

### No files found
- Make sure the directory contains supported audio files
- Check file permissions
- Use the `r` key to refresh the file list

### Build errors
- Ensure you have the latest Rust toolchain
- Install system audio libraries (ALSA on Linux, etc.)

## Contributing

Feel free to open issues or submit pull requests to improve the player!

## License

This project is open source. See LICENSE file for details.