use serde::Serialize;

#[derive(Serialize)]
pub struct FileEntry {
    pub name: String,
    #[serde(rename = "isDir")]
    pub is_dir: bool,
    pub size: u64,
    #[serde(rename = "modTime")]
    pub mod_time: String,
}

#[derive(Serialize)]
pub struct DirectoryListing {
    #[serde(rename = "currentPath")]
    pub current_path: String,
    pub items: Vec<FileEntry>,
    #[serde(rename = "lanIp")]
    pub lan_ip: String,
    pub port: u16,
}

pub fn render_html(listing: &DirectoryListing) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang='en'><head><meta charset='utf-8'>");
    html.push_str("<meta name='viewport' content='width=device-width, initial-scale=1.0'>");
    html.push_str("<link rel='icon' type='image/svg+xml' href='/favicon.ico'>");
    html.push_str("<title>Air - ");
    html.push_str(&listing.current_path);
    html.push_str("</title>");
    html.push_str("<style>");
    html.push_str(r#"
        :root {
            --primary: #2563eb;
            --accent-cyan: #00f7ff;
            --accent-green: #10ff70;
            --background-dark: #030305;
            --slate-100: #f1f5f9;
            --slate-200: #e2e8f0;
            --slate-300: #cbd5e1;
            --slate-400: #94a3b8;
            --slate-500: #64748b;
            --slate-600: #475569;
            --slate-700: #334155;
            --slate-800: #1e293b;
            --slate-900: #0f172a;
        }
        * { box-sizing: border-box; }
        body {
            margin: 0;
            font-family: system-ui, -apple-system, sans-serif;
            background-color: var(--background-dark);
            color: var(--slate-100);
            height: 100vh;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            background-image:
                linear-gradient(rgba(37, 99, 235, 0.02) 1px, transparent 1px),
                linear-gradient(90deg, rgba(37, 99, 235, 0.02) 1px, transparent 1px);
            background-size: 48px 48px;
        }
        .container {
            width: 100%;
            max-width: 1152px; /* max-w-6xl */
            margin: 0 auto;
            padding: 0 1.5rem;
            display: flex;
            flex-direction: column;
            height: 100%;
        }
        header {
            padding: 2rem 0;
            border-bottom: 1px solid rgba(255,255,255,0.1);
            background: rgba(3, 3, 5, 0.5);
            backdrop-filter: blur(12px);
            flex-shrink: 0;
            z-index: 10;
        }
        .header-content {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .logo-section {
            display: flex;
            align-items: center;
            gap: 3rem;
        }
        .logo-group {
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }
        .logo-box {
            width: 2rem;
            height: 2rem;
            background: var(--primary);
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 2px;
            box-shadow: 0 0 15px rgba(37, 99, 235, 0.5);
        }
        .logo-text {
            font-size: 1.875rem;
            font-weight: 700;
            letter-spacing: 0.15em;
            text-shadow: 0 0 10px rgba(255, 255, 255, 0.4);
            margin: 0;
        }
        .status-badge {
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }
        .pulse-dot {
            width: 0.5rem;
            height: 0.5rem;
            background: var(--accent-green);
            border-radius: 50%;
            box-shadow: 0 0 15px rgba(16, 255, 112, 0.6);
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(16, 255, 112, 0.6); }
            70% { transform: scale(1); box-shadow: 0 0 0 6px rgba(16, 255, 112, 0); }
            100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(16, 255, 112, 0); }
        }
        .status-text {
            font-size: 0.75rem;
            font-weight: 900;
            text-transform: uppercase;
            letter-spacing: 0.25em;
            color: var(--accent-green);
        }
        .ip-display {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            font-family: monospace;
            font-size: 0.75rem;
            color: var(--accent-cyan);
            text-shadow: 0 0 15px rgba(0, 247, 255, 0.6);
            font-weight: 700;
        }
        main { 
            flex: 1; 
            overflow: hidden; 
            display: flex; 
            flex-direction: column;
            padding: 4rem 0;
        }
        .glass-card {
            background: rgba(8, 8, 12, 0.85);
            backdrop-filter: blur(24px);
            border: 1px solid rgba(0, 247, 255, 0.25);
            border-radius: 0.75rem;
            box-shadow: 0 0 30px rgba(0, 247, 255, 0.05);
            display: flex;
            flex-direction: column;
            min-height: 0;
            flex: 1;
        }
        .table-scroll {
            overflow-y: auto;
            flex: 1;
        }
        table { width: 100%; border-collapse: collapse; font-family: monospace; }
        thead th {
            position: sticky;
            top: 0;
            background: rgba(255, 255, 255, 0.04);
            backdrop-filter: blur(10px);
            z-index: 5;
            padding: 1.75rem 2.5rem;
            text-align: left;
            font-size: 0.75rem;
            font-weight: 900;
            text-transform: uppercase;
            letter-spacing: 0.3em;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            color: white;
            white-space: nowrap;
        }
        td {
            padding: 2rem 2.5rem;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            transition: all 0.3s;
            white-space: nowrap;
        }
        tr:hover td { background: rgba(255, 255, 255, 0.05); }
        .file-link {
            display: flex;
            align-items: center;
            gap: 1.5rem;
            text-decoration: none;
            color: var(--slate-100);
            font-weight: 500;
            font-size: 1.125rem;
        }
        .file-link:hover { color: white; }
        .file-name {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
        .icon-svg { width: 1.5rem; height: 1.5rem; fill: var(--primary); transition: fill 0.3s; flex-shrink: 0; }
        tr:hover .icon-svg { fill: var(--accent-cyan); }
        .size-text { font-size: 1rem; color: var(--slate-200); }
        .time-text { font-size: 0.875rem; color: var(--slate-400); }
        .time-sep { color: var(--slate-600); padding: 0 0.25rem; }
        .download-btn {
            background: none;
            border: none;
            color: var(--slate-400);
            cursor: pointer;
            transition: all 0.3s;
        }
        .download-btn:hover {
            color: var(--accent-cyan);
            transform: scale(1.25);
        }
        .breadcrumb {
            display: flex;
            align-items: center;
            gap: 1rem;
            padding: 0.5rem 1rem;
            font-family: monospace;
            text-transform: uppercase;
            font-size: 0.875rem;
            margin-bottom: 0.5rem;
            flex-shrink: 0;
        }
        .breadcrumb a {
            text-decoration: none;
            color: var(--slate-300);
            font-weight: 700;
            transition: color 0.3s;
        }
        .breadcrumb a:hover { color: white; }
        .breadcrumb-active {
            color: var(--accent-cyan);
            text-shadow: 0 0 15px rgba(0, 247, 255, 0.6);
            font-weight: 900;
        }
        .breadcrumb-sep { color: rgba(37, 99, 235, 0.6); font-size: 1.125rem; text-shadow: 0 0 10px rgba(37, 99, 235, 0.9); }
        footer {
            padding: 2rem 1rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 0.75rem;
            letter-spacing: 0.25em;
            text-transform: uppercase;
            color: var(--slate-400);
            font-weight: 900;
            flex-shrink: 0;
        }
        .bg-glow-1 {
            position: fixed;
            top: 25%;
            left: -10rem;
            width: 50rem;
            height: 50rem;
            background: rgba(37, 99, 235, 0.1);
            border-radius: 50%;
            filter: blur(180px);
            pointer-events: none;
            z-index: -1;
        }
        .bg-glow-2 {
            position: fixed;
            bottom: 0;
            right: -10rem;
            width: 43.75rem;
            height: 43.75rem;
            background: rgba(0, 247, 255, 0.1);
            border-radius: 50%;
            filter: blur(160px);
            pointer-events: none;
            z-index: -1;
        }
    "#);
    html.push_str("</style></head><body>");
    html.push_str("<div class='bg-glow-1'></div><div class='bg-glow-2'></div>");

    // Container Start
    html.push_str("<div class='container'>");

    // Header
    html.push_str("<header><div class='header-content'><div class='logo-section'>");
    html.push_str("<div class='logo-group'><div class='logo-box'>");
    html.push_str(&get_icon_svg("air", "white"));
    html.push_str("</div><h1 class='logo-text'>AIR</h1></div>");
    html.push_str("<div class='status-badge'><div class='pulse-dot'></div><span class='status-text'>System Online</span></div>");
    html.push_str("</div><div class='ip-display'>");
    html.push_str(&get_icon_svg("sensors", "currentColor"));
    html.push_str(&format!("<span>{}</span>", listing.lan_ip));
    html.push_str("</div></div></header>");

    // Main
    html.push_str("<main>");
    
    // Breadcrumbs
    html.push_str("<nav class='breadcrumb'>");
    html.push_str(&render_breadcrumbs(&listing.current_path));
    html.push_str("</nav>");

    // Table
    html.push_str("<section class='glass-card'><div class='table-scroll'><table class='listing-table'><thead><tr>");
    html.push_str("<th>Filename</th><th style='width: 150px;'>Size</th><th style='width: 250px;'>Last Sync</th><th style='text-align: right; width: 100px;'>Access</th>");
    html.push_str("</tr></thead><tbody>");

    if listing.current_path != "/" && !listing.current_path.is_empty() {
        html.push_str("<tr><td colspan='4'><a href='../' class='file-link'>");
        html.push_str(&get_icon_svg("folder", "currentColor"));
        html.push_str("<span>..</span></a></td></tr>");
    }

    for item in &listing.items {
        let icon_name = if item.is_dir { "folder" } else { get_file_icon(&item.name) };
        let href = if item.is_dir { format!("{}/", item.name) } else { item.name.clone() };
        let size_str = if item.is_dir { "-".to_string() } else { format_size(item.size) };
        
        let (date, time) = if item.mod_time.len() >= 19 {
            (item.mod_time[..10].replace('-', "."), &item.mod_time[11..16])
        } else {
            (item.mod_time.clone(), "")
        };

        let display_name = truncate_filename(&item.name, 40);

        html.push_str("<tr><td>");
        html.push_str(&format!("<a href='{}' class='file-link' title='{}'>", href, item.name));
        html.push_str(&get_icon_svg(icon_name, "currentColor"));
        html.push_str(&format!("<span class='file-name'>{}</span></a></td>", display_name));
        html.push_str(&format!("<td><span class='size-text'>{}</span></td>", size_str));
        html.push_str(&format!("<td><span class='time-text'>{} <span class='time-sep'>//</span> {}</span></td>", date, time));
        html.push_str("<td style='text-align: right;'>");
        if !item.is_dir {
            html.push_str(&format!("<a href='{}' download class='download-btn'>", href));
            html.push_str(&get_icon_svg("download", "currentColor"));
            html.push_str("</a>");
        }
        html.push_str("</td></tr>");
    }

    html.push_str("</tbody></table></div></section>");

    // Footer
    html.push_str("<footer><div style='display: flex; align-items: center; gap: 1.5rem;'>");
    html.push_str("<span>Active Node: 0x8F2</span>");
    html.push_str("<span style='color: var(--slate-700); font-size: 1.125rem;'>//</span>");
    html.push_str("<span>Secure Protocol v4.1</span>");
    html.push_str("</div><div>Directory Terminal End</div></footer>");

    html.push_str("</main>");

    // Container End
    html.push_str("</div>");

    html.push_str("</body></html>");
    html
}

fn get_icon_svg(name: &str, color: &str) -> String {
    let path = match name {
        "air" => "M12.5 4L2 14h6.5v6L19 10h-6.5V4z",
        "sensors" => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5z",
        "folder" => "M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z",
        "description" => "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
        "analytics" => "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z",
        "database" => "M12 2C9.24 2 7 3.79 7 6v12c0 2.21 2.24 4 5 4s5-1.79 5-4V6c0-2.21-2.24-4-5-4zm0 2c1.93 0 3 1.08 3 2s-1.07 2-3 2-3-1.08-3-2 1.07-2 3-2z",
        "map" => "M20.5 3l-.16.03L15 5.1 9 3 3.36 4.9c-.21.07-.36.25-.36.48V20.5c0 .28.22.5.5.5l.16-.03L9 18.9l6 2.1 5.64-1.9c.21-.07.36-.25.36-.48V3.5c0-.28-.22-.5-.5-.5zM15 19l-6-2.11V5l6 2.11V19z",
        "download" => "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z",
        "hub" => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
        _ => "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
    };
    format!("<svg class='icon-svg' style='color: {};' viewBox='0 0 24 24'><path d='{}'/></svg>", color, path)
}

fn get_file_icon(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.ends_with(".log") || lower.ends_with(".txt") { "analytics" }
    else if lower.ends_with(".csv") || lower.ends_with(".db") { "database" }
    else if lower.ends_with(".svg") || lower.ends_with(".png") || lower.ends_with(".jpg") { "map" }
    else { "description" }
}

fn truncate_filename(name: &str, max_len: usize) -> String {
    let chars: Vec<char> = name.chars().collect();
    if chars.len() <= max_len {
        return name.to_string();
    }
    if max_len < 5 {
        return chars.iter().take(max_len).collect();
    }

    let keep = (max_len - 3) / 2;
    let first: String = chars.iter().take(keep).collect();
    let last: String = chars.iter().skip(chars.len() - keep).collect();
    format!("{}...{}", first, last)
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn render_breadcrumbs(path: &str) -> String {
    let mut html = String::new();
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    // Home / Hub icon
    html.push_str("<span class='glow-blue' style='font-weight: 900; color: var(--primary);'>AIR</span>");
    
    html.push_str("<span class='breadcrumb-sep'>&gt;</span>");
    html.push_str("<a href='/'>ROOT</a>");

    let mut accumulator = String::from("");
    let total_parts = parts.len();

    for (i, part) in parts.iter().enumerate() {
        accumulator.push('/');
        accumulator.push_str(part);

        html.push_str("<span class='breadcrumb-sep'>&gt;</span>");
        if i == total_parts - 1 {
            html.push_str(&format!("<span class='breadcrumb-active'>{}</span>", part));
        } else {
            html.push_str(&format!("<a href='{}/'>{}</a>", accumulator, part));
        }
    }

    html
}

pub fn format_range(range: &str) -> String {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return range.to_string();
    }

    let format_part = |p: &str| -> String {
        if let Ok(bytes) = p.parse::<u64>() {
            const KB: u64 = 1024;
            const MB: u64 = KB * 1024;
            const GB: u64 = MB * 1024;

            if bytes >= GB {
                format!("{:.1}G", bytes as f64 / GB as f64)
            } else if bytes >= MB {
                format!("{:.1}M", bytes as f64 / MB as f64)
            } else if bytes >= KB {
                format!("{:.1}K", bytes as f64 / KB as f64)
            } else {
                bytes.to_string()
            }
        } else {
            p.to_string()
        }
    };

    let start = format_part(parts[0]);
    let end = format_part(parts[1]);
    
    if end.is_empty() {
        format!("{}-", start)
    } else {
        format!("{}-{}", start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(
            format_duration(std::time::Duration::from_secs(3661)),
            "01:01:01"
        );
    }

    #[test]
    fn test_format_range() {
        assert_eq!(format_range("0-1023"), "0-1023");
        assert_eq!(format_range("0-1024"), "0-1.0K");
        assert_eq!(format_range("1048576-2097152"), "1.0M-2.0M");
        assert_eq!(format_range("1048576-"), "1.0M-");
        assert_eq!(format_range("abc-def"), "abc-def");
    }

    #[test]
    fn test_truncate_filename() {
        assert_eq!(truncate_filename("short.txt", 20), "short.txt");
        let long = "this_is_a_very_long_filename_that_should_be_truncated.txt";
        let truncated = truncate_filename(long, 20);
        assert!(truncated.contains("..."));
        assert!(truncated.starts_with("this_is_"));
        assert!(truncated.ends_with("ated.txt"));
    }
}
