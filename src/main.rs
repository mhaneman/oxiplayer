use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::ListState,
    Terminal,
};
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

mod audio;
mod ui;

use audio::AudioPlayer;

#[derive(Clone)]
pub struct MusicFile {
    pub path: PathBuf,
    pub name: String,
}

pub struct App {
    pub music_files: Vec<MusicFile>,
    pub selected_index: usize,
    pub list_state: ListState,
    pub audio_player: AudioPlayer,
    pub current_playing: Option<String>,
    pub status_message: String,
    pub music_directory: PathBuf,
    pub is_paused: bool,
    pub volume: f32,
}

impl App {
    pub fn new(music_dir: PathBuf) -> Result<Self> {
        let music_files = Self::scan_music_files(&music_dir)?;
        let mut list_state = ListState::default();
        if !music_files.is_empty() {
            list_state.select(Some(0));
        }

        let status_message = if music_files.is_empty() {
            String::from("No music files found - Press 'r' to refresh or 'q' to quit")
        } else {
            String::from("Ready - Use ↑/↓ to navigate, Enter to play (auto-advances to next song), 'q' to quit")
        };

        Ok(App {
            music_files,
            selected_index: 0,
            list_state,
            audio_player: AudioPlayer::new()?,
            current_playing: None,
            status_message,
            music_directory: music_dir,
            is_paused: false,
            volume: 0.7,
        })
    }

    fn scan_music_files(dir: &PathBuf) -> Result<Vec<MusicFile>> {
        let mut files = Vec::new();
        let music_extensions = ["mp3", "wav", "flac", "ogg", "m4a", "aac"];

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if music_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        if let Some(name) = path.file_name() {
                            files.push(MusicFile {
                                path: path.to_path_buf(),
                                name: name.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }

        files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(files)
    }

    pub fn next(&mut self) {
        if !self.music_files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.music_files.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn previous(&mut self) {
        if !self.music_files.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.music_files.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn play_selected(&mut self) -> Result<()> {
        if self.music_files.is_empty() {
            self.status_message = String::from("No music files available to play");
            return Ok(());
        }

        if let Some(file) = self.music_files.get(self.selected_index) {
            match self.audio_player.play(&file.path) {
                Ok(_) => {
                    self.current_playing = Some(file.name.clone());
                    self.is_paused = false;
                    self.audio_player.set_volume(self.volume);
                    self.status_message = format!("♪ Playing: {}", file.name);
                }
                Err(e) => {
                    self.status_message = format!("Error playing file: {}", e);
                }
            }
        } else {
            self.status_message = String::from("No file selected");
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        self.audio_player.stop();
        self.current_playing = None;
        self.is_paused = false;
        self.status_message = String::from("Stopped");
    }

    pub fn pause(&mut self) {
        if self.current_playing.is_some() && !self.is_paused {
            self.audio_player.pause();
            self.is_paused = true;
            self.status_message = String::from("Paused");
        }
    }

    pub fn resume(&mut self) {
        if self.current_playing.is_some() && self.is_paused {
            self.audio_player.resume();
            self.is_paused = false;
            if let Some(ref name) = self.current_playing {
                self.status_message = format!("♪ Playing: {}", name);
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.current_playing.is_some() {
            if self.is_paused {
                self.resume();
            } else {
                self.pause();
            }
        }
    }

    pub fn play_next(&mut self) -> Result<()> {
        if !self.music_files.is_empty() {
            let was_at_end = self.selected_index == self.music_files.len() - 1;
            self.next();
            self.play_selected()?;

            // Show special message when looping back to start
            if was_at_end {
                if let Some(file) = self.music_files.get(0) {
                    self.status_message = format!("♪ Looped to beginning - Playing: {}", file.name);
                }
            }
        }
        Ok(())
    }

    pub fn play_previous(&mut self) -> Result<()> {
        if !self.music_files.is_empty() {
            self.previous();
            self.play_selected()?;
        }
        Ok(())
    }

    pub fn volume_up(&mut self) {
        self.volume = (self.volume + 0.1).min(1.0);
        self.audio_player.set_volume(self.volume);
        self.status_message = format!("Volume: {}%", (self.volume * 100.0) as u8);
    }

    pub fn volume_down(&mut self) {
        self.volume = (self.volume - 0.1).max(0.0);
        self.audio_player.set_volume(self.volume);
        self.status_message = format!("Volume: {}%", (self.volume * 100.0) as u8);
    }

    pub fn refresh_files(&mut self) -> Result<()> {
        self.music_files = Self::scan_music_files(&self.music_directory)?;
        if self.selected_index >= self.music_files.len() && !self.music_files.is_empty() {
            self.selected_index = self.music_files.len() - 1;
        }
        if !self.music_files.is_empty() {
            self.list_state.select(Some(self.selected_index));
        } else {
            self.list_state.select(None);
        }
        if self.music_files.is_empty() {
            self.status_message = String::from("No music files found in directory");
        } else {
            self.status_message = format!("Refreshed - Found {} music files", self.music_files.len());
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // Get music directory from command line args or use default
    let args: Vec<String> = std::env::args().collect();
    let music_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };

    if !music_dir.exists() {
        eprintln!("Error: Directory '{}' does not exist", music_dir.display());
        eprintln!("Usage: {} [music_directory]", args[0]);
        std::process::exit(1);
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(music_dir)?;

    // Main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Check if current song has finished and auto-play next
        if app.current_playing.is_some() && !app.is_paused && app.audio_player.is_empty() {
            app.status_message = String::from("Auto-advancing to next song...");
            app.play_next()?;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        app.play_selected()?;
                    }
                    KeyCode::Char('s') => app.stop(),
                    KeyCode::Char(' ') => app.toggle_pause(),
                    KeyCode::Char('+') => app.volume_up(),
                    KeyCode::Char('-') => app.volume_down(),
                    KeyCode::Char('r') => {
                        app.refresh_files()?;
                    }
                    KeyCode::Char('n') => {
                        app.play_next()?;
                    }
                    KeyCode::Char('p') => {
                        app.play_previous()?;
                    }
                    _ => {}
                }
            }
        }
    }
}
