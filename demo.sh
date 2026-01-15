#!/bin/bash

# OxiPlayer Demo Script
# This script demonstrates the TUI music player functionality

echo "🎵 OxiPlayer Demo"
echo "=================="
echo ""

# Build the project
echo "Building OxiPlayer..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Check if example music directory exists
if [ ! -d "example_music" ]; then
    echo "Creating example music directory..."
    mkdir -p example_music

    # Create some placeholder files for demonstration
    echo "# Sample MP3 - Replace with real audio files" > example_music/sample1.mp3
    echo "# Sample WAV - Replace with real audio files" > example_music/song.wav
    echo "# Sample FLAC - Replace with real audio files" > example_music/track.flac
    echo "# Another MP3 - Replace with real audio files" > example_music/music.mp3
    echo "# OGG file - Replace with real audio files" > example_music/audio.ogg
fi

echo "📁 Demo music directory: $(pwd)/example_music"
echo "📊 Files found:"
ls -la example_music/

echo ""
echo "🎮 Controls:"
echo "  ↑/↓ or j/k  - Navigate"
echo "  Enter/Space - Play selected"
echo "  p           - Pause/Resume"
echo "  s           - Stop"
echo "  +/-         - Volume up/down"
echo "  r           - Refresh files"
echo "  q           - Quit"
echo ""

echo "🚀 Starting OxiPlayer..."
echo "   (Press 'q' to quit when you're done)"
echo ""

# Run with the example directory
cargo run --release example_music

echo ""
echo "👋 Demo finished!"
echo ""
echo "💡 Tips:"
echo "  - Replace files in example_music/ with real audio files to test playback"
echo "  - Run with any directory: cargo run /path/to/your/music"
echo "  - Supports: MP3, WAV, FLAC, OGG, M4A, AAC"
