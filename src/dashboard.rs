use image::{DynamicImage, Rgb};
use qrcode::QrCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use ratatui_image::{picker::Picker, Resize, StatefulImage, protocol::StatefulProtocol};
use std::sync::Arc;
use crate::handlers::{AppState, LogAction};
use crate::view::{format_duration, format_range, format_size};

pub struct DashboardState {
    pub scroll_offset: usize,
    pub lan_ip: String,
    pub port: u16,
    pub hostname: Option<String>,
    pub picker: Option<Picker>,
    pub image_state: Option<StatefulProtocol>,
}

pub fn render(f: &mut Frame, app_state: &Arc<AppState>, ui_state: &mut DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Shrunk header height
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

    let protocol = if app_state.enable_https { "https" } else { "http" };
    let url = format!("{}://{}:{}", protocol, ui_state.lan_ip, ui_state.port);

    // Split inner header based on picker availability (exclude Halfblocks)
    let has_graphics = ui_state.picker.as_ref()
        .map(|p| p.protocol_type() != ratatui_image::picker::ProtocolType::Halfblocks)
        .unwrap_or(false);

    let header_chunks = if has_graphics {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(24), // Increased width to accommodate padding
                Constraint::Min(0),
            ])
            .split(inner_header)
    } else {
        // No real graphics protocol, hide QR and use full width for info
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(inner_header)
    };

    // 3. Render QR Code if real graphics support is available
    if has_graphics {
        if let Some(picker) = &ui_state.picker {
            if let Ok(code) = QrCode::new(url.as_bytes()) {
                // Generate a high-resolution QR in memory
                let image_buffer = code.render::<Rgb<u8>>()
                    .module_dimensions(8, 8) // Sharp base modules
                    .quiet_zone(true)
                    .build();
                let dyn_img = DynamicImage::ImageRgb8(image_buffer);
                
                // Use Resize::Fit with Nearest filtering for maximum clarity
                if ui_state.image_state.is_none() {
                    ui_state.image_state = Some(picker.new_resize_protocol(dyn_img));
                }

                let qr_block = Block::default()
                    .borders(Borders::RIGHT)
                    .title(" Scan ")
                    .padding(ratatui::widgets::Padding::new(2, 0, 0, 0)); // Add left padding
                let inner_qr_area = qr_block.inner(header_chunks[0]);
                f.render_widget(qr_block, header_chunks[0]);

                if let Some(state) = &mut ui_state.image_state {
                    let image_widget = StatefulImage::default()
                        .resize(Resize::Fit(Some(image::imageops::FilterType::Nearest)));
                    f.render_stateful_widget(image_widget, inner_qr_area, state);
                }
            }
        }
    }


    // 4. Header Info (Now on the right or full width)
    let info_area = if has_graphics { header_chunks[1] } else { header_chunks[0] };
    
    let stats = &app_state.stats;
    let files = stats.total_files.load(std::sync::atomic::Ordering::Relaxed);
    let bytes = stats.total_bytes.load(std::sync::atomic::Ordering::Relaxed);
    
    let mut header_text = vec![
        Line::from(vec![
            Span::styled(" Air File Server ", Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::raw(" Serving: "),
            Span::styled(app_state.root_path.to_string_lossy(), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  ➜ Network: "),
            Span::styled(format!("{}://{}:{}", protocol, ui_state.lan_ip, ui_state.port), Style::default().fg(Color::Green)),
        ]),
    ];

    if let Some(ref h) = ui_state.hostname {
        header_text.push(Line::from(vec![
            Span::raw("  ➜ Host:    "),
            Span::styled(format!("{}://{}:{}", protocol, h, ui_state.port), Style::default().fg(Color::Green)),
        ]));
    }

    header_text.extend(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" [Stats] ", Style::default().fg(Color::Cyan)),
            Span::raw("Files: "),
            Span::styled(files.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" | Volume: "),
            Span::styled(format_size(bytes), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" | Uptime: "),
            Span::styled(format_duration(stats.start_time.elapsed()), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" | Sandbox: "),
            Span::styled("ENABLED 🔒", Style::default().fg(Color::Green)),
        ]),
    ]);

    if app_state.enable_https {
        header_text.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(" [Performance] ", Style::default().fg(Color::Magenta)),
            Span::styled("HTTP/2 Multiplexing: ENABLED 🚀", Style::default().fg(Color::White)),
        ]));
    }

    let header_info = Paragraph::new(header_text)
        .block(Block::default().padding(ratatui::widgets::Padding::new(2, 0, 0, 0)));
    f.render_widget(header_info, info_area);

    // Logs
    let logs_lock = stats.logs.lock().unwrap();
    let logs_count = logs_lock.len();
    
    // Calculate displayable area for logs
    let list_height = chunks[1].height as usize - 2; // -2 for borders
    let list_width = chunks[1].width as usize - 2;
    
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
            .map(|entry| {
                let action_style = match entry.action {
                    LogAction::OpenDir => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    LogAction::DownloadFile => Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                    LogAction::Favicon => Style::default().fg(Color::DarkGray),
                };
                
                let action_str = match entry.action {
                    LogAction::OpenDir => "OPEN DIR",
                    LogAction::DownloadFile => "DOWNLOAD",
                    LogAction::Favicon => "FAVICON",
                };

                let duration_str = format!("{:.2?}", entry.duration);
                
                // Calculate remaining width for path
                // [Time] (10) + [IP] (16) + [Action] (10) + [Duration] (10) + padding/spaces
                let fixed_width = 10 + 16 + 10 + 10 + 8; // approx
                let path_width = if list_width > fixed_width { list_width - fixed_width } else { 10 };
                
                let (path_display, path_style) = if entry.is_success {
                    let p = if let Some(ref r) = entry.range {
                        format!("{} [{}]", entry.path, format_range(r))
                    } else {
                        entry.path.clone()
                    };
                    (p, Style::default().fg(Color::White))
                } else {
                    (format!("✖ {}", entry.path), Style::default().fg(Color::Red))
                };

                let truncated_path = if path_display.len() > path_width {
                    format!("{}...", &path_display[..path_width.saturating_sub(3)])
                } else {
                    path_display
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!("[{}] ", entry.time), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:<15} ", entry.ip), Style::default().fg(Color::White)),
                    Span::styled(format!("{:<10} ", action_str), action_style),
                    Span::styled(format!("{:<10} ", duration_str), Style::default().fg(Color::DarkGray)),
                    Span::styled(truncated_path, path_style),
                ]))
            })
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
