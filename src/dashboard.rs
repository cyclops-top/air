use crate::domain::models::Stats;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::sync::Arc;
use crate::view;
use qrcode::QrCode;
use image::Luma;

pub struct DashboardState {
    pub scroll_offset: usize,
    pub lan_ip: String,
    pub port: u16,
    pub hostname: Option<String>,
    pub picker: Option<ratatui_image::picker::Picker>,
    pub image_state: Option<Box<dyn std::any::Any>>,
}

pub fn render(f: &mut Frame, stats: &Arc<Stats>, state: &mut DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Header & Stats
            Constraint::Min(0),     // Logs
        ])
        .split(f.area());

    render_header(f, stats, state, chunks[0]);
    render_logs(f, stats, state, chunks[1]);
}

fn render_header(f: &mut Frame, stats: &Arc<Stats>, state: &mut DashboardState, area: Rect) {
    // 1. 检测真实图形支持 (非 Halfblocks)
    let has_graphics = state.picker.as_ref()
        .map(|p| p.protocol_type() != ratatui_image::picker::ProtocolType::Halfblocks)
        .unwrap_or(false);

    let (qr_area, info_area) = if has_graphics {
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35), // QR Code
                Constraint::Percentage(65), // Info
            ])
            .split(area);
        (Some(horizontal[0]), horizontal[1])
    } else {
        (None, area)
    };

    // 2. 渲染 Info Block (始终存在)
    let uptime = view::format_duration(stats.start_time.elapsed());
    let files = stats.total_files.load(std::sync::atomic::Ordering::Relaxed);
    let bytes = view::format_size(stats.total_bytes.load(std::sync::atomic::Ordering::Relaxed));

    let info_text = vec![
        Line::from(vec![
            Span::styled(" AIR SERVER ", Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)),
            Span::raw(" is active"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(" ➜  Address:  ", Style::default().fg(Color::Gray)), Span::styled(format!("http://{}:{}", state.lan_ip, state.port), Style::default().add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled(" ➜  Uptime:   ", Style::default().fg(Color::Gray)), Span::raw(uptime)]),
        Line::from(vec![Span::styled(" ➜  Traffic:  ", Style::default().fg(Color::Gray)), Span::raw(format!("{} files / {}", files, bytes))]),
    ];

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title(" System Status "))
        .wrap(Wrap { trim: true });
    f.render_widget(info, info_area);

    // 3. 渲染 QR Code (仅在有图形支持时)
    if let Some(outer_area) = qr_area {
        let qr_block = Block::default().borders(Borders::ALL).title(" Access QR ");
        f.render_widget(qr_block, outer_area);

        let inner_area = outer_area.inner(Margin { vertical: 1, horizontal: 2 });

        if let Some(ref picker) = state.picker {
            if state.image_state.is_none() {
                let url = format!("http://{}:{}", state.lan_ip, state.port);
                if let Ok(code) = QrCode::new(url.as_bytes()) {
                    let image = code.render::<Luma<u8>>().build();
                    let dynamic_image = image::DynamicImage::ImageLuma8(image);
                    if let Ok(protocol) = picker.new_protocol(dynamic_image, inner_area, ratatui_image::Resize::Fit(None)) {
                        state.image_state = Some(Box::new(protocol));
                    }
                }
            }

            if let Some(ref image_any) = state.image_state {
                if let Some(protocol) = image_any.downcast_ref::<ratatui_image::protocol::Protocol>() {
                    let image_widget = ratatui_image::Image::new(protocol);
                    f.render_widget(image_widget, inner_area);
                }
            }
        }
    }
}

fn render_logs(f: &mut Frame, stats: &Arc<Stats>, state: &DashboardState, area: Rect) {
    let logs = stats.logs.lock().unwrap();
    let log_area_height = area.height.saturating_sub(2) as usize;
    
    let total_logs = logs.len();
    let display_logs: Vec<_> = logs.iter()
        .rev()
        .skip(state.scroll_offset)
        .take(log_area_height)
        .collect();

    let mut log_lines = Vec::new();
    for entry in display_logs {
        let action_style = match entry.action {
            crate::domain::models::LogAction::OpenDir => Style::default().fg(Color::Blue),
            crate::domain::models::LogAction::DownloadFile => Style::default().fg(Color::Green),
            crate::domain::models::LogAction::Favicon => Style::default().fg(Color::DarkGray),
        };

        let status = if entry.is_success {
            Span::styled(" OK ", Style::default().bg(Color::Green).fg(Color::Black))
        } else {
            Span::styled(" ERR ", Style::default().bg(Color::Red).fg(Color::Black))
        };

        let mut line_parts = vec![
            Span::styled(format!("[{}] ", entry.time), Style::default().fg(Color::DarkGray)),
            status,
            Span::raw(" "),
            Span::styled(format!("{:?}", entry.action), action_style),
            Span::raw(" "),
            Span::styled(&entry.path, Style::default().add_modifier(Modifier::BOLD)),
        ];

        if let Some(ref r) = entry.range {
            line_parts.push(Span::styled(format!(" (Range: {})", r), Style::default().fg(Color::Yellow)));
        }

        line_parts.push(Span::styled(format!(" {:.2?}", entry.duration), Style::default().fg(Color::DarkGray)));
        log_lines.push(Line::from(line_parts));
    }

    let logs_p = Paragraph::new(log_lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" Traffic Logs ({}/{}) ", total_logs.saturating_sub(state.scroll_offset), total_logs)));
    
    f.render_widget(logs_p, area);
}
