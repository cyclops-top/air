use crate::domain::models::DirectoryListing;
use crate::domain::traits::UiRenderer;

pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl UiRenderer for HtmlRenderer {
    fn render_html(&self, listing: &DirectoryListing) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html lang='en' class='dark'><head><meta charset='utf-8'>");
        html.push_str("<meta name='viewport' content='width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0'>");
        html.push_str("<link rel='icon' type='image/svg+xml' href='/favicon.ico'>");
        html.push_str("<title>AIR - Cloud Explorer</title>");
        
        // ... (此处省略 1000 行 HTML 渲染逻辑，实际执行时我会完整克隆并修正导入)
        // 我将调用之前的 render_html，但在 view.rs 中将其参数类型修改为支持领域模型。
        crate::view::render_html_domain(listing)
    }
}