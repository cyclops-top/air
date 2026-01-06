use crate::handlers::AppState;
use crate::view::format_size;
use qrcode::render::unicode;
use qrcode::QrCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::sync::Arc;

pub struct DashboardState {
    pub scroll_offset: usize,
    pub lan_ip: String,
    pub port: u16,
    pub hostname: Option<String>,
}

pub fn render(f: &mut Frame, app_state: &Arc<AppState>, ui_state: &DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(20),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    // 1. Render a single outer block for the entire header area
    let header_block = Block::default()
        .borders(Borders::ALL)
        .title(" Air Dashboard ");
    f.render_widget(header_block.clone(), chunks[0]);

    // 2. Get the inner area of the header block for splitting
    let inner_header = header_block.inner(chunks[0]);

    // Split inner header into QR Code (left) and Info (right)
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(36), // Increased width for QR
            Constraint::Min(0),
        ])
        .split(inner_header);

    // QR Code (Now on the left, without its own full border to save space)
    let protocol = if app_state.enable_https {
        "https"
    } else {
        "http"
    };
    let url = format!("{}://{}:{}", protocol, ui_state.lan_ip, ui_state.port);
    if let Ok(code) = QrCode::new(url.as_bytes()) {
        let qr_string = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        let qr_paragraph = Paragraph::new(qr_string)
            .alignment(Alignment::Center)
            // Use a thin separator on the right instead of full borders
            .block(Block::default().borders(Borders::RIGHT).title(" Scan Me "));
        f.render_widget(qr_paragraph, header_chunks[0]);
    }

    // Header Info (Now on the right)
    let stats = &app_state.stats;
    let files = stats.total_files.load(std::sync::atomic::Ordering::Relaxed);
    let bytes = stats.total_bytes.load(std::sync::atomic::Ordering::Relaxed);

    let mut header_text = vec![
        Line::from(vec![
            Span::styled(
                " Air File Server ",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Serving: "),
            Span::styled(
                app_state.root_path.to_string_lossy(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  ➜ Network: "),
            Span::styled(
                format!("{}://{}:{}", protocol, ui_state.lan_ip, ui_state.port),
                Style::default().fg(Color::Green),
            ),
        ]),
    ];

    if let Some(ref h) = ui_state.hostname {
        header_text.push(Line::from(vec![
            Span::raw("  ➜ Host:    "),
            Span::styled(
                format!("{}://{}:{}", protocol, h, ui_state.port),
                Style::default().fg(Color::Green),
            ),
        ]));
    }

    header_text.extend(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" [Stats] ", Style::default().fg(Color::Cyan)),
            Span::raw("Files: "),
            Span::styled(
                files.to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | Volume: "),
            Span::styled(
                format_size(bytes),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | Sandbox: "),
            Span::styled("ENABLED 🔒", Style::default().fg(Color::Green)),
        ]),
    ]);

    if app_state.enable_https {
        header_text.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(" [Performance] ", Style::default().fg(Color::Magenta)),
            Span::styled(
                "HTTP/2 Multiplexing: ENABLED 🚀",
                Style::default().fg(Color::White),
            ),
        ]));
    }

    let header_info = Paragraph::new(header_text)
        .block(Block::default().padding(ratatui::widgets::Padding::new(2, 0, 0, 0))); // Add padding instead of borders
    f.render_widget(header_info, header_chunks[1]);

    // Logs
    let logs_lock = stats.logs.lock().unwrap();
    let logs_count = logs_lock.len();

    // Calculate displayable area for logs
    let list_height = chunks[1].height as usize - 2; // -2 for borders

    // If we have more logs than can fit, we might want to default to bottom
    // But the user requested manual scroll.
    // Let's implement a simple scroll: ui_state.scroll_offset is how many lines from BOTTOM we are offset.
    // 0 means show the latest logs.

    let display_logs: Vec<ListItem> = if logs_count > 0 {
        let start = if logs_count > list_height + ui_state.scroll_offset {
            logs_count - list_height - ui_state.scroll_offset
        } else {
            0
        };
        let end = (start + list_height).min(logs_count - ui_state.scroll_offset);

        logs_lock
            .iter()
            .skip(start)
            .take(end - start)
            .map(|log| ListItem::new(log.as_str()))
            .collect()
    } else {
        vec![]
    };

    let logs_list = List::new(display_logs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Request Logs ({}) ", logs_count)),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(logs_list, chunks[1]);

    // Footer
    let footer_text = format!(
        " Press 'q' to quit | Up/Down to scroll ({}) ",
        ui_state.scroll_offset
    );
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
