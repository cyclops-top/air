use serde::Serialize;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::discovery::DiscoveryMsg;
use std::collections::HashMap;

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
    html.push_str("<!DOCTYPE html><html lang='en' class='dark'><head><meta charset='utf-8'>");
    html.push_str("<meta name='viewport' content='width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0'>");
    html.push_str("<link rel='icon' type='image/svg+xml' href='/favicon.ico'>");
    html.push_str("<title>AIR - Cloud Explorer</title>");
    
    // Theme Switch Logic
    html.push_str(r##"
        <script>
            (function() {
                const savedTheme = localStorage.getItem('theme');
                if (savedTheme === 'dark' || (!savedTheme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
                    document.documentElement.classList.add('dark');
                } else {
                    document.documentElement.classList.remove('dark');
                }
            })();
            function toggleTheme() {
                const isDark = document.documentElement.classList.toggle('dark');
                localStorage.setItem('theme', isDark ? 'dark' : 'light');
            }
        </script>
    "##);

    // CSS Block
    html.push_str("<style>");
    html.push_str(r##"
        *, ::before, ::after { box-sizing: border-box; margin: 0; padding: 0; }
        html { line-height: 1.5; -webkit-text-size-adjust: 100%; font-family: system-ui, -apple-system, sans-serif; }
        body { 
            margin: 0; min-height: 100vh; overflow: hidden; 
            background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 40%, #dbeafe 100%);
            color: #334155; transition: background 0.5s, color 0.5s;
        }
        .dark body { background: radial-gradient(circle at top left, #1e293b, #0f172a); color: #f1f5f9; }
        
        body::before, body::after { content: ''; position: fixed; border-radius: 50%; filter: blur(80px); z-index: -1; opacity: 0.4; pointer-events: none; }
        body::before { top: -10%; right: -10%; width: 300px; height: 300px; background: rgba(0, 122, 255, 0.15); }
        body::after { bottom: 5%; left: -5%; width: 250px; height: 250px; background: rgba(6, 182, 212, 0.15); }

        :root {
            --glass-bg: rgba(255, 255, 255, 0.7);
            --glass-border: rgba(255, 255, 255, 0.5);
            --text-filename: #1e293b;
            --btn-hover: rgba(0, 122, 255, 0.08);
            --icon-btn: #007AFF;
            --breadcrumb-bg: rgba(255, 255, 255, 0.4);
            --breadcrumb-active-bg: rgba(0, 122, 255, 0.05);
            --breadcrumb-sep: 0.2;
        }
        .dark {
            --glass-bg: rgba(15, 23, 42, 0.65);
            --glass-border: rgba(255, 255, 255, 0.08);
            --text-filename: #ffffff;
            --btn-hover: rgba(255, 255, 255, 0.1);
            --icon-btn: #ffffff;
            --breadcrumb-bg: rgba(30, 41, 59, 0.4);
            --breadcrumb-active-bg: rgba(59, 130, 246, 0.15);
            --breadcrumb-sep: 0.4;
        }

        .glass-header { 
            position: sticky; top: 0; z-index: 50; width: 100%; 
            backdrop-filter: blur(25px); -webkit-backdrop-filter: blur(25px);
            background: var(--glass-bg); border-bottom: 1px solid var(--glass-border);
        }
        
        .main-container { display: flex; flex-direction: column; height: 100vh; width: 100%; }
        .max-width-wrapper { width: 100%; max-width: 1280px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between; }

        .glass-explorer { 
            flex: 1; display: flex; flex-direction: column; overflow: hidden;
            backdrop-filter: blur(40px); -webkit-backdrop-filter: blur(40px);
            background: var(--glass-bg); border: 1px solid var(--glass-border);
            border-radius: 18px; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.05);
        }

        .file-row { 
            display: flex; align-items: center; justify-content: space-between; padding: 1.25rem; 
            border-bottom: 1px solid rgba(0, 0, 0, 0.04); transition: background 0.2s; width: 100%;
        }
        .dark .file-row { border-bottom: 1px solid rgba(255, 255, 255, 0.04); }
        .file-row:active { background: rgba(255, 255, 255, 0.6); }
        .dark .file-row:active { background: rgba(255, 255, 255, 0.05); }

        @media (min-width: 768px) {
            .file-row { display: grid; grid-template-columns: repeat(12, minmax(0, 1fr)); padding: 0.875rem 1.5rem; }
            .file-row:hover { background: rgba(255, 255, 255, 0.4); }
            .dark .file-row:hover { background: rgba(255, 255, 255, 0.03); }
        }

        .glass-button { 
            display: flex; align-items: center; justify-content: center; color: var(--icon-btn);
            backdrop-filter: blur(10px); background: var(--glass-bg);
            border: 1px solid var(--glass-border); border-radius: 12px; transition: all 0.2s;
        }
        .glass-button:hover { background: var(--btn-hover); transform: translateY(-1px); border-color: rgba(0, 122, 255, 0.2); }
        .glass-button:active { transform: scale(0.95); }

        .icon-box { display: flex; align-items: center; justify-content: center; flex-shrink: 0; width: 3rem; height: 3rem; }
        .rounded-full { border-radius: 50% !important; }

        .theme-thumb { 
            position: absolute; top: 2px; left: 2px; width: 1rem; height: 1rem; 
            background: white; border-radius: 50%; box-shadow: 0 2px 4px rgba(0,0,0,0.1); 
            transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }
        .dark .theme-thumb { transform: translateX(18px); }

        .breadcrumb-section { width: 100%; max-width: 1280px; margin: 0 auto; padding: 1.25rem; display: flex; }
        .breadcrumb-capsule { 
            display: inline-flex; align-items: center; gap: 0.5rem; padding: 0 1.25rem; height: 2.5rem;
            background: var(--breadcrumb-bg); border: 1px solid var(--glass-border); border-radius: 9999px;
            overflow-x: auto; white-space: nowrap; scrollbar-width: none; font-size: 0.75rem; font-weight: 600;
            box-shadow: 0 4px 15px rgba(0,0,0,0.05); transition: background 0.5s, border 0.5s;
            backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px);
        }
        .breadcrumb-capsule::-webkit-scrollbar { display: none; }

        .explorer-section { flex: 1; width: 100%; max-width: 1280px; margin: 0 auto; padding: 0 1.25rem 2.5rem 1.25rem; min-height: 0; display: flex; }

        .col-pc-main { grid-column: span 6; display: flex; align-items: center; gap: 1rem; min-width: 0; }
        .col-pc-sync { grid-column: span 3; display: none; }
        .col-pc-size { grid-column: span 2; display: none; }
        .col-pc-action { grid-column: span 1; display: flex; justify-content: flex-end; }

        @media (min-width: 768px) {
            .col-pc-sync { display: flex; align-items: center; justify-content: center; gap: 0.5rem; }
            .col-pc-size { display: flex; align-items: center; justify-content: center; }
            .sub-info-mobile { display: none !important; }
        }

        .text-primary { color: #007AFF; }
        .filename { font-size: 16px; font-weight: 600; color: var(--text-filename); transition: color 0.3s; }
        .font-mono { font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, monospace; }
        .truncate { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
        
        .dir-link { display: flex; align-items: center; gap: 1rem; flex: 1; min-width: 0; text-decoration: none; border-radius: 12px; }
        .dir-link:hover .filename { color: #007AFF; }
    "##);
    html.push_str("</style></head><body>");

    html.push_str("<div class='main-container'>");

    // Header
    html.push_str("<header class='glass-header'>");
    html.push_str("<div class='max-width-wrapper' style='padding: 0.75rem 1.25rem;'>");
    html.push_str("<div style='display:flex; align-items:center; gap:0.75rem;'>");
    html.push_str("<div style='width:2.25rem; height:2.25rem; display:flex; align-items:center; justify-content:center; background:#007AFF; border-radius:12px; box-shadow:0 8px 16px rgba(0,122,255,0.25);'>");
    html.push_str(&get_icon_svg("air", "width:20px; height:20px;", "white"));
    html.push_str("</div>");
    html.push_str("<div><h1 style='font-size:1.25rem; font-weight:800; letter-spacing:-0.03em; line-height:1;'>AIR</h1>");
    html.push_str("<div style='display:flex; align-items:center; gap:0.25rem; margin-top:2px;'>");
    html.push_str("<div style='width:6px; height:6px; background:#10b981; border-radius:50%; box-shadow:0 0 8px #10b981;'></div>");
    html.push_str("<span style='font-size:9px; font-weight:900; text-transform:uppercase; color:#059669; letter-spacing:0.05em;'>Online</span>");
    html.push_str("</div></div></div>");

    html.push_str("<div style='display:flex; align-items:center; gap:0.75rem;'>");
    html.push_str("<button onclick='toggleTheme()' class='glass-button' style='height:2.25rem; padding:0 0.75rem; border-radius:9999px; gap:0.5rem; position:relative;'>");
    html.push_str("<div class='dark:hidden flex items-center'>");
    html.push_str(&get_icon_svg("light_mode", "width:16px; height:16px;", "#f59e0b"));
    html.push_str("</div><div class='hidden dark:block flex items-center'>");
    html.push_str(&get_icon_svg("dark_mode", "width:16px; height:16px;", "#818cf8"));
    html.push_str("</div>");
    html.push_str("<div style='width:2.25rem; height:1.25rem; background:rgba(0,122,255,0.1); border-radius:9999px; position:relative;'>");
    html.push_str("<div class='theme-thumb'></div>");
    html.push_str("</div></button></div></div></header>");

    // Breadcrumbs
    html.push_str("<nav class='breadcrumb-section'>");
    html.push_str("<div class='breadcrumb-capsule'>");
    html.push_str(&render_adaptive_breadcrumbs(&listing.current_path));
    html.push_str("</div></nav>");

    // Explorer
    html.push_str("<div class='explorer-section'>");
    html.push_str("<section class='glass-explorer'>");
    html.push_str("<div style='padding:0.875rem 1.5rem; display:flex; justify-content:space-between; align-items:center; background:rgba(255,255,255,0.1); border-bottom:1px solid rgba(0,0,0,0.03);'>");
    html.push_str(&format!("<span style='font-size:10px; font-weight:800; opacity:0.4; text-transform:uppercase;'>{} Items Shared</span>", listing.items.len()));
    html.push_str("</div>");

    html.push_str("<div style='flex:1; overflow-y:auto;' class='custom-scrollbar'>");
    if listing.current_path != "/" && !listing.current_path.is_empty() {
        html.push_str("<a href='../' class='file-row' style='border-bottom:1px solid rgba(0,0,0,0.02);'>");
        html.push_str("<div class='col-pc-main'>");
        html.push_str("<div class='icon-box rounded-full' style='background:rgba(148, 163, 184, 0.05); color:#94a3b8;'>");
        html.push_str(&get_icon_svg("folder_open", "width:22px; height:22px;", "currentColor"));
        html.push_str("</div><span style='font-size:14px; font-weight:700; color:#94a3b8;'>..</span></div>");
        html.push_str("<div class='col-pc-action'></div></a>");
    }

    for item in &listing.items {
        let (icon, color_bg, hex) = get_dynamic_style_details(&item.name, item.is_dir);
        let href = if item.is_dir { format!("{}/", item.name) } else { item.name.clone() };
        let size_str = if item.is_dir { "".to_string() } else { format_size(item.size) };
        let date = if item.mod_time.len() >= 10 { item.mod_time[5..10].replace('-', ".") } else { "".to_string() };
        let time = if item.mod_time.len() >= 16 { &item.mod_time[11..16] } else { "" };

        html.push_str("<div class='file-row'>");
        
        // 1. Column Main
        html.push_str("<div class='col-pc-main'>");
        if item.is_dir { html.push_str(&format!("<a href='{href}' class='dir-link'>")); }
        else { html.push_str("<div style='display:flex; align-items:center; gap:1rem; flex:1; min-width:0;'>"); }
        html.push_str(&format!("<div class='icon-box rounded-full' style='background:{};'>", color_bg));
        html.push_str(&get_icon_svg(icon, "width:24px; height:24px;", hex));
        html.push_str("</div>");
        html.push_str("<div style='min-width:0;'>");
        html.push_str(&format!("<span class='filename truncate' style='display:block;'>{}</span>", item.name));
        html.push_str("<div class='sub-info-mobile' style='font-size:11px; font-weight:600; opacity:0.4; font-family:monospace; margin-top:2px;'>");
        if item.is_dir { html.push_str(&format!("{} {}", date, time)); }
        else { html.push_str(&format!("{} {} • {}", date, time, size_str)); }
        html.push_str("</div></div>");
        if item.is_dir { html.push_str("</a>"); } else { html.push_str("</div>"); }
        html.push_str("</div>");

        html.push_str("<div class='col-pc-sync' style='font-family:monospace; font-size:13px; font-weight:700; opacity:0.6;'>");
        html.push_str(&format!("<span>{}</span><span class='text-primary' style='opacity:0.7;'>{}</span>", date, time));
        html.push_str("</div>");

        html.push_str("<div class='col-pc-size' style='font-family:monospace; font-size:13px; font-weight:700; opacity:0.6;'>");
        html.push_str(&size_str);
        html.push_str("</div>");

        html.push_str("<div class='col-pc-action'>");
        if !item.is_dir {
            html.push_str(&format!("<a href='{href}' download class='glass-button rounded-full' style='width:2.75rem; height:2.75rem; color:#007AFF; box-shadow:0 2px 5px rgba(0,0,0,0.05);'>"));
            html.push_str(&get_icon_svg("download", "width:20px; height:20px;", "currentColor"));
            html.push_str("</a>");
        } else {
            html.push_str(&format!("<a href='{href}' class='glass-button rounded-full' style='width:2.75rem; height:2.75rem; opacity:0.4;'>"));
            html.push_str(&get_icon_svg("chevron_right", "width:18px; height:18px;", "currentColor"));
            html.push_str("</a>");
        }
        html.push_str("</div></div>");
    }

    html.push_str("<div style='height:5rem;'></div>");
    html.push_str("</div></section></div></main></div></body></html>");
    html
}

fn get_icon_svg(name: &str, style: &str, color: &str) -> String {
    let path = match name {
        "air" => "M12.5 4L2 14h6.5v6L19 10h-6.5V4z",
        "folder" | "folder_open" => "M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z",
        "description" | "article" => "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
        "database" => "M12 2C9.24 2 7 3.79 7 6v12c0 2.21 2.24 4 5 4s5-1.79 5-4V6c0-2.21-2.24-4-5-4zm0 2c1.93 0 3 1.08 3 2s-1.07 2-3 2-3-1.08-3-2 1.07-2 3-2z",
        "image" => "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
        "video_library" => "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8 12.5v-9l6 4.5-6 4.5z",
        "verified_user" => "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z",
        "download" => "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z",
        "light_mode" => "M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1z",
        "dark_mode" => "M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9 9-4.03 9-9c0-.46-.04-.92-.1-1.36-.98 1.37-2.58 2.26-4.4 2.26-3.03 0-5.5-2.47-5.5-5.5 0-1.82.89-3.42 2.26-4.4-.44-.06-.9-.1-1.36-.1z",
        "home" => "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
        "chevron_right" => "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
        _ => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z",
    };
    format!("<svg style='display:block; {}' fill='{}' viewBox='0 0 24 24'><path d='{}'/></svg>", style, color, path)
}

fn get_dynamic_style_details(name: &str, is_dir: bool) -> (&'static str, String, &'static str) {
    if is_dir { return ("folder", "rgba(59, 130, 246, 0.1)".to_string(), "#3b82f6"); }
    let l = name.to_lowercase();
    let (icon, r, g, b, hex) = if l.ends_with(".log") || l.ends_with(".txt") { ("description", 16, 185, 129, "#10b981") }
    else if l.ends_with(".csv") || l.ends_with(".db") { ("database", 6, 182, 212, "#06b6d4") }
    else if l.ends_with(".png") || l.ends_with(".jpg") || l.ends_with(".svg") { ("image", 99, 102, 241, "#6366f1") }
    else { ("article", 100, 116, 139, "#64748b") };
    
    (icon, format!("rgba({}, {}, {}, 0.1)", r, g, b), hex)
}

fn render_adaptive_breadcrumbs(path: &str) -> String {
    let mut html = String::new();
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    html.push_str("<a href='/' style='display:flex; align-items:center; gap:0.4rem; color:#64748b; font-weight:600;'>");
    html.push_str(&get_icon_svg("home", "width:14px; height:14px;", "currentColor"));
    html.push_str("AIR</a>");
    let mut acc = String::from("");
    for (i, p) in parts.iter().enumerate() {
        acc.push('/'); acc.push_str(p);
        html.push_str("<span style='opacity:var(--breadcrumb-sep); margin:0 0.4rem; display:flex; align-items:center;'>");
        html.push_str(&get_icon_svg("chevron_right", "width:12px; height:12px;", "currentColor"));
        html.push_str("</span>");
        if i == parts.len() - 1 {
            html.push_str(&format!("<span style='color:#007AFF; font-weight:800; background:var(--breadcrumb-active-bg); padding:0.25rem 0.625rem; border-radius:8px;'>{}</span>", p));
        } else {
            html.push_str(&format!("<a href='{acc}/' style='font-weight:600; color:#64748b;'>{p}</a>"));
        }
    }
    html
}

pub fn format_size(bytes: u64) -> String {
    if bytes >= 1073741824 { format!("{:.2} GB", bytes as f64 / 1073741824.0) }
    else if bytes >= 1048576 { format!("{:.2} MB", bytes as f64 / 1048576.0) }
    else if bytes >= 1024 { format!("{:.2} KB", bytes as f64 / 1024.0) }
    else { format!("{} B", bytes) }
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let s = duration.as_secs();
    format!("{:02}:{:02}:{:02}", s/3600, (s%3600)/60, s%60)
}

pub fn format_range(range: &str) -> String {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 { return range.to_string(); }
    let f = |p: &str| {
        if let Ok(b) = p.parse::<u64>() {
            if b >= 1073741824 { format!("{:.1}G", b as f64 / 1073741824.0) }
            else if b >= 1048576 { format!("{:.1}M", b as f64 / 1048576.0) }
            else if b >= 1024 { format!("{:.1}K", b as f64 / 1024.0) }
            else { b.to_string() }
        } else { p.to_string() }
    };
    format!("{}-{}", f(parts[0]), f(parts[1]))
}

pub struct DiscoverUI {
    pub state: ListState,
    pub nodes: Vec<DiscoveryMsg>,
    pub node_map: HashMap<String, DiscoveryMsg>,
}

impl DiscoverUI {
    pub fn new() -> Self { Self { state: ListState::default(), nodes: Vec::new(), node_map: HashMap::new() } }
    pub fn update_nodes(&mut self, node: DiscoveryMsg) {
        let sid = self.selected_node().map(|n| n.id.clone());
        if !node.is_online {
            if self.node_map.contains_key(&node.id) { self.node_map.remove(&node.id); self.nodes.retain(|n| n.id != node.id); }
        } else if !self.node_map.contains_key(&node.id) {
            self.node_map.insert(node.id.clone(), node.clone()); self.nodes.push(node);
        } else if let Some(e) = self.node_map.get_mut(&node.id) {
            *e = node.clone(); if let Some(p) = self.nodes.iter().position(|n| n.id == node.id) { self.nodes[p] = node; }
        }
        self.nodes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        if let Some(id) = sid {
            if let Some(np) = self.nodes.iter().position(|n| n.id == id) { self.state.select(Some(np)); }
            else if !self.nodes.is_empty() { let c = self.state.selected().unwrap_or(0); self.state.select(Some(c.min(self.nodes.len()-1))); }
            else { self.state.select(None); }
        } else if !self.nodes.is_empty() && self.state.selected().is_none() { self.state.select(Some(0)); }
    }
    pub fn next(&mut self) { if !self.nodes.is_empty() { let i = match self.state.selected() { Some(i) => if i >= self.nodes.len()-1 {0} else {i+1}, None => 0 }; self.state.select(Some(i)); } }
    pub fn previous(&mut self) { if !self.nodes.is_empty() { let i = match self.state.selected() { Some(i) => if i == 0 {self.nodes.len()-1} else {i-1}, None => 0 }; self.state.select(Some(i)); } }
    pub fn selected_node(&self) -> Option<&DiscoveryMsg> { self.state.selected().and_then(|i| self.nodes.get(i)) }
}

pub fn render_discover(f: &mut Frame, ui: &mut DiscoverUI) {
    let chunks = Layout::default().direction(Direction::Vertical).margin(1).constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)]).split(f.area());
    let title = Paragraph::new("🔍 Air Service Discovery").style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)).block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
    let items: Vec<ListItem> = ui.nodes.iter().map(|node| {
        let content = format!(" {:<20} | {:<15} | {:<5} | {:<5}", node.name, node.ip, node.port, node.scheme);
        ListItem::new(content).style(Style::default().fg(Color::White))
    }).collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Available Nodes ")).highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)).highlight_symbol(">> ");
    f.render_stateful_widget(list, chunks[1], &mut ui.state);
    let footer = Paragraph::new(" [↑/↓] Navigate | [Enter] Open Address | [Q] Quit ").style(Style::default().fg(Color::Gray)).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_format_duration() { assert_eq!(format_duration(std::time::Duration::from_secs(3661)), "01:01:01"); }
    #[test] fn test_format_range() { assert_eq!(format_range("0-1024"), "0-1.0K"); }
}
