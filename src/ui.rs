use crate::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Footer/Status
        ])
        .split(f.size());

    // Draw header
    draw_header(f, chunks[0], app);

    // Draw main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // File list
            Constraint::Percentage(30), // Info panel
        ])
        .split(chunks[1]);

    draw_file_list(f, main_chunks[0], app);
    draw_info_panel(f, main_chunks[1], app);

    // Draw footer
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("🎵 ", Style::default().fg(Color::Yellow)),
            Span::styled("OxiPlayer", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Rust TUI Music Player"),
        ]),
        Line::from(vec![
            Span::raw("Directory: "),
            Span::styled(
                app.music_directory.display().to_string(),
                Style::default().fg(Color::Green),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(title, area);
}

fn draw_file_list(f: &mut Frame, area: Rect, app: &App) {
    if app.music_files.is_empty() {
        let empty_message = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("No music files found", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Supported formats: "),
                Span::styled("MP3, WAV, FLAC, OGG, M4A, AAC", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("'r'", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" to refresh the file list"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Or specify a different directory when starting:"),
            ]),
            Line::from(vec![
                Span::styled("cargo run /path/to/music", Style::default().fg(Color::Gray)),
            ]),
        ];

        let empty_widget = Paragraph::new(empty_message)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Music Files (0/0)")
                    .border_style(Style::default().fg(Color::White)),
            );
        f.render_widget(empty_widget, area);
        return;
    }

    let items: Vec<ListItem> = app
        .music_files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if Some(&file.name) == app.current_playing.as_ref() {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else if i == app.selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if Some(&file.name) == app.current_playing.as_ref() {
                "♪ "
            } else if i == app.selected_index {
                "> "
            } else {
                "  "
            };

            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(&file.name, style),
            ]))
        })
        .collect();

    let title = format!("Music Files ({}/{})",
                       app.selected_index + 1,
                       app.music_files.len());

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn draw_info_panel(f: &mut Frame, area: Rect, app: &App) {
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Currently playing info
            Constraint::Min(0),     // Controls help
        ])
        .split(area);

    // Currently playing section
    let currently_playing = if let Some(ref playing) = app.current_playing {
        let status_text = if app.is_paused { "Paused" } else { "Playing" };
        let status_color = if app.is_paused { Color::Yellow } else { Color::Green };

        vec![
            Line::from(vec![
                Span::styled("Now Playing:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("♪ ", Style::default().fg(Color::Yellow)),
                Span::raw(playing),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Cyan)),
                Span::styled(status_text, Style::default().fg(status_color)),
            ]),
            Line::from(vec![
                Span::styled("Volume: ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{}%", (app.volume * 100.0) as u8), Style::default().fg(Color::White)),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("Now Playing:", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("No music playing", Style::default().fg(Color::Gray)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Cyan)),
                Span::styled("Stopped", Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("Volume: ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{}%", (app.volume * 100.0) as u8), Style::default().fg(Color::White)),
            ]),
        ]
    };

    let now_playing = Paragraph::new(currently_playing)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Player Info")
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(now_playing, info_chunks[0]);

    // Controls help section
    let controls = vec![
        Line::from(vec![
            Span::styled("Controls:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("↑/k    - Move up"),
        Line::from("↓/j    - Move down"),
        Line::from("Enter  - Play selected"),
        Line::from("n      - Play next song"),
        Line::from("p      - Play previous song"),
        Line::from("Space  - Pause/Resume"),
        Line::from("s      - Stop playback"),
        Line::from("+/-    - Volume up/down"),
        Line::from("r      - Refresh files"),
        Line::from("q      - Quit"),
    ];

    let help = Paragraph::new(controls)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(help, info_chunks[1]);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let status_style = if app.current_playing.is_some() {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::Cyan)),
        Span::styled(&app.status_message, status_style),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(footer, area);
}
