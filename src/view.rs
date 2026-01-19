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
    html.push_str("<!DOCTYPE html><html lang='en'><head><meta charset='utf-8'>");
    html.push_str("<meta name='viewport' content='width=device-width, initial-scale=1.0'>");
    html.push_str("<link rel='icon' type='image/svg+xml' href='/favicon.ico'>");
    html.push_str("<title>AIR - Browser</title>");
    
    // Theme Persistence Script
    html.push_str(r#"
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
    "#);

    html.push_str("<style>");
    html.push_str(r#"
        :root {
            --primary: #3b82f6;
            --accent-cyan: #22d3ee;
            --accent-green: #4ade80;
            --slate-50: #f8fafc;
            --slate-100: #f1f5f9;
            --slate-200: #e2e8f0;
            --slate-300: #cbd5e1;
            --slate-400: #94a3b8;
            --slate-500: #64748b;
            --slate-600: #475569;
            --slate-700: #334155;
            --slate-800: #1e293b;
            --slate-900: #0f172a;
            --mac-bg-start: #f1f5f9;
            --mac-bg-end: #e2e8f0;
        }
        .dark {
            --mac-bg-start: #1e293b;
            --mac-bg-end: #0f172a;
        }
        * { 
            box-sizing: border-box; 
            transition: background-color 0.8s ease, color 0.8s ease, border-color 0.8s ease, box-shadow 0.8s ease, backdrop-filter 0.8s ease;
        }
        body {
            margin: 0;
            font-family: system-ui, -apple-system, sans-serif;
            background-color: var(--mac-bg-start);
            background-image: radial-gradient(circle at top left, var(--mac-bg-start), var(--mac-bg-end));
            color: var(--slate-900);
            height: 100vh;
            overflow: hidden;
            display: flex;
            flex-direction: column;
        }
        .dark body { color: var(--slate-200); }
        
        .bokeh {
            position: fixed;
            width: 500px;
            height: 500px;
            border-radius: 50%;
            filter: blur(120px);
            opacity: 0.15;
            z-index: -1;
            pointer-events: none;
        }
        .bokeh-1 { top: -100px; left: -100px; background: rgba(59, 130, 246, 0.4); }
        .dark .bokeh-1 { background: rgba(59, 130, 246, 0.8); }
        .bokeh-2 { bottom: -100px; right: -100px; background: rgba(147, 51, 234, 0.4); }
        .dark .bokeh-2 { background: rgba(147, 51, 234, 0.8); }

        .container {
            width: 100%;
            max-width: 1280px;
            margin: 0 auto;
            padding: 0 2rem;
            display: flex;
            flex-direction: column;
            height: 100%;
            min-height: 0;
        }
        
        header {
            flex-shrink: 0;
            z-index: 50;
            width: 100%;
            padding: 0.75rem 0;
            background: rgba(255, 255, 255, 0.8);
            backdrop-filter: blur(20px);
            border-bottom: 1px solid rgba(0, 0, 0, 0.05);
        }
        .dark header {
            background: rgba(15, 23, 42, 0.6);
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
        }
        
        .header-wide {
            width: 100%;
            padding: 0 2rem;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        
        .logo-group {
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }
        .logo-box {
            width: 2.25rem;
            height: 2.25rem;
            background: var(--primary);
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 12px;
            box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
        }
        .logo-text {
            font-size: 1.5rem;
            font-weight: 700;
            letter-spacing: -0.02em;
            color: var(--slate-900);
            margin: 0;
        }
        .dark .logo-text { color: white; }

        .status-pill {
            display: flex;
            align-items: center;
            gap: 0.625rem;
            padding: 0.375rem 0.75rem;
            background: rgba(0, 0, 0, 0.05);
            border: 1px solid rgba(0, 0, 0, 0.05);
            border-radius: 9999px;
        }
        .dark .status-pill {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        .pulse-dot {
            width: 0.5rem;
            height: 0.5rem;
            background: var(--accent-green);
            border-radius: 50%;
            box-shadow: 0 0 10px rgba(74, 222, 128, 0.4);
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0% { transform: scale(0.95); opacity: 0.8; }
            50% { transform: scale(1.05); opacity: 1; }
            100% { transform: scale(0.95); opacity: 0.8; }
        }
        .status-text {
            font-size: 0.625rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            color: var(--accent-green);
        }

        .action-group {
            display: flex;
            align-items: center;
            gap: 1.25rem;
        }
        .glass-button {
            height: 2.5rem;
            background: rgba(0, 0, 0, 0.05);
            border: 1px solid rgba(0, 0, 0, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 12px;
            display: flex;
            align-items: center;
            gap: 0.75rem;
            padding: 0 0.75rem;
            cursor: pointer;
            color: var(--slate-700);
        }
        .dark .glass-button {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
            color: var(--slate-300);
        }
        
        .theme-toggle-track {
            width: 2.25rem;
            height: 1.125rem;
            background: rgba(0, 0, 0, 0.1);
            border-radius: 9999px;
            position: relative;
            transition: background 0.3s;
        }
        .dark .theme-toggle-track {
            background: var(--primary);
        }
        .theme-toggle-thumb {
            width: 0.875rem;
            height: 0.875rem;
            background: white;
            border-radius: 50%;
            position: absolute;
            top: 2px;
            left: 2px;
            transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            box-shadow: 0 1px 3px rgba(0,0,0,0.2);
        }
        .dark .theme-toggle-thumb {
            transform: translateX(1.125rem);
        }
        
        main { 
            flex: 1; 
            display: flex; 
            flex-direction: column;
            min-height: 0;
            padding: 0 2rem 3rem 2rem;
        }
        
        .breadcrumb-container {
            padding: 1rem 0.25rem;
            flex-shrink: 0;
        }
        .breadcrumb-pill {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.375rem 1rem;
            background: rgba(0, 0, 0, 0.03);
            border: 1px solid rgba(255, 255, 255, 0.5);
            border-radius: 9999px;
            font-size: 0.875rem;
            font-weight: 500;
            color: var(--slate-500);
        }
        .dark .breadcrumb-pill {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
            color: var(--slate-400);
        }
        .breadcrumb-pill a {
            text-decoration: none;
            color: inherit;
            transition: color 0.2s;
        }
        .breadcrumb-pill a:hover { color: var(--primary); }
        .breadcrumb-active {
            color: var(--slate-900);
            font-weight: 700;
            background: rgba(59, 130, 246, 0.1);
            padding: 0.125rem 0.5rem;
            border-radius: 6px;
        }
        .dark .breadcrumb-active { color: white; }

        .glass-container {
            background: rgba(255, 255, 255, 0.4);
            backdrop-filter: blur(40px);
            border: 1px solid rgba(0, 0, 0, 0.05);
            border-radius: 16px;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.1), inset 0 1px 1px rgba(255, 255, 255, 0.5);
            display: flex;
            flex-direction: column;
            min-height: 0;
            flex: 1;
            overflow: hidden;
            margin: 0 0.5rem 1.5rem 0.5rem;
        }
        .dark .glass-container {
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid rgba(255, 255, 255, 0.12);
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5), inset 0 1px 1px rgba(255, 255, 255, 0.1);
        }
        
        .table-scroll {
            overflow-y: auto;
            flex: 1;
        }
        .table-scroll::-webkit-scrollbar { width: 6px; }
        .table-scroll::-webkit-scrollbar-track { background: transparent; }
        .table-scroll::-webkit-scrollbar-thumb {
            background: rgba(59, 130, 246, 0.2);
            border-radius: 10px;
        }
        .table-scroll::-webkit-scrollbar-thumb:hover { background: rgba(59, 130, 246, 0.4); }

        table { width: 100%; border-collapse: collapse; table-layout: fixed; }
        
        .table-header-bg {
            background: rgba(0, 0, 0, 0.02);
            border-bottom: 1px solid rgba(0, 0, 0, 0.05);
            flex-shrink: 0;
        }
        .dark .table-header-bg {
            background: rgba(255, 255, 255, 0.04);
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }

        th {
            padding: 1rem 2rem;
            text-align: left;
            font-size: 0.6875rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            color: var(--slate-500);
        }
        .dark th { color: var(--slate-400); }
        
        td {
            padding: 1.25rem 2rem;
            transition: background 0.2s;
            border-bottom: 1px solid rgba(0, 0, 0, 0.03);
        }
        .dark td { border-bottom: 1px solid rgba(255, 255, 255, 0.03); }
        tr:hover td { background: rgba(0, 0, 0, 0.02); }
        .dark tr:hover td { background: rgba(255, 255, 255, 0.03); }

        .file-info {
            display: flex;
            align-items: center;
            gap: 1rem;
            text-decoration: none;
            color: inherit;
        }
        .icon-capsule {
            width: 2.5rem;
            height: 2.5rem;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 12px;
            flex-shrink: 0;
        }
        .file-name {
            font-size: 1.0625rem;
            font-weight: 500;
            color: var(--slate-900);
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
        .dark .file-name { color: white; }
        
        .mono-text {
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            font-size: 0.9375rem;
            color: var(--slate-600);
        }
        .dark .mono-text { color: var(--slate-400); }
        
        .download-action {
            display: flex;
            justify-content: flex-end;
        }
        
        footer {
            padding: 1rem 0.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 0.625rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            color: var(--slate-400);
            flex-shrink: 0;
        }
        .dark footer { color: var(--slate-500); }
    "#);
    html.push_str("</style></head><body>");
    html.push_str("<div class='bokeh bokeh-1'></div><div class='bokeh bokeh-2'></div>");

    // Header (Full Width)
    html.push_str("<header><div class='header-wide'><div class='logo-group'>");
    html.push_str("<div class='logo-box'>");
    html.push_str(&get_icon_svg("air", "white"));
    html.push_str("</div><h1 class='logo-text'>AIR</h1>");
    html.push_str("<div class='status-pill'><div class='pulse-dot'></div><span class='status-text'>System Online</span></div>");
    html.push_str("</div><div class='action-group'>");
    
    // IP and Node Info (Hidden on small)
    html.push_str("<div class='hidden md:flex flex-col items-end mr-4'>");
    html.push_str("<span style='font-size: 9px; opacity: 0.6; font-weight: 700; letter-spacing: 0.1em;'>NODE IDENTITY</span>");
    html.push_str(&format!("<span class='mono-text' style='font-size: 11px;'>{}</span>", listing.lan_ip));
    html.push_str("</div>");

    // Theme Toggle
    html.push_str("<div class='glass-button' onclick='toggleTheme()' title='Toggle Theme'>");
    html.push_str(&get_icon_svg("light_mode", "var(--slate-400)"));
    html.push_str("<div class='theme-toggle-track'><div class='theme-toggle-thumb'></div></div>");
    html.push_str(&get_icon_svg("dark_mode", "var(--slate-400)"));
    html.push_str("</div>");

    // // Settings placeholder
    // html.push_str("<button class='glass-button' style='width: 2.5rem; padding: 0; justify-content: center;'>");
    // html.push_str(&get_icon_svg("settings", "currentColor"));
    // html.push_str("</button>");

    html.push_str("</div></div></header>");

    // Container Start (For Main content)
    html.push_str("<div class='container'>");

    // Main
    html.push_str("<main>");
    
    // Breadcrumbs
    html.push_str("<div class='breadcrumb-container'>");
    html.push_str("<nav class='breadcrumb-pill'>");
    html.push_str(&render_breadcrumbs(&listing.current_path));
    html.push_str("</nav></div>");

    // Table
    html.push_str("<section class='glass-container'>");
    
    // Fixed Header
    html.push_str("<div class='table-header-bg'><table style='margin-bottom: 0;'><thead><tr>");
    html.push_str("<th style='width: 50%;'>Filename</th>");
    html.push_str("<th style='width: 15%;'>Size</th>");
    html.push_str("<th style='width: 20%;'>Last Sync</th>");
    html.push_str("<th style='width: 15%; text-align: right;'>Action</th>");
    html.push_str("</tr></thead></table></div>");

    // Scrollable Body
    html.push_str("<div class='table-scroll'><table><tbody class='divide-y'>");

    if listing.current_path != "/" && !listing.current_path.is_empty() {
        html.push_str("<tr><td colspan='4' style='padding: 0;'>");
        html.push_str("<a href='../' class='file-info' style='padding: 1.25rem 2rem;'>");
        html.push_str("<div class='icon-capsule' style='background: rgba(0,0,0,0.05); color: var(--slate-400);'>");
        html.push_str(&get_icon_svg("folder", "currentColor"));
        html.push_str("</div><span class='file-name'>..</span></a></td></tr>");
    }

    for item in &listing.items {
        let (icon_name, bg_color, fg_color) = if item.is_dir { 
            ("folder", "rgba(100, 116, 139, 0.1)", "var(--slate-500)")
        } else { 
            get_file_style(&item.name) 
        };
        
        let href = if item.is_dir { format!("{}/", item.name) } else { item.name.clone() };
        let size_str = if item.is_dir { "--".to_string() } else { format_size(item.size) };
        
        let (date, time) = if item.mod_time.len() >= 19 {
            (item.mod_time[..10].replace('-', "."), &item.mod_time[11..16])
        } else {
            (item.mod_time.clone(), "")
        };

        let display_name = truncate_filename(&item.name, 45);

        html.push_str("<tr>");
        // Filename
        html.push_str("<td style='width: 50%;'>");
        html.push_str(&format!("<a href='{}' class='file-info' title='{}'>", href, item.name));
        html.push_str(&format!("<div class='icon-capsule' style='background: {}; color: {};'>", bg_color, fg_color));
        html.push_str(&get_icon_svg(icon_name, "currentColor"));
        html.push_str("</div>");
        html.push_str(&format!("<span class='file-name'>{}</span></a></td>", display_name));
        
        // Size
        html.push_str("<td style='width: 15%;'><span class='mono-text'>");
        html.push_str(&size_str);
        html.push_str("</span></td>");
        
        // Time
        html.push_str("<td style='width: 20%;'><span class='mono-text' style='font-size: 0.8rem;'>");
        html.push_str(&format!("{} <span style='opacity: 0.3; padding: 0 0.25rem;'>/</span> {}", date, time));
        html.push_str("</span></td>");
        
        // Action
        html.push_str("<td style='width: 15%; text-align: right;'>");
        if !item.is_dir {
            html.push_str(&format!("<a href='{}' download class='glass-button' style='display: inline-flex; width: 2.25rem; height: 2.25rem; border-radius: 10px; color: var(--primary);'>", href));
            html.push_str(&get_icon_svg("download", "currentColor"));
            html.push_str("</a>");
        } else {
            html.push_str(&format!("<a href='{}' class='glass-button' style='display: inline-flex; width: 2.25rem; height: 2.25rem; border-radius: 10px;'>", href));
            html.push_str(&get_icon_svg("open_in_new", "currentColor"));
            html.push_str("</a>");
        }
        html.push_str("</td></tr>");
    }

    html.push_str("</tbody></table></div></section>");

    // Footer
    html.push_str("<footer><div style='display: flex; align-items: center; gap: 1rem;'>");
    html.push_str("<span>Active Node: 0x8F2</span>");
    html.push_str("<span style='opacity: 0.2;'>|</span>");
    html.push_str("<span>Secure Protocol v4.1</span>");
    html.push_str("</div><div>AIR Browser Engine &copy; 2026</div></footer>");

    html.push_str("</main></div>"); // Close container
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
        "light_mode" => "M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58c-.39-.39-1.03-.39-1.41 0s-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37c-.39-.39-1.03-.39-1.41 0s-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41l-1.06-1.06zm1.06-12.37c-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.39-.38.39-1.02 0-1.41zM7.05 18.01c-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06c.39-.38.39-1.02 0-1.41z",
        "dark_mode" => "M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9 9-4.03 9-9c0-.46-.04-.92-.1-1.36-.98 1.37-2.58 2.26-4.4 2.26-3.03 0-5.5-2.47-5.5-5.5 0-1.82.89-3.42 2.26-4.4-.44-.06-.9-.1-1.36-.1z",
        "terminal" => "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V8h16v10zm-2-1h-6v-2h6v2zM7.5 17l-1.41-1.41L8.83 13l-2.74-2.59L7.5 9l4 4-4 4z",
        "settings" => "M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z",
        "open_in_new" => "M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z",
        "video_library" => "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8 12.5v-9l6 4.5-6 4.5z",
        "verified_user" => "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z",
        "chevron_right" => "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
        _ => "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
    };
    format!("<svg style='width: 1.25rem; height: 1.25rem; fill: {};' viewBox='0 0 24 24'><path d='{}'/></svg>", color, path)
}

fn get_file_style(name: &str) -> (&'static str, &'static str, &'static str) {
    let lower = name.to_lowercase();
    if lower.ends_with(".log") || lower.ends_with(".txt") { 
        ("analytics", "rgba(16, 185, 129, 0.1)", "var(--accent-green)") 
    }
    else if lower.ends_with(".csv") || lower.ends_with(".db") || lower.ends_with(".sql") { 
        ("database", "rgba(6, 182, 212, 0.1)", "var(--accent-cyan)") 
    }
    else if lower.ends_with(".svg") || lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg") || lower.ends_with(".webp") { 
        ("map", "rgba(99, 102, 241, 0.1)", "#818cf8") 
    }
    else if lower.ends_with(".mp4") || lower.ends_with(".mkv") || lower.ends_with(".mov") || lower.ends_with(".avi") {
        ("video_library", "rgba(249, 115, 22, 0.1)", "#fb923c")
    }
    else if lower.ends_with(".key") || lower.ends_with(".cert") || lower.ends_with(".pem") || lower.ends_with(".pub") {
        ("verified_user", "rgba(244, 63, 94, 0.1)", "#fb7185")
    }
    else { 
        ("description", "rgba(59, 130, 246, 0.1)", "var(--primary)") 
    }
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

    // Home / Hub
    html.push_str("<a href='/' style='display: flex; align-items: center; gap: 0.5rem;'>");
    html.push_str("<span style='font-weight: 800; color: var(--primary);'>AIR</span>");
    html.push_str("</a>");
    
    html.push_str("<span style='opacity: 0.2;'>");
    html.push_str(&get_icon_svg("chevron_right", "currentColor"));
    html.push_str("</span>");
    
    html.push_str("<a href='/'>ROOT</a>");

    let mut accumulator = String::from("");
    let total_parts = parts.len();

    for (i, part) in parts.iter().enumerate() {
        accumulator.push('/');
        accumulator.push_str(part);

        html.push_str("<span style='opacity: 0.2;'>");
        html.push_str(&get_icon_svg("chevron_right", "currentColor"));
        html.push_str("</span>");
        
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

pub struct DiscoverUI {
    pub state: ListState,
    pub nodes: Vec<DiscoveryMsg>,
    pub node_map: HashMap<String, DiscoveryMsg>,
}

impl DiscoverUI {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            nodes: Vec::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn update_nodes(&mut self, node: DiscoveryMsg) {
        let selected_id = self.selected_node().map(|n| n.id.clone());

        if !node.is_online {
            // Remove node if it's offline. 
            if self.node_map.contains_key(&node.id) {
                self.node_map.remove(&node.id);
                self.nodes.retain(|n| n.id != node.id);
            }
        } else if !self.node_map.contains_key(&node.id) {
            self.node_map.insert(node.id.clone(), node.clone());
            self.nodes.push(node);
        } else {
            // Update existing node info if needed (e.g. IP changed)
            if let Some(existing) = self.node_map.get_mut(&node.id) {
                *existing = node.clone();
                // Find and update in the list as well
                if let Some(pos) = self.nodes.iter().position(|n| n.id == node.id) {
                    self.nodes[pos] = node;
                }
            }
        }

        // Sort nodes by name (case-insensitive)
        self.nodes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        // Restore or fix selection
        if let Some(id) = selected_id {
            if let Some(new_pos) = self.nodes.iter().position(|n| n.id == id) {
                self.state.select(Some(new_pos));
            } else if !self.nodes.is_empty() {
                // Previously selected node is gone, clamp selection
                let current = self.state.selected().unwrap_or(0);
                self.state.select(Some(current.min(self.nodes.len() - 1)));
            } else {
                self.state.select(None);
            }
        } else if !self.nodes.is_empty() && self.state.selected().is_none() {
            self.state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.nodes.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.nodes.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.nodes.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.nodes.is_empty() {
            self.state.select(Some(i));
        }
    }

    pub fn selected_node(&self) -> Option<&DiscoveryMsg> {
        self.state.selected().and_then(|i| self.nodes.get(i))
    }
}

pub fn render_discover(f: &mut Frame, ui: &mut DiscoverUI) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🔍 Air Service Discovery")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // List
    let items: Vec<ListItem> = ui.nodes
        .iter()
        .map(|node| {
            let content = format!(
                " {:<20} | {:<15} | {:<5} | {:<5}",
                node.name, node.ip, node.port, node.scheme
            );
            ListItem::new(content).style(Style::default().fg(Color::White))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Available Nodes "))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], &mut ui.state);

    // Footer
    let footer = Paragraph::new(" [↑/↓] Navigate | [Enter] Open Address | [Q] Quit ")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
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
