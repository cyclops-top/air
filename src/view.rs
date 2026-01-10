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
}

pub fn render_html(listing: &DirectoryListing) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><meta charset='utf-8'>");
    html.push_str("<link rel='icon' type='image/svg+xml' href='/favicon.ico'>");
    html.push_str("<title>Air - ");
    html.push_str(&listing.current_path);
    html.push_str("</title>");
    html.push_str("<style>");
    html.push_str(
        "body { font-family: sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }",
    );
    html.push_str("ul { list-style: none; padding: 0; }");
    html.push_str("li { padding: 8px 12px; border-bottom: 1px solid #eee; display: flex; justify-content: space-between; align-items: center; }");
    html.push_str("li:hover { background-color: #f5f5f5; }");
    html.push_str("a { text-decoration: none; color: #333; flex-grow: 1; }");
    html.push_str(".icon { margin-right: 10px; }");
    html.push_str(".time { color: #888; font-size: 0.9em; margin-right: 20px; white-space: nowrap; }");
    html.push_str(".size { color: #888; font-size: 0.9em; width: 80px; text-align: right; }");
    html.push_str(".header { font-size: 1.2em; margin-bottom: 20px; color: #555; }");
    html.push_str(".header a { color: #007bff; text-decoration: none; }");
    html.push_str(".header a:hover { text-decoration: underline; }");
    html.push_str(".separator { margin: 0 8px; color: #999; }");
    html.push_str("</style></head><body>");

    html.push_str("<div class='header'>");
    html.push_str(&render_breadcrumbs(&listing.current_path));
    html.push_str("</div>");

    html.push_str("<ul>");

    if listing.current_path != "/" && !listing.current_path.is_empty() {
        html.push_str("<li><span class='icon'>📁</span><a href='../'>..</a></li>");
    }

    for item in &listing.items {
        let icon = if item.is_dir { "📁" } else { "📄" };
        let href = if item.is_dir {
            format!("{}/", item.name)
        } else {
            item.name.clone()
        };

        let size_str = if item.is_dir {
            "-".to_string()
        } else {
            format_size(item.size)
        };

        // Format RFC 3339 to a simpler YYYY-MM-DD HH:MM:SS
        let display_time = if item.mod_time.len() >= 19 {
            item.mod_time[..19].replace('T', " ")
        } else {
            item.mod_time.clone()
        };

        html.push_str("<li><span class='icon'>");
        html.push_str(icon);
        html.push_str("</span><a href='");
        html.push_str(&href);
        html.push_str("'>");
        html.push_str(&item.name);
        html.push_str("</a><span class='time'>");
        html.push_str(&display_time);
        html.push_str("</span><span class='size'>");
        html.push_str(&size_str);
        html.push_str("</span></li>");
    }
    html.push_str("</ul>");
    html.push_str("</body></html>");
    html
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

pub fn format_range(range: &str) -> String {
    // range is usually "start-end" or "start-"
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

fn render_breadcrumbs(path: &str) -> String {
    let mut html = String::new();
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    // Always start with Home
    html.push_str("<a href='/'>Home</a>");

    let mut accumulator = String::from("");

    for part in parts {
        accumulator.push('/');
        accumulator.push_str(part);

        html.push_str("<span class='separator'>/</span>");
        html.push_str(&format!("<a href='{}/'>{}</a>", accumulator, part));
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumbs_root() {
        let html = render_breadcrumbs("/");
        assert_eq!(html, "<a href='/'>Home</a>");
    }

    #[test]
    fn test_breadcrumbs_deep() {
        let html = render_breadcrumbs("/Movies/Action");
        assert_eq!(html, "<a href='/'>Home</a><span class='separator'>/</span><a href='/Movies/'>Movies</a><span class='separator'>/</span><a href='/Movies/Action/'>Action</a>");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(
            format_duration(std::time::Duration::from_secs(0)),
            "00:00:00"
        );
        assert_eq!(
            format_duration(std::time::Duration::from_secs(59)),
            "00:00:59"
        );
        assert_eq!(
            format_duration(std::time::Duration::from_secs(60)),
            "00:01:00"
        );
        assert_eq!(
            format_duration(std::time::Duration::from_secs(3600)),
            "01:00:00"
        );
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
}
