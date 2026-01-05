use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::sync::Arc;
use crate::handlers::AppState;
use crate::view::format_size;

pub struct DashboardState {
    pub scroll_offset: usize,
    pub lan_ip: String,
    pub port: u16,
}

pub fn render(f: &mut Frame, app_state: &Arc<AppState>, ui_state: &DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    // Header
    let stats = &app_state.stats;
    let files = stats.total_files.load(std::sync::atomic::Ordering::Relaxed);
    let bytes = stats.total_bytes.load(std::sync::atomic::Ordering::Relaxed);
    
    let header_text = vec![
        Line::from(vec![
            Span::styled(" Air File Server ", Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::raw(" Serving: "),
            Span::styled(app_state.root_path.to_string_lossy(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  ➜ Local:   "),
            Span::styled(format!("http://localhost:{}", ui_state.port), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("  ➜ Network: "),
            Span::styled(format!("http://{}:{}", ui_state.lan_ip, ui_state.port), Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [Stats] ", Style::default().fg(Color::Cyan)),
            Span::raw("Files: "),
            Span::styled(files.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" | Volume: "),
            Span::styled(format_size(bytes), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" | Sandbox: "),
            Span::styled("ENABLED 🔒", Style::default().fg(Color::Green)),
        ]),
    ];

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title(" System Status "));
    f.render_widget(header, chunks[0]);

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
        
        logs_lock.iter()
            .skip(start)
            .take(end - start)
            .map(|log| ListItem::new(log.as_str()))
            .collect()
    } else {
        vec![]
    };
    
    let logs_list = List::new(display_logs)
        .block(Block::default().borders(Borders::ALL).title(format!(" Request Logs ({}) ", logs_count)))
        .style(Style::default().fg(Color::White));
    f.render_widget(logs_list, chunks[1]);

    // Footer
    let footer_text = format!(" Press 'q' to quit | Up/Down to scroll ({}) ", ui_state.scroll_offset);
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
